// TODO: type this result

use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct QueryData {
  skip: i32,
}

#[api_v2_operation(
  description = "View the images of a user",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/images")]
pub async fn post(req: HttpRequest, query: web::Json<QueryData>) -> Result<HttpResponse, Error> {
  if let Ok(data) = validate_request(&req).await {
    let images = get_images::exec(data.id, 20, query.skip).await.map_err(|err| {
      println!("{err:?}");
      Error::DatabaseError
    })?;
    Ok(HttpResponse::Ok().json(images))
  } else {
    Err(Error::NotAuthorized)
  }
}
