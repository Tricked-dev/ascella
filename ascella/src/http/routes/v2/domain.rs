use crate::http::models::domain::DomainData;
use crate::prelude::*;

/// set domain
///
/// set the domain of the user
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/domain")]
pub async fn post(domain_info: web::Json<DomainData>, data: AccessToken) -> Result<SendMessage, Error> {
  set_domain::exec(data.id(), domain_info.domain.clone()).await.map_err(|_| Error::BadRequest)?;
  Ok(SendMessage::new(200, true, "Successfully updated your domain."))
}
