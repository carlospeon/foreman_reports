use axum::{
  extract::{Query, State},
  http::{StatusCode, HeaderMap},
  middleware::Next,
  response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::{Authorization, Basic}, Referer};
use axum_login::{AuthUser, AuthnBackend, AuthSession};

use http::Request;
use ldap3::{LdapConnAsync, LdapConnSettings, LdapResult, ResultEntry, Scope, SearchEntry,
  result::LdapError};

use serde::Deserialize;
use std::{
  collections::HashMap,
  sync::Arc,
};
use tokio::sync::RwLock;

use crate::AppState;
use crate::configuration::Ldap;
use crate::response::{
  self,
  ContentType,
  Error,
  IntoAxumResponse,
  UrlParams,
  StringResponse
};


#[derive(Debug, Clone)]
pub struct User {
  pub login: String,
  pub full_name: String,
}

impl User {
  pub fn new(login: String, full_name: String) -> Self {
    Self {
      login,
      full_name,
    }
  }
}

impl AuthUser for User {
  type Id = String;

  fn id(&self) -> Self::Id {
    self.login.clone()
  }

  fn session_auth_hash(&self) -> &[u8] {
    self.login.as_bytes()
  }
}

#[derive(Clone)]
pub struct Backend {
  ldap_config: Ldap,
  user_store: Arc<RwLock<HashMap<String, User>>>,
}

impl Backend {
  pub fn new(ldap_config: Ldap, user_store: Arc<RwLock<HashMap<String, User>>>) -> Self {
    Self { ldap_config, user_store }
  }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

impl AuthnBackend for Backend {
  type User = User;
  type Credentials = Credentials;
  type Error = std::convert::Infallible;

  async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
    let user = match ldap_auth(&self.ldap_config, creds.username, creds.password).await {
      Err(_) => return Ok(None),
      Ok(u) => u
    };
    self.user_store.write().await.insert(user.login.clone(), user.clone());
    Ok(Some(user))
  }

  async fn get_user(&self, user_id: &axum_login::UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    Ok(self.user_store.read().await.get(user_id).cloned())
  }
}

pub type AuthContext = AuthSession<Backend>;

#[derive(Clone)]
pub struct Claims(
  Option<Basic>,
  Option<Referer>
);

impl Claims {
  async fn extract(parts: &mut http::request::Parts) -> Self {
    let basic = parts
      .headers
      .get(http::header::AUTHORIZATION)
      .and_then(|v| v.to_str().ok())
      .and_then(|s| {
        use axum_extra::headers::Header;
        let mut iter = std::iter::once(s.parse::<http::HeaderValue>().ok()?);
        Authorization::<Basic>::decode(&mut std::iter::once(&iter.next()?)).ok()
      })
      .map(|auth| auth.0);

    let referer = parts
      .headers
      .get(http::header::REFERER)
      .and_then(|v| v.to_str().ok())
      .and_then(|s| {
        use axum_extra::headers::Header;
        let val = s.parse::<http::HeaderValue>().ok()?;
        Referer::decode(&mut std::iter::once(&val)).ok()
      });

    Claims(basic, referer)
  }
}

async fn ldap_search(config: &Ldap, username: &str, password: &str) -> Result<(Vec<ResultEntry>, LdapResult), LdapError> {
  let (conn, mut ldap) = LdapConnAsync::with_settings(
    LdapConnSettings::new().set_no_tls_verify(true),
    &config.url).await?;
  ldap3::drive!(conn);
  ldap.simple_bind(format!("{}@{}", username, config.domain).as_str(), password).await?;
  ldap.search(
    &config.search,
    Scope::Subtree,
    format!("(&(objectClass=*)(samAccountName={})(memberOf:1.2.840.113556.1.4.1941:={}))", username, config.member_of).as_str(),
    vec!["cn"]
  ).await?.success()
}

async fn ldap_auth(config: &Ldap, username: String, password: String) -> response::Result<User> {
  let result_entry = match ldap_search(config, &username, &password).await {
    Err(error) => match error {
      LdapError::LdapResult{..} => return Err(Error::new(StatusCode::UNAUTHORIZED, "wrong credentials".to_string())),
      _ => return Err(error.to_string().into())
    },
    Ok((re, _)) => re
  };
  let entry = match result_entry.first() {
    None => return Err(Error::new(StatusCode::FORBIDDEN, "forbidden".to_string())),
    Some(e) => e.clone()
  };
  let common_name = match SearchEntry::construct(entry).attrs["cn"].first() {
    None => return Err(Error::new(StatusCode::SERVICE_UNAVAILABLE, "no cn attribute in ldap response".to_string())),
    Some(cn) => cn.to_string()
  };
  Ok(User::new(username, common_name))
}

pub fn request_basic_headers() -> response::Result<HeaderMap> {
  let mut headers = HeaderMap::new();
  let header = "Basic realm=\"Foreman Reports\"".parse().map_err(|e: http::header::InvalidHeaderValue| e.to_string().into())?;
  headers.insert("WWW-Authenticate", header);
  Ok(headers)
}

pub async fn basic_auth(
  State(state): State<AppState>,
  mut auth: AuthContext,
  Query(url_params): Query<UrlParams>,
  request: Request<axum::body::Body>,
  next: Next
) -> Response {
  if auth.user.is_none() {
    let content_type = url_params.accept.unwrap_or_default();

    let (mut parts, body) = request.into_parts();
    let claims = Claims::extract(&mut parts).await;
    let request = Request::from_parts(parts, body);

    let (username, password) = match claims.0 {
      None => {
        let headers: Option<HeaderMap> = match claims.1 {
          Some(_) => None,
          None => match request_basic_headers() {
            Err(e) => return (e, content_type).into_axum_response(),
            Ok(h) => Some(h)
          }
        };
        return (Error::new(StatusCode::UNAUTHORIZED, "Missing credentials".to_string()),
                content_type, headers
               ).into_axum_response();
      },
      Some(b) => (b.username().to_string(), b.password().to_string())
    };

    let user = match ldap_auth(
      &state.configuration.ldap,
      username,
      password
    ).await {
      Err(e) => return (e, content_type).into_axum_response(),
      Ok(u) => u
    };

    state.user_store.write().await.insert(user.login.clone(), user.clone());
    if auth.login(&user).await.is_err() {
      return (Error::new(StatusCode::SERVICE_UNAVAILABLE, "session error".to_string()),
              content_type).into_axum_response();
    }

    return next.run(request).await;
  }
  next.run(request).await
}

#[derive(Deserialize)]
pub struct UserLogin {
  username: String,
  password: String
}

pub async fn post_login(
  State(state): State<AppState>,
  mut auth: AuthContext,
  axum::extract::Json(user_login): axum::extract::Json<UserLogin>
) -> Response {

  let user = match ldap_auth(&state.configuration.ldap, user_login.username, user_login.password)
    .await {
    Err(e) => return (e, ContentType::Json).into_axum_response(),
    Ok(u) => u
  };
  state.user_store.write().await.insert(user.login.clone(), user.clone());
  if auth.login(&user).await.is_err() {
    return (Error::new(StatusCode::SERVICE_UNAVAILABLE, "session error".to_string()),
            ContentType::Json).into_axum_response();
  }
  StringResponse(ContentType::Json, "User logged".to_string()).into_response()
}
