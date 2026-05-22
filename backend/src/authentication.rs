use axum::{
  async_trait,
  extract::{self, Query, State, FromRequestParts},
  headers::authorization::{Authorization, Basic},
  headers::{Referer},
  http::{request::Parts, StatusCode, HeaderMap},
  middleware::Next,
  response::{IntoResponse, Response},
  RequestPartsExt,
  TypedHeader,
};
use axum_login::{
  memory_store::MemoryStore,
  secrecy::SecretVec,
  AuthUser,
};

use http::{header::InvalidHeaderValue, Request};
use ldap3::{LdapConnAsync, LdapConnSettings, LdapResult, ResultEntry, Scope, SearchEntry,
  result::{LdapError}};

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
      login: login,
      full_name: full_name,
    }
  }
}

impl AuthUser<String> for User {
  fn get_id(&self) -> String {
    self.login.clone()
  }
  fn get_password_hash(&self) -> SecretVec<u8> {
    SecretVec::new(self.login.clone().into())
  }
}

type AuthContext = axum_login::extractors
    ::AuthContext<String, User, MemoryStore<String, User>>;

#[derive(Clone)]
pub struct Claims(
  Option<Basic>, 
  Option<Referer>
);

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
  S: Send + Sync,
{
  type Rejection = std::convert::Infallible;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
    let basic = match parts
      .extract::<TypedHeader<Authorization<Basic>>>()
      .await
      .ok() {
      Some(TypedHeader(Authorization(basic))) => Some(basic),
      None => None
    };

    let referer = match parts
      .extract::<TypedHeader<Referer>>()
      .await
      .ok() {
      Some(TypedHeader(referer)) => Some(referer),
      None => None
    };

    Ok(Claims(basic, referer))
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

async fn login(user_store: &Arc<RwLock<HashMap<String, User>>>, mut auth: AuthContext, user: User) 
  -> response::Result<()>
{
  user_store.write().await.insert(user.login.clone(), user.clone());
  auth.login(&user).await.map_err(|e| e.to_string().into())
}

pub fn request_basic_headers() -> response::Result<HeaderMap> {
  let mut headers = HeaderMap::new();
  let header = "Basic realm=\"Foreman Reports\"".parse().map_err(|e: InvalidHeaderValue| e.to_string().into())?;
  headers.insert("WWW-Authenticate", header);
  Ok(headers)
}

pub async fn basic_auth<B>(
  State(state): State<AppState>,
  Claims(basic, referer): Claims,
  Query(url_params): Query<UrlParams>,
  auth: AuthContext,
  request: Request<B>, 
  next: Next<B>
) -> Response {
  if auth.current_user.is_none() {
    let content_type = url_params.accept.unwrap_or_default();
    let (username, password) = match basic {
      None => {
        let headers: Option<HeaderMap> = match referer {
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
    
    match login(&state.user_store, auth, user).await {
      Err(e) => return (e, content_type).into_axum_response(),
      Ok(_) => ()
    }
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
  auth: AuthContext,
  extract::Json(user_login): extract::Json<UserLogin>
) -> Response {

  let user = match ldap_auth(&state.configuration.ldap, user_login.username, user_login.password)
    .await {
    Err(e) => return (e, ContentType::Json).into_axum_response(),
    Ok(u) => u
  };
  match login(&state.user_store, auth, user).await {
    Err(e) => return (e, ContentType::Json).into_axum_response(),
    Ok(_) => ()
  }
  StringResponse(ContentType::Json, "User logged".to_string()).into_response()
}
