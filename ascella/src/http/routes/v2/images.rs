use crate::http::models::images::QueryData;
use crate::prelude::*;

/// get images
///
/// View the images of a user
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/images")]
pub async fn post(query: web::Json<QueryData>, data: AccessToken) -> Result<OkResponse<Vec<SimpleImages>>, Error> {
  let images = get_images::exec(data.id(), 20, query.skip).await.map_err(|err| {
    println!("{err:?}");
    Error::DatabaseError
  })?;
  Ok(OkResponse(images))
}
