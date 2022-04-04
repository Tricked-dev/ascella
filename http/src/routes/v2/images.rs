use crate::routes::prelude::*;

#[derive(Deserialize)]
struct QueryData {
  skip: i32,
}

#[api_v2_operation]
#[post("/images")]
pub async fn post(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
  if let Ok(data) = validate_request(&req).await {
    let result = from_str(std::str::from_utf8(&body).unwrap()); // return Result
    let query: QueryData = match result {
      Ok(v) => v,
      _ => return Err(Error::BadRequest),
    };
    let images = get_images::exec(data.id, 20, query.skip).await.map_err(|err| {
      println!("{err:?}");
      Error::DatabaseError
    })?;
    Ok(HttpResponse::Ok().json(images))
  } else {
    Err(Error::NotAuthorized)
  }
}
