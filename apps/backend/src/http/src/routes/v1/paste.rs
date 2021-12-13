use crate::{
    queries::{create_paste, get_paste_content},
    ran_str, send_message,
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    pub content: String,
}

#[fn_handler]
pub async fn get(req: &mut Request, res: &mut Response) {
    let paste = req.params().get("paste");
    match paste {
        Some(body) => {
            let paste = get_paste_content::exec(body.to_string()).await;
            match paste {
                Ok(paste) => res.render_json(&json!({ "content": &paste })),
                Err(_) => {
                    res.set_status_code(StatusCode::BAD_REQUEST);
                    res.render_json(&send_message(400, false, "Paste not found"))
                }
            }
        }
        None => {
            res.set_status_code(StatusCode::NOT_FOUND);
            res.render_json(&send_message(404, false, "Paste not found"))
        }
    }
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) {
    let body = req.read::<Data>().await;

    match body {
        Ok(body) => {
            if body.content.chars().count() > 50_000 {
                res.set_status_code(StatusCode::BAD_REQUEST);
                return res.render_json(&send_message(400, false, "content exceeded limit"));
            }

            let paste = create_paste::exec(ran_str(), body.content).await;
            match paste {
                Ok(paste) => res.render_json(&json!({
                    "code": &paste.id,
                    "url": format!("https://ascella.xyz/{}", &paste.id)
                })),
                Err(_) => {
                    res.set_status_code(StatusCode::BAD_REQUEST);
                    res.render_json(&send_message(400, false, "User not found"))
                }
            }
        }
        Err(_) => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render_json(&send_message(400, false, "malformed request"))
        }
    }
}
