use crate::prelude::*;
#[api_v2_operation(
  summary = "verify a user",
  description = "Used to check if someone is actually a user",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/verify")]
pub async fn post(req: HttpRequest) -> Result<Users, Error> {
  if let Ok(data) = validate_request(&req).await {
    Ok(data)
  } else {
    Err(Error::NotAuthorized)
  }
}
