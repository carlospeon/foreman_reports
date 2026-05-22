use std::{backtrace::Backtrace, fmt, str::FromStr};
use std::result::Result as StdResult;
use serde::{de, Serialize, Deserialize, Deserializer};
use serde_json::json;

use axum::{
  http::{StatusCode, HeaderMap},
  response::IntoResponse,
  response::Response as AxumResponse,
  Json,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UrlParams {
  #[serde(default, deserialize_with = "empty_as_none")]
  pub accept: Option<ContentType>,
}
fn empty_as_none<'de, D, T>(de: D) -> std::result::Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr,
  T::Err: fmt::Display,
{
  let opt = Option::<String>::deserialize(de)?;
  match opt.as_deref() {
    None | Some("") => Ok(None),
    Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
  }
}

#[derive(Clone, Debug)]
pub enum Body<T> {
  String(String),
  Json(Json<T>)
}

impl Into<Body<serde_json::Value>> for (StatusCode, String, ContentType) {
  fn into(self) -> Body<serde_json::Value> {
    match self.2 {
      ContentType::Text => Body::String(format!("{:?}: {}", self.0.canonical_reason(), 
                                                            self.1)),
      ContentType::Json => Body::Json(Json(json!(
        {
          "status": self.0.canonical_reason(),
          "result": self.1 
        }
      )))
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContentType {
  Text,
  Json
}

impl Default for ContentType {
  fn default() -> Self { ContentType::Json }
}

impl FromStr for ContentType {
  type Err = &'static str;
  fn from_str(input: &str) -> StdResult<Self, Self::Err> {
      match input {
          "csv" => Ok(ContentType::Text),
          "json" => Ok(ContentType::Json),
          "text" => Ok(ContentType::Text),
          _ => Err("ContentType::from_str input not valid"),
      }
  }
}

#[derive(Debug, Clone)]
pub struct Error {
  status_code: StatusCode,
  description: String,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{:?}: {}", self.status_code.canonical_reason(), self.description)
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
      &self.description
  }
}

impl Error {
  pub fn new(status_code: StatusCode, description: String) -> Error {
    Error{ status_code: status_code,
           description: description,
    }
  }
}

impl Into<Error> for (StatusCode, String) {
  fn into(self) -> Error {
    Error::new(self.0, self.1)
  }
}

impl Into<Error> for String {
  fn into(self) -> Error {
    Error::new(StatusCode::SERVICE_UNAVAILABLE, self)
  }
}

pub trait IntoAxumResponse {
  fn into_axum_response(self) -> AxumResponse;
}

impl IntoAxumResponse for (Body<serde_json::Value>, StatusCode, Option<HeaderMap>) {
  fn into_axum_response(self) -> AxumResponse {
    match self.0 {
      Body::String(body) => match self.2 {
        Some(headers) => (self.1, headers, body).into_response(),
        None => (self.1, body).into_response(),
      }
      Body::Json(body) => match self.2 {
        Some(headers) => (self.1, headers, body).into_response(),
        None => (self.1, body).into_response(),
      }
    }
  }
}
impl IntoAxumResponse for (Body<serde_json::Value>, StatusCode) {
  fn into_axum_response(self) -> AxumResponse {
    (self.0, self.1, None).into_axum_response()
  }
}

impl IntoAxumResponse for (Error, ContentType, Option<HeaderMap>) {
  fn into_axum_response(self) -> AxumResponse {
    tracing::error!("Error: {}", self.0.to_string());
    if self.0.status_code == StatusCode::SERVICE_UNAVAILABLE {
      tracing::debug!("Backtrace: {:?}", Backtrace::force_capture());
    }

    let body: Body<serde_json::Value> = (self.0.status_code, self.0.description, self.1).into();
    (body, self.0.status_code, self.2).into_axum_response()
  }
}
impl IntoAxumResponse for (Error, ContentType) {
  fn into_axum_response(self) -> AxumResponse {
    (self.0, self.1, None).into_axum_response()
  }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct StringResponse(pub ContentType, pub String);
impl IntoResponse for StringResponse {
  fn into_response(self) -> AxumResponse {
    let body: Body<serde_json::Value> = (StatusCode::OK, self.1, self.0).into();
    (body, StatusCode::OK).into_axum_response()
  }
}

fn csv<T>(vec: Vec<T>) -> Result<String>
where 
  T: Serialize
{
  let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
  for row in vec {
    wtr.serialize(row)
      .map_err(|e| e.to_string().into())?;
  }
  let vector = wtr.into_inner()
    .map_err(|e| e.to_string().into())?;
  
  let data = String::from_utf8(vector)
    .map_err(|e| e.to_string().into())?;

  Ok(data)
}


fn body_from<T>(vector: Vec<T>, content_type: ContentType) 
  -> Result<Body<serde_json::Value>>
where
  T: Serialize
{
  let body: Body<serde_json::Value> = match content_type {
    ContentType::Text => { 
      let data = csv(vector)?;
      Body::String(data)
    },
    ContentType::Json => Body::Json(Json(json!(
      {
        "status": "ok",
        "result_length": vector.len(),
        "result": vector
      })))
  };
  return Ok(body)
}

impl<T> IntoAxumResponse for (Vec<T>, ContentType)
where T: Serialize
{
  fn into_axum_response(self) -> AxumResponse {
    let body = match body_from(self.0, self.1) {
      Err(e) => return (e, self.1).into_axum_response(),
      Ok(b) => b
    };

    let headers: Option<HeaderMap> = match self.1 {
      ContentType::Text => {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/csv".parse().unwrap());
        Some(headers)
      },
      ContentType::Json => None
    };
    (body, StatusCode::OK, headers).into_axum_response()
  }
}

