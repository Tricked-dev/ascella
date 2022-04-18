use crate::database::s3::S3;
use crate::http::models::stats::{DisplayEmbed, StatsResponse};
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
      // Updates the view count in a non blocking way this endpoint gets called on every view
      tokio::task::spawn(async move { update_image_views::exec(image.id).await.ok() });

      let mut json = StatsResponse {
        user_name: user.name,
        user_id: user.id,
        id: image.id,
        views: image.views,
        redirect: image.redirect,
        content_type: image.content_type,
        image_size: data.clone(),
        // embed: get_embed::exec(user.id).await.ok(),
        embed: None,
      };

      let embed = get_embed::exec(user.id).await;
      if let Ok(rembed) = embed {
        let replacers = [
          ("%SIZE%", data.unwrap_or_default()),
          ("%ID%", image.id.to_string()),
          ("%TOTAL_IMAGES%", get_user_image_count::exec(user.id).await.unwrap_or_default().to_string()),
          ("%VIEWS%", image.views.to_string()),
        ];

        // Create a new embed that replaces everything from the replacers above\
        // you can get the embed with get_embed::exec and then create a new DisplayEmbed and set that embed on the json

        let mut embed = DisplayEmbed {
          title: rembed.title,
          description: rembed.description,
          color: rembed.color,
          url: rembed.url,
        };
        for (k, v) in replacers {
          embed.title = embed.title.map(|x| x.replace(k, &v));
          embed.description = embed.description.map(|x| x.replace(k, &v));
        }
        json.embed = Some(embed);
      }

      Ok(OkResponse(json))
    } else {
      Err(Error::MissingData)
    }
  } else {
    Err(Error::NotFound)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_things() {
    let replacers = [("%SIZE%", "68kb"), ("%ID%", "20"), ("%TOTAL_IMAGES%", "300"), ("%VIEWS%", "e00")];

    let mut embed = DisplayEmbed {
      title: Some("Hello %VIEWS% and i am %SIZE%".into()),
      description: Some("Hello %ID% and i am %SIZE%".into()),
      color: Some(String::new()),
      url: Some(String::new()),
    };
    for (k, v) in replacers {
      embed.title = embed.title.map(|x| x.replace(k, &v));
      embed.description = embed.description.map(|x| x.replace(k, &v));
    }
    dbg!(embed);
  }
}
