use crate::{queries::get_image_vanity_only, send_message};
use http::header::CACHE_CONTROL;
use http::HeaderMap;
use salvo::{http::HeaderValue, prelude::*};
use std::fs;

#[fn_handler]
pub async fn get(req: &mut Request, res: &mut Response) {
    let default = &String::from("none");
    let img: &String = req.params().get("img").unwrap_or(default);
    let data = get_image_vanity_only::exec(img.to_string()).await;

    match data {
        Ok(image) => {
            let path = format!("./images/{}/{}", image.owner, image.id);
            let data = &fs::read(path);
            match data {
                Ok(data) => {
                    let mut cache_headers = HeaderMap::new();
                    cache_headers.append(
                        CACHE_CONTROL,
                        "public, max-age=604800, immutable".parse().unwrap(),
                    );
                    res.set_headers(cache_headers);
                    res.render_binary(HeaderValue::from_str(&image.content_type).unwrap(), data)
                }
                Err(_) => {
                    res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);

                    res.render_json(&send_message(500, false, "COULD NOT FIND IMAGE"))
                }
            }
        }
        Err(_) => {
            res.set_status_code(StatusCode::BAD_REQUEST);

            res.render_json(&send_message(400, false, "COULD NOT FIND IMAGE"))
        }
    }
}
