use validator::Validate;

use crate::{http::models::user::ModifyUser, prelude::*};

/// verify a user
///
/// Used to check if a user is verified
#[api_v2_operation(
  tags(Dashboard)
  produces = "application/json"
)]
#[get("/user")]
pub async fn get(data: AccessToken) -> Result<OkResponse<Users>, Error> {
  Ok(OkResponse(data.inner()))
}

// #[derive(Deserialize, Apiv2Schema, Clone, TS)]
// #[ts(export)]
// pub struct ModifyUser {
//   pub name: Option<String>,
//   pub key: Option<String>,
//   pub domain: Option<String>,
//   pub autodelete: Option<i32>,
//   pub deleteall: Option<i32>,
//   pub upload_key: Option<String>,
//   pub lang: Option<String>,
// }

#[api_v2_operation(
  tags(Dashboard)
  produces = "application/json"
)]
#[post("/user")]
pub async fn post(data: AccessToken, modify: web::Json<ModifyUser>) -> Result<OkResponse<Users>, Error> {
  modify.validate()?;

  if let Some(name) = &modify.name {
    set_user_name::exec(data.id(), name.clone()).await?;
  }
  if let Some(key) = &modify.key {
    set_user_key::exec(data.id(), key.clone()).await?;
  }
  if let Some(domain) = &modify.domain {
    set_domain::exec(data.id(), domain.clone()).await?;
  }
  if let Some(autodelete) = &modify.autodelete {
    set_autodelete::exec(data.id(), *autodelete).await?;
  }
  if let Some(deleteall) = &modify.deleteall {
    set_deleteall::exec(data.id(), *deleteall).await?;
  }
  if let Some(upload_key) = &modify.upload_key {
    set_upload_key::exec(data.id(), &upload_key).await?;
  }
  if let Some(lang) = &modify.lang {
    set_language::exec(data.id(), lang.clone()).await?;
  }

  Ok(OkResponse(data.inner()))
}
