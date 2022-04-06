use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct EmbedData {
  pub title: Option<String>,
  #[allow(dead_code)]
  pub link: Option<String>,
  pub url: Option<String>,
  pub description: Option<String>,
  pub color: Option<String>,
}

#[api_v2_operation(
  tags(Dashboard),
  summary = "set embed",
  description = "Set the embed of the user",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/embed")]
pub async fn post(_req: HttpRequest, embed: web::Json<EmbedData>, data: AccessToken) -> Result<SendMessage, Error> {
  let embed = embed.clone();

  set_embed::exec(data.id(), embed.description, embed.title, embed.url, embed.color)
    .await
    .map_err(|_| Error::BadRequest)?;
  Ok(SendMessage::new(200, true, "Successfully updated your domain."))
}
