use crate::{
    queries::{get_user_token, set_domain},
    send_message,
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VerifyData {
    pub id: i32,
    pub key: String,
    pub domain: String,
}

#[fn_handler]
pub async fn post(req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
    let body = req.read::<VerifyData>().await;

    match body {
        Ok(body) => {
            let user_data = get_user_token::exec(body.id, body.key).await;

            match user_data {
                Ok(r) => {
                    set_domain::exec(r.id, body.domain).await.unwrap();
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
