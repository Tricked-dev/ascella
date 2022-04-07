use crate::http::models::embed::EmbedData;
use crate::prelude::*;

/// set embed
///
/// set the embed of the user
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/embed")]
pub async fn post(embed: web::Json<EmbedData>, data: AccessToken) -> Result<OkResponse<SendMessage>, Error> {
  let embed = embed.clone();

  set_embed::exec(data.id(), embed.description, embed.title, embed.url, embed.color)
    .await
    .map_err(|_| Error::BadRequest)?;
  Ok(OkResponse(SendMessage::new(200, true, "Successfully updated your domain.")))
}
