use crate::database::s3::S3;
use crate::http::models::stats::StatsResponse;
use crate::prelude::*;

/// get image stats
///
/// View the stats & info of an image
#[api_v2_operation(tags(Images), consumes = "application/json", produces = "application/json")]
#[get("/view/{image}/stats")]
pub async fn get(image: web::Path<String>) -> Result<OkResponse<StatsResponse>, Error> {
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

      Ok(OkResponse(json))
    } else {
      Err(Error::MissingData)
    }
  } else {
    Err(Error::NotFound)
  }
}
