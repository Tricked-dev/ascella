use crate::http::models::redirect::RedirectData;
use crate::prelude::*;

/// Create a redirect
///
/// Used to create redirects
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/redirect")]
pub async fn post(data: web::Json<RedirectData>, user: AccessToken) -> Result<OkResponse<SendMessage>, Error> {
  let redirect = create_redirect::exec(user.id(), data.to.clone(), data.vanity.clone()).await;

  match redirect {
    Ok(_) => Ok(OkResponse(SendMessage::new(200, false, "Successfully created your redirect."))),
    _ => Err(Error::DatabaseError),
  }
}
