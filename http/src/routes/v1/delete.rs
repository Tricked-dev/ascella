// use crate::get_postgres;
// use salvo::prelude::*;
// use ascella::send_message;

// #[fn_handler]
// pub async fn delete_image(req: &mut Request, res: &mut Response) {
//     let default = &String::from("none");
//     let img: &String = req.params().get("img").unwrap_or(default);
//     let password: &String = req.params().get("password").unwrap_or(default);
//     let user: &String = req.params().get("user").unwrap_or(default);

//     let user_id: i32 = req.get_header("x-user-id").unwrap_or(-1);
//     let user_token: String = req
//         .get_header("x-user-token")
//         .unwrap_or_else(|| "none".to_owned());

//     let data = sqlx::query!(
//         "SELECT * FROM users WHERE id = $1 AND key = $2",
//         user,
//         password
//     )
//     .fetch_one(get_postgres().await)
//     .await;

//     if let Ok(data) = data {}
// }
