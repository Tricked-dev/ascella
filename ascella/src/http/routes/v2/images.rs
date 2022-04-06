// TODO: type this result

use crate::prelude::*;
#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct QueryData {
  skip: i32,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetImagesResponse(Vec<SimpleImages>);

#[api_v2_operation(
  tags(Dashboard),
  summary = "get images",
  description = "View the images of a user",
  consumes = "application/json",
  produces = "application/json"
)]
#[post("/images")]
pub async fn post(query: web::Json<QueryData>, data: AccessToken) -> Result<GetImagesResponse, Error> {
  let images = get_images::exec(data.id(), 20, query.skip).await.map_err(|err| {
    println!("{err:?}");
    Error::DatabaseError
  })?;
  Ok(GetImagesResponse(images))
}
apply_responders!(GetImagesResponse);
