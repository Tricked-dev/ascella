use crate::{
  queries::{get_embed, get_image_vanity_only, get_user},
  send_message,
};
use byte_unit::Byte;
use http::{header::CACHE_CONTROL, HeaderMap};
use salvo::prelude::*;
use std::fs::metadata;

#[fn_handler]
pub async fn get(req: &mut Request, res: &mut Response) {
  let default = &String::from("none");
  let img: &String = req.params().get("img").unwrap_or(default);

  let data = get_image_vanity_only::exec(img.to_string()).await;

  if let Ok(image) = data {
    if let Ok(user) = get_user::exec(image.owner).await {
      if let Ok(embed) = get_embed::exec(image.owner).await {
        let data = if !image.redirect.is_some() {
          let data = metadata(format!("./images/{}/{}", image.owner, image.id)).unwrap();
          let bytes = Byte::from_bytes(data.len().into());
          let adjusted_byte = bytes.get_appropriate_unit(false);

          Some(adjusted_byte.to_string())
        } else {
          None
        };
        let json_data = serde_json::json!({
            "user_name": user.name,
            "user_id": user.id,
            "id": image.id,
            "redirect": image.redirect,
            "content_type": image.content_type,
            "embed": {
                "color": embed.color,
                "description": embed.description,
                "title": embed.title,
                "url": embed.url
            },
            "image_size": data
            // "file_size": data
        });
        res.render_json(&json_data);
        return;
      }
      let json_data = serde_json::json!({
          "user_name": user.name,
          "user_id": user.id,
          "id": image.id,
          "redirect": image.redirect,
          "content_type": image.content_type
      });
      let mut cache_headers = HeaderMap::new();
      cache_headers.append(CACHE_CONTROL, "public, max-age=604800, immutable".parse().unwrap());
      res.set_headers(cache_headers);
      res.render_json(&json_data);
      return;
    }
  }

  res.render_json(&send_message(400, false, "COULD NOT FIND IMAGE"));
}
