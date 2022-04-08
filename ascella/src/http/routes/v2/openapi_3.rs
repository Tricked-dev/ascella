use reqwest::StatusCode;
use tokio::sync::OnceCell;

use crate::prelude::*;

#[actix_web::get("/v2/ascella/spec/v3")]
pub async fn get() -> Result<HttpResponse, Error> {
  lazy_static! {
    static ref V3: OnceCell<String> = OnceCell::new();
  }
  let res = match V3.get() {
    Some(v3) => v3.to_owned(),
    None => {
      let v3 = CLIENT
        .get("https://converter.swagger.io/api/convert?url=https://ascella.wtf/v2/ascella/spec/v2")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
      V3.set(v3.clone()).unwrap();
      v3
    }
  };
  Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").body(res))
}
