use crate::{http::models::upload_style::UploadStyleData, prelude::*};

/// set domain
///
/// set the domain of the user
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/upload_style")]
pub async fn post(upload_style_info: web::Json<UploadStyleData>, data: AccessToken) -> Result<OkResponse<SendMessage>, Error> {
  set_url_style::exec(data.id(), upload_style_info.style).await.map_err(|_| Error::BadRequest)?;
  Ok(OkResponse(SendMessage::new(200, true, "Successfully updated your upload style.")))
}
