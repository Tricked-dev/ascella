use crate::prelude::*;

#[api_v2_operation(
  tags(Dashboard)
  summary = "verify a user",
  description = "Used to check if someone is actually a user",
  consumes = "application/json",
  produces = "application/json"
)]
#[post("/verify")]
pub async fn post(_req: HttpRequest, data: AccessToken) -> Result<Users, Error> {
  Ok(data.inner())
}
