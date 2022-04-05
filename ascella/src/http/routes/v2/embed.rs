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
  summary = "Embed",
  description = "Set the embed of the user",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/embed")]
pub async fn post(req: HttpRequest, embed: web::Json<EmbedData>) -> Result<HttpResponse, Error> {
  if let Ok(data) = validate_request(&req).await {
    let embed = embed.clone();
    // let result = from_str(std::str::from_utf8(&body).unwrap()); // return Result
    // let embed: EmbedData = match result {
    //   Ok(v) => v,
    //   _ => return Err(Error::BadRequest),
    // };

    set_embed::exec(data.id, embed.description, embed.title, embed.url, embed.color)
      .await
      .map_err(|_| Error::BadRequest)?;
    Ok(HttpResponse::Ok().json(&send_message(200, true, "Successfully updated your domain.")))
  } else {
    Err(Error::NotAuthorized)
  }
}
