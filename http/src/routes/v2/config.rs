use crate::routes::prelude::*;
use actix_web::web::Query;

#[derive(Deserialize)]
pub struct Data {
    auth: String,
}

/// Entry point for our websocket route

#[get("/config")]
pub async fn get(data: Query<Data>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("content-disposition", "attachment;filename=ascella.sxcu"))
        .append_header(("content-type", "application/octet-stream"))
        .json(create_config(&data.auth)))
}
