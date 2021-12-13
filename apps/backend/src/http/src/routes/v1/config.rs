use crate::create_config;
use salvo::{
    http::{HeaderMap, HeaderValue},
    prelude::*,
};

#[fn_handler]
pub async fn get(req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
    let default = &String::from("invalid");
    let user: &String = req.params().get("user").unwrap_or(default);
    let password: &String = req.params().get("password").unwrap_or(default);

    let mut headers = HeaderMap::new();
    headers.insert(
        "content-disposition",
        HeaderValue::from_str("attachment;filename=sogga.sics").unwrap(),
    );

    res.set_headers(headers);

    res.render_binary(
        HeaderValue::from_str("application/octet-stream").unwrap(),
        create_config(user, password).to_string().as_bytes(),
    );
    Ok(())
}
