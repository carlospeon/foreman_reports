use axum::{
  extract::{FromRef, FromRequestParts, Path, Query, State},
  http::{request::Parts, StatusCode},
  response::IntoResponse,
  response::Response,
};

use serde::{Serialize};
use serde::{Deserialize};
use sqlx::{FromRow};
use sqlx::postgres::{PgRow, PgConnection, PgQueryResult};
use std::borrow::Cow;

use crate::AppState;

use crate::response::{
  Error,
  IntoAxumResponse,
  Result,
  StringResponse,
  UrlParams,
};

pub mod hosts;
pub mod updated;
pub mod report;
pub mod erratas;

pub type Str<'l> = Cow<'l, str>;
pub type Sql<'l> = Str<'l>;
pub enum Param<'l> {
  I32(i32),
  Str(Str<'l>)
}
pub type SqlParams<'l> = Option<Vec<Param<'l>>>;

// pub trait SQL2<'a> {
//   fn sql() -> Sql<'a> { "".into() }
//   fn sql_with_params(_p: Vec<String>) -> Sql<'a> { "".into() }
//   fn params() -> SqlParams<'a> { None }
//   fn refresh_mvs() -> Option<Vec<&'a str>> { None }
// }

pub type Mvs<'a> = Option<Vec<MatView<'a>>>;
pub trait SQL<'a> {
  fn sql() -> Sql<'a> { "".into() }
  fn sql_with_params(_p: Vec<&String>) -> Sql<'a> { "".into() }
  fn params() -> SqlParams<'a> { None }
  fn default_filter(_p: Vec<&String>) -> Sql<'a> { "".into() }
  fn filter(_p: Vec<&String>) -> Sql<'a> { "".into() }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { None }
  fn refresh_mvs() -> Mvs<'a> { None }
}

pub struct Test;
impl<'a> SQL<'a> for Test {
  fn sql() -> Sql<'a> {
    "select 'hello world from pg'".into()
  }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MvRefresh {
    pub result: String
}
impl<'a> SQL<'a> for MvRefresh {
  fn sql() -> Sql<'a> {
    "select 'refreshing materialized views' as result".into()
  }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView { name: "full_report", ttl: Some(0)},
                                                           MatView { name: "content_view_releases", ttl: Some(0)},
                                                           MatView { name: "openscap_report", ttl: Some(0)}]) }
}

pub struct MatView<'a> {
    pub name: &'a str,
    pub ttl: Option<i32>
}

/*impl<'a> Into<MatView<'a>> for &'a str {
    fn into(self) -> MatView<'a> {
        MatView {
            name: self,
            ttl: None
        }
    }
}*/
impl<'a> From<&'a str> for MatView<'a> {
    fn from(s: &'a str) -> Self {
        MatView {
            name: s,
            ttl: None
        }
    }
}
impl<'a> Into<MatView<'a>> for (&'a str, i32) {
    fn into(self) -> MatView<'a> {
        MatView {
            name: self.0,
            ttl: Some(self.1)
        }
    }
}


type DatabaseConnectionResult = std::result::Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error>;
pub struct DatabaseConnection(DatabaseConnectionResult);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> std::result::Result<Self, Self::Rejection> {
      let state = AppState::from_ref(state);
      Ok(Self(state.pool.acquire().await))
    }
}

async fn query_scalar<T>(DatabaseConnection(dbconn): DatabaseConnection) 
  -> Result<String> 
where
  T: for<'a> SQL<'a>
{
  let mut conn = dbconn.map_err(|e| e.to_string().into())?;

  sqlx::query_scalar::<_, String>(&T::sql())
                    .fetch_one(&mut *conn)
                    .await
                    .map_err(|e| e.to_string().into())

}

pub async fn get_string<T>(
  State(_state): State<AppState>,
  database_connection: DatabaseConnection,
  url_params: Query<UrlParams>,
) -> Response
where
  T: for<'a> SQL<'a>
{

  let content_type = url_params.accept.unwrap_or_default();

  match query_scalar::<T>(database_connection).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(result) => StringResponse(content_type, result).into_response()
  }
}

async fn refresh_query(state: &AppState, conn: &mut PgConnection, mv: &MatView<'_>)
  -> std::result::Result<PgQueryResult, sqlx::Error> 
{
  let ttl = match mv.ttl {
      Some(t) => t,
      None => state.configuration.database.materialized_view_ttl
  };
  sqlx::query("call refresh_materialized_view($1, $2::interval)")
    .bind(mv.name)
    .bind(format!("{} hours", ttl))
    .execute(conn).await
}

async fn vector_from<'a, T>(
  state: AppState,
  DatabaseConnection(dbconn): DatabaseConnection,
  sql: Sql<'a>,
  params: SqlParams<'a>,
) -> Result<Vec<T>>
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + SQL<'a>
{
  let mut conn = dbconn.map_err(|e| e.to_string().into())?;

  if let Some(mvs) = T::refresh_mvs() {
    for mv in mvs {
      refresh_query(&state, &mut *conn, &mv)
        .await
        .map_err(|e| e.to_string().into())?;
    }
  }

  let mut query = sqlx::query_as::<_, T>(&sql);
  if let Some(params) = params {
    for param in params {
      query = match param {
        Param::Str(p) => query.bind(p),
        Param::I32(p) => query.bind(p),
      }
    }
  }

  let report = query.fetch_all(&mut *conn)
    .await
    .map_err(|e| e.to_string().into())?;

  Ok(report)
}

pub async fn get<T>(
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  match vector_from::<T>(state, database_connection, T::sql(), T::params()).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
}

#[derive(Deserialize)]
pub struct PathParams{
  pub key: String,
  pub value: Option<String>
}
#[derive(Deserialize)]
pub struct DaysParams{
  pub days: String
}

pub async fn get_with_path_param<T>(
  Path(path_params): Path<PathParams>,
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  
  let sql = T::sql();
  let sql_params: SqlParams<'_> = Some(vec![Param::Str(path_params.key.into())]);

  match vector_from::<T>(state, database_connection, sql, sql_params).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
 
}

pub async fn get_with_path_filter<T>(
  Path(path_params): Path<PathParams>,
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  let escaped_key = path_params.key.replace("'", "''");
  let (sql, sql_params) = match path_params.value {
    None => {
      //(T::sql_with_params(vec![ escaped_key, "".to_string() ]), None)
      tracing::info!("filter: {:?}", T::default_filter(vec![&escaped_key]) );
      (T::sql_with_params(vec![ &escaped_key, &T::default_filter(vec![&escaped_key]).into() ]), None)
    },
    Some(v) => {
      //let filter = format!("and result->>'{}' = $1", escaped_key);
      (T::sql_with_params(vec![ &escaped_key, &T::filter(vec![&escaped_key]).into() ]), Some(vec![Param::Str(v.into())]))
    }
  };
  /*let sql_params: SqlParams<'_> = match path_params.value {
    Some(v) => Some(vec![Param::Str(v.into())]),
    None => None,
  };*/

  tracing::debug!("sql: {:?}", sql);

  match vector_from::<T>(state, database_connection, sql, sql_params).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
}

/*
pub async fn get_host_facts<T>(
  Path(path_params): Path<PathParams>,
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  let escaped_key = path_params.key.replace("'", "''");
  let sql = match path_params.value {
    None => {
      T::sql_with_params(vec![ &escaped_key, &"".to_string() ])
    },
    Some(_) => {
      let w = format!("and result->>'{}' = $1", escaped_key);
      T::sql_with_params(vec![ &escaped_key, &w ])
    }
  };

  let sql_params: SqlParams<'_> = match path_params.value {
    Some(v) => Some(vec![Param::Str(v.into())]),
    None => None,
  };

  tracing::debug!("sql: {:?}", sql);

  match vector_from::<T>(state, database_connection, sql, sql_params).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
 
}*/

pub async fn get_facts_groupby<'s, T>(
  Path(path_params): Path<PathParams>,
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  let escaped_key = path_params.key.replace("'", "''");
  let sql = match path_params.value {
    None => {
      T::sql_with_params(vec![ &escaped_key, &"".to_string() ])
    },
    Some(_) => {
      let w = format!("and facts->>'{}' = $1", escaped_key);
      T::sql_with_params(vec![ &escaped_key, &w ])
    }
  };

  let sql_params: SqlParams<'_> = match path_params.value {
    Some(v) => Some(vec![Param::Str(v.into())]),
    None => None,
  };

  match vector_from::<T>(state, database_connection, sql, sql_params).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
}


pub async fn get_noncompliant_erratas<T>(
  Path(days_params): Path<DaysParams>,
  Query(url_params): Query<UrlParams>,
  State(state): State<AppState>,
  database_connection: DatabaseConnection,
) -> Response
where
  T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin + Serialize + for<'a> SQL<'a>
{
  let content_type = url_params.accept.unwrap_or_default();
  let parse_days: std::result::Result<i32, _> = days_params.days.parse();
  let days = match parse_days {
      Err(error) => return 
          (Error::new(StatusCode::BAD_REQUEST, error.to_string()), content_type).into_axum_response(),
      Ok(d) => d,
  };
  let sql = T::sql_with_params(
      vec![ 
        &format!("supported = false or older_errata::date < CURRENT_DATE - '{} days'::interval", days).to_string() 
      ]
  );
  let sql_params = None;
  //let sql_params: SqlParams<'_> = Some(vec![Param::I32(days)]);
  tracing::debug!("sql: {:?}", sql);

  match vector_from::<T>(state, database_connection, sql, sql_params).await {
    Err(error) => (error, content_type).into_axum_response(),
    Ok(v) => (v, content_type).into_axum_response()
  }
 
}
