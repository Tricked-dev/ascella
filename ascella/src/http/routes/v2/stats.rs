use actix_web::body::BoxBody;
use actix_web::Responder;

use crate::database::s3::S3;
use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone, Serialize)]
pub struct StatsResponse {
  user_name: String,
  user_id: i32,
  id: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  redirect: Option<String>,
  content_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  image_size: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  embed: Option<Embeds>,
}
apply_responders!(StatsResponse);

#[api_v2_operation(
  summary = "get image stats",
  description = "View info about a image",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[get("/view/{image}/stats")]
pub async fn get(image: web::Path<String>) -> Result<StatsResponse, Error> {
  let data = get_image_vanity_only::exec(image.to_string()).await;

  if let Ok(image) = data {
    if let Ok(user) = get_user::exec(image.owner).await {
      let data = if image.redirect.is_none() {
        let r = S3.metadata(format!("{}/{}", image.owner, image.id)).await.map_err(|_| Error::DatabaseError)?;
        let text = Byte::from_bytes(r.content_length.unwrap().try_into().unwrap()).get_appropriate_unit(false);
        Some(text.to_string())
      } else {
        None
      };

      let json = StatsResponse {
        user_name: user.name,
        user_id: user.id,
        id: image.id,
        redirect: image.redirect,
        content_type: image.content_type,
        image_size: data,
        embed: get_embed::exec(user.id).await.ok(),
      };

      Ok(json)
    } else {
      Err(Error::MissingData)
    }
  } else {
    Err(Error::NotFound)
  }
}
