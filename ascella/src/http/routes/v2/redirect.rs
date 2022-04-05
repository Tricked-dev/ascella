use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct RedirectData {
  pub vanity: String,
  pub to: String,
}
#[api_v2_operation(
  tags(Dashboard),
  description = "Create a redirect",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/redirect")]
pub async fn post(req: HttpRequest, data: web::Json<RedirectData>) -> Result<SendMessage, Error> {
  if let Ok(user) = validate_request(&req).await {
    let redirect = create_redirect::exec(user.id, data.to.clone(), data.vanity.clone()).await;

    match redirect {
      Ok(_) => Ok(SendMessage::new(200, false, "Successfully created your redirect.")),
      _ => Err(Error::DatabaseError),
    }
  } else {
    Err(Error::NotAuthorized)
  }
}
