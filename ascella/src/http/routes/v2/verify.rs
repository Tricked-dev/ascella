use crate::prelude::*;
/// verify a user
///
/// Used to check if a user is verified
#[api_v2_operation(
  tags(Dashboard)
  consumes = "application/json",
  produces = "application/json"
)]
#[post("/verify")]
pub async fn post(data: AccessToken) -> Result<Users, Error> {
  Ok(data.inner())
}
