use crate::{
    queries::{create_redirect, get_user_token},
    send_message,
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VerifyData {
    pub id: i32,
    pub key: String,
    pub vanity: String,
    pub to: String,
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) {
    let body = req.read::<VerifyData>().await;

    match body {
        Ok(body) => {
            let user_data = get_user_token::exec(body.id, body.key).await;
            // let user_data = sqlx::query!(
            //     "SELECT * FROM users WHERE id = $1 AND key = $2",
            //     body.id,
            //     body.key
            // )
            // .fetch_one(get_postgres().await)
            // .await;
            match user_data {
                Ok(r) => {
                    let aa = create_redirect::exec(r.id, body.to, body.vanity).await;
                    if aa.is_ok() {
                        res.set_status_code(StatusCode::OK);
                        res.render_json(&send_message(200, false, "Successfully updated the embed"))
                    } else {
                        if let Err(e) = aa {
                            println!("{:#?}", e);
                        }
                        res.set_status_code(StatusCode::BAD_REQUEST);
                        res.render_json(&send_message(400, false, "Vanity already exists"))
                    }
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
    }
}
