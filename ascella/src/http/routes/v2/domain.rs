use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct DomainData {
  domain: String,
}

#[api_v2_operation(
  description = "Set the domain of the user",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/domain")]
pub async fn post(req: HttpRequest, domain_info: web::Json<DomainData>) -> Result<SendMessage, Error> {
  if let Ok(data) = validate_request(&req).await {
    set_domain::exec(data.id, domain_info.domain.clone()).await.map_err(|_| Error::BadRequest)?;
    Ok(SendMessage::new(200, true, "Successfully updated your domain."))
  } else {
    Err(Error::NotAuthorized)
  }
}
