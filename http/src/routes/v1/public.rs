use crate::{
  queries::{get_user_token, make_public},
  send_message,
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VerifyData {
  pub id: i32,
  pub key: String,
  pub image_id: i32,
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) {
  let body = req.read::<VerifyData>().await;

  match body {
    Ok(body) => {
      let user_data = get_user_token::exec(body.id, body.key).await;

      if user_data.is_ok() {
        let img = make_public::exec(body.id, body.image_id).await;

        if let Ok(_img) = img {
          res.render_json(&send_message(200, true, "Image is now public"))
        } else {
          res.set_status_code(StatusCode::BAD_REQUEST);
          res.render_json(&send_message(400, false, "Image not found"))
        }
      }
    }
    Err(_) => {
      res.set_status_code(StatusCode::BAD_REQUEST);
      res.render_json(&send_message(400, false, "malformed request"))
    }
  }
}

// #[fn_handler]
// pub async fn get(_req: &mut Request, res: &mut Response) {
//     let data =
//         sqlx::query!("SELECT vanity FROM images WHERE public  = true ORDER BY random() LIMIT 20")
//             .fetch_all(get_postgres().await)
//             .await
//             .unwrap();

//     let vanitys = data.iter().map(|image| &image.vanity).collect::<Vec<_>>();
//     res.render_json(&vanitys)
// }
