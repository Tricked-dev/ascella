use crate::prelude::*;

/// verify a user
///
/// Used to check if a user is verified
#[api_v2_operation(
  tags(Dashboard)
  produces = "application/json"
)]
#[post("/verify")]
pub async fn post(data: AccessToken) -> Result<OkResponse<Users>, Error> {
  Ok(OkResponse(data.inner()))
}
