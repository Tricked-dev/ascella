use crate::{
    queries::{get_user_token, set_embed},
    send_message,
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VerifyData {
    pub id: i32,
    pub key: String,
    pub title: Option<String>,
    pub link: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
    let body = req.read::<VerifyData>().await;

    match body {
        Ok(body) => {
            let user_data = get_user_token::exec(body.id, body.key).await;
            match user_data {
                Ok(r) => {
                    set_embed::exec(r.id, body.description, body.title, body.url, body.color)
                        .await
                        .unwrap();
                    res.set_status_code(StatusCode::OK);
                    res.render_json(&send_message(200, false, "Successfully updated the embed"))
                }
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
    };
    Ok(())
}
