use crate::routes::prelude::*;

#[derive(Deserialize)]
struct EmbedData {
  pub title: Option<String>,
  #[allow(dead_code)]
  pub link: Option<String>,
  pub url: Option<String>,
  pub description: Option<String>,
  pub color: Option<String>,
}

#[post("/embed")]
pub async fn post(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
  if let Ok(data) = validate_request(&req).await {
    let result = from_str(std::str::from_utf8(&body).unwrap()); // return Result
    let embed: EmbedData = match result {
      Ok(v) => v,
      _ => return Err(Error::BadRequest),
    };

    set_embed::exec(data.id, embed.description, embed.title, embed.url, embed.color)
      .await
      .map_err(|_| Error::BadRequest)?;
    Ok(HttpResponse::Ok().json(&send_message(200, true, "Successfully updated your domain.")))
  } else {
    Err(Error::NotAuthorized)
  }
}
