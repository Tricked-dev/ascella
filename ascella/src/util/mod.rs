use crate::database::{
  queries::{get_user_auth, get_user_token},
  structs::Users,
};
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use anyhow::{anyhow, Result};
use http::{HeaderValue, StatusCode};
use lazy_static::lazy_static;
use paperclip::v2::schema::Apiv2Errors;
use rand::{distributions::Alphanumeric, prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Display;
lazy_static! {
  pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Serialize, Debug)]
#[serde(tag = "error")]
pub enum Error {
  FileTooLarge { max_size: usize },
  FileTypeNotAllowed,
  FailedToReceive,
  NotAuthorized,
  BlockingError,
  DatabaseError,
  MissingData,
  UnknownTag,
  BadRequest,
  ProbeError,
  RateLimit { message: String },
  NotFound,
  IOError,
  LabelMe,
  Four04 { message: String },
}

impl Apiv2Errors for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, failed)", self.status_code())
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }

  fn description(&self) -> &str {
    "description() is deprecated; use Display"
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    self.source()
  }
}

impl Error {
  const fn message(&self) -> &str {
    match &self {
      Error::FileTooLarge { .. } => "File too large",
      Error::FileTypeNotAllowed => "Bad request",
      Error::NotAuthorized => "Not authorized",
      Error::FailedToReceive => "Failed to receive",
      Error::DatabaseError => "Database Error",
      Error::MissingData => "Missing data",
      Error::ProbeError => "Probe error",
      Error::NotFound => "Not found",
      Error::BlockingError => "Internal server error",
      Error::IOError => "",
      Error::BadRequest => "Bad request",
      Error::RateLimit { .. } => "Rate limit exceeded",
      Error::Four04 { .. } => "404",
      _ => "",
    }
  }
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match &self {
      Error::FileTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
      Error::FileTypeNotAllowed => StatusCode::BAD_REQUEST,
      Error::NotAuthorized => StatusCode::FORBIDDEN,
      Error::FailedToReceive => StatusCode::BAD_REQUEST,
      Error::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
      Error::MissingData => StatusCode::BAD_REQUEST,
      Error::UnknownTag => StatusCode::BAD_REQUEST,
      Error::ProbeError => StatusCode::INTERNAL_SERVER_ERROR,
      Error::NotFound => StatusCode::NOT_FOUND,
      Error::BlockingError => StatusCode::INTERNAL_SERVER_ERROR,
      Error::IOError => StatusCode::INTERNAL_SERVER_ERROR,
      Error::LabelMe => StatusCode::INTERNAL_SERVER_ERROR,
      Error::BadRequest => StatusCode::BAD_REQUEST,
      Error::RateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
      Error::Four04 { .. } => StatusCode::NOT_FOUND,
    }
  }

  fn error_response(&self) -> HttpResponse {
    // let body = serde_json::to_string(&self).unwrap();
    let body = serde_json::to_string(&json!({
        "code": &self.status_code().as_u16(),
        "error": &self.message(),
        "success": false
    }))
    .unwrap();
    HttpResponse::build(self.status_code()).content_type("application/json").body(body)
  }
}

const EMOJIS: [char; 92] = [
  '✌', '😂', '😝', '😁', '😱', '👉', '🙌', '🍻', '🔥', '🌈', '☀', '🎈', '🌹', '💄', '🎀', '⚽', '🎾', '🏁', '😡', '👿', '🐻', '🐶', '🐬', '🐟', '🍀', '👀',
  '🚗', '🍎', '💝', '💙', '👌', '❤', '😍', '😉', '😓', '😳', '💪', '💩', '🍸', '🔑', '💖', '🌟', '🎉', '🌺', '🎶', '👠', '🏈', '⚾', '🏆', '👽', '💀', '🐵',
  '🐮', '🐩', '🐎', '💣', '👃', '👂', '🍓', '💘', '💜', '👊', '💋', '😘', '😜', '😵', '🙏', '👋', '🚽', '💃', '💎', '🚀', '🌙', '🎁', '⛄', '🌊', '⛵', '🏀',
  '🎱', '💰', '👶', '👸', '🐰', '🐷', '🐍', '🐫', '🔫', '👄', '🚲', '🍉', '💛', '💚',
];

fn get_header(val: Option<&HeaderValue>) -> Result<&str> {
  if let Some(val) = val {
    if let Ok(val) = val.to_str() {
      return Ok(val);
    }
  }
  Err(anyhow!(""))
}
pub async fn validate_request_upload(req: &HttpRequest) -> Result<(Users, Result<&str>), Error> {
  let headers = req.headers();

  let auth = get_header(headers.get("authorization"));
  if let Ok(auth) = auth {
    if let Ok(user) = get_user_auth::exec(auth.to_owned()).await {
      log::info!("{} {}", &user.name, &user.id);
      actix_web::rt::spawn(send_text_webhook(format!("**{}** {} {}", &req.path(), &user.name, &user.id)));
      Ok((user, get_header(headers.get("x-image-effects"))))
    } else {
      Err(Error::NotAuthorized)
    }
  } else {
    let user_id = get_header(headers.get("x-user-id")).unwrap_or("-1").parse::<i32>().unwrap_or(-1);
    let user_token = get_header(headers.get("x-user-token")).unwrap_or_else(|_| get_header(headers.get("x-user-key")).unwrap_or(""));

    if user_id == -1 || user_token.is_empty() {
      Err(Error::NotAuthorized)
    } else if let Ok(user) = get_user_token::exec(user_id, user_token.to_owned()).await {
      log::info!("{} {}", &user.name, &user.id);
      actix_web::rt::spawn(send_text_webhook(format!("**{}** {} {}", &req.path(), &user.name, &user.id)));
      Ok((user, get_header(headers.get("x-image-effects"))))
    } else {
      Err(Error::NotAuthorized)
    }
  }
}
pub async fn validate_request(req: &HttpRequest) -> Result<Users, Error> {
  let headers = req.headers();

  let auth = get_header(headers.get("authorization"));
  if let Ok(auth) = auth {
    if let Ok(user) = get_user_auth::exec(auth.to_owned()).await {
      log::info!("{} {}", &user.name, &user.id);
      actix_web::rt::spawn(send_text_webhook(format!("**{}** {} {}", &req.path(), &user.name, &user.id)));
      Ok(user)
    } else {
      Err(Error::NotAuthorized)
    }
  } else {
    let user_id = get_header(headers.get("x-user-id")).unwrap_or("-1").parse::<i32>().unwrap_or(-1);
    let user_token = get_header(headers.get("x-user-token")).unwrap_or_else(|_| get_header(headers.get("x-user-key")).unwrap_or(""));

    if user_id == -1 || user_token.is_empty() {
      Err(Error::NotAuthorized)
    } else if let Ok(user) = get_user_token::exec(user_id, user_token.to_owned()).await {
      log::info!("{} {}", &user.name, &user.id);
      actix_web::rt::spawn(send_text_webhook(format!("**{}** {} {}", &req.path(), &user.name, &user.id)));
      Ok(user)
    } else {
      Err(Error::NotAuthorized)
    }
  }
}

fn random_char() -> &'static char {
  EMOJIS.choose(&mut rand::thread_rng()).unwrap()
}

pub fn random_emojis() -> String {
  let mut result = "".to_owned();
  for _i in 1..10 {
    result.push_str(&random_char().to_string());
  }
  result
}

#[cfg(test)]
pub mod test {
  use super::*;
  #[test]
  fn test_emojis() {
    let emojis = random_emojis();
    println!("{}", emojis)
  }
}

pub fn create_config<T: std::fmt::Display>(auth: T) -> serde_json::Value {
  serde_json::json!({
    "Version": "13.1.0",
    "Name": "Ascella.host - images",
    "DestinationType": "ImageUploader",
    "RequestMethod": "POST",

    "RequestURL": "https://ascella.wtf/v2/ascella/upload",
    "Body": "MultipartFormData",
    "Headers": {
      "authorization": auth.to_string(),
    },
    "FileFormName": "image",
    "URL": "$json:url$",
  })
}

pub fn send_message(code: i32, success: bool, message: &str) -> serde_json::Value {
  serde_json::json!({
    "code": code,
    "success": success,
    "message": message
  })
}
#[derive(Serialize, Deserialize)]
pub struct UploadSuccess {
  code: i32,
  success: bool,
  url: String,
  raw: String,
}

pub fn upload_success(vanity: &str, domain: &str) -> UploadSuccess {
  UploadSuccess {
    code: 200,
    success: true,
    url: format!("{}/{}", domain, vanity),
    raw: format!("https://ascella.wtf/images/raw/{}", vanity),
  }
}

pub fn ran_str() -> String {
  rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect()
}

pub async fn send_text_webhook<T: Display>(data: T) -> Result<()> {
  let json = serde_json::json!({ "content": data.to_string() });
  log_shit(json).await?;
  Ok(())
}

pub async fn log_shit(data: Value) -> Result<()> {
  // Set up and send the post request
  CLIENT
    .post(dotenv::var("WEBHOOK").unwrap())
    .header("Content-Type", "application/json")
    .header("User-Agent", "Ascella.host/v2 Ascella is a fast image uploader")
    .body(data.to_string())
    .send()
    .await?;

  Ok(())
}
