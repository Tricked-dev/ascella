use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct DomainData {
  domain: String,
}

#[api_v2_operation(
  tags(Dashboard),
  summary = "set domain",
  description = "Set the domain of the user",
  consumes = "application/json",
  produces = "application/json"
)]
#[post("/domain")]
pub async fn post(domain_info: web::Json<DomainData>, data: AccessToken) -> Result<SendMessage, Error> {
  set_domain::exec(data.id(), domain_info.domain.clone()).await.map_err(|_| Error::BadRequest)?;
  Ok(SendMessage::new(200, true, "Successfully updated your domain."))
}
