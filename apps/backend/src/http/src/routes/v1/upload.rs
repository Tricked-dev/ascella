use crate::{
    queries::{create_image, get_user_token},
    ran_str, random_emojis, send_message, send_text_webhook, upload_success,
};
use image::io::Reader as ImageReader;
use lazy_static::lazy_static;
use salvo::{http::HeaderValue, prelude::*};
use std::fs::{create_dir_all, write};
use std::io::Cursor;
use std::path::Path;

lazy_static! {
    static ref ALLOWED_FORMATS: Vec<&'static str> = vec!["image/png", "image/jpeg", "image/gif"];
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) {
    let user_id: i32 = req.get_header("x-user-id").unwrap_or(-1);
    let user_token: String = req
        .get_header("x-user-token")
        .unwrap_or_else(|| "none".to_owned());

    if user_id == -1 || user_token == "none" {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render_json(&send_message(400, false, "Invalid credentials"));
    } else {
        let data = get_user_token::exec(user_id, user_token).await;
        if let Err(ref data) = data {
            println!("{:#?}", data)
        }
        if let Ok(data) = data {
            let file = req.get_file("image").await;
            if let Some(file) = file {
                let default_content_type = &HeaderValue::from_str("image/png").unwrap();
                let content_type = file
                    .headers()
                    .get("content-type")
                    .unwrap_or(default_content_type)
                    .to_str()
                    .unwrap();
                println!("{}", content_type);
                if !ALLOWED_FORMATS.contains(&content_type) {
                    res.set_status_code(StatusCode::BAD_REQUEST);
                    return res.render_json(&send_message(400, false, "Disallowed file format"));
                }
                // println!("{:#?}", file.headers);

                let img = create_image::exec(
                    user_id,
                    content_type.to_string(),
                    if data.emojis.unwrap_or(false) {
                        random_emojis()
                    } else {
                        ran_str()
                    },
                )
                .await
                .unwrap();

                create_dir_all(format!("images/{}", user_id)).unwrap();
                let dest = format!("images/{}/{}", user_id, img.id,);

                let mut file = std::fs::read(&file.path()).unwrap();
                if content_type == "image/jpeg" {
                    let mut bytes: Vec<u8> = Vec::new();

                    ImageReader::new(Cursor::new(file))
                        .with_guessed_format()
                        .unwrap()
                        .decode()
                        .unwrap()
                        .write_to(&mut bytes, image::ImageOutputFormat::Jpeg(90))
                        .unwrap();

                    file = bytes;
                };

                if write(Path::new(&dest), file).is_err() {
                    res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render_json(&send_message(500, false, "Could not save file"));
                } else {
                    res.render_json(&upload_success(&img.vanity, &data.domain));
                    //Sorry its needed to monitor potentially illegal images, dw we wont use the images for anything
                    send_text_webhook(format!(
                        "**IMAGE UPLOAD**: {}({}) FILE https://ascella.wtf/images/raw/{}",
                        data.id, data.name, img.vanity
                    ))
                    .await
                    .unwrap();
                }
            } else {
                res.set_status_code(StatusCode::BAD_REQUEST);
                res.render_json(&send_message(400, false, "File not found"));
            }
        } else {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render_json(&send_message(400, false, "Invalid request"));
        }
    }
}
