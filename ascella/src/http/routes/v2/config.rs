use crate::http::models::config::Data;
use crate::prelude::*;

/// get config
///
/// Returns the upload config of the given auth token
#[api_v2_operation(tags(Etc), consumes = "application/json", produces = "application/json")]
#[get("/config")]
pub async fn get(data: web::Query<Data>) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .append_header(("content-disposition", "attachment;filename=ascella.sxcu"))
      .append_header(("content-type", "application/octet-stream"))
      .json(create_config(&data.auth)),
  )
}
