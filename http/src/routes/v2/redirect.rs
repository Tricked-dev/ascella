use crate::routes::prelude::*;

#[derive(Deserialize)]
struct RedirectData {
  pub vanity: String,
  pub to: String,
}

#[api_v2_operation]
#[post("/redirect")]
pub async fn post(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
  if let Ok(user) = validate_request(&req).await {
    let result = from_str(std::str::from_utf8(&body).unwrap());
    let data: RedirectData = match result {
      Ok(v) => v,
      _ => return Err(Error::BadRequest),
    };

    let redirect = create_redirect::exec(user.id, data.to, data.vanity).await;

    match redirect {
      Ok(_) => Ok(HttpResponse::Ok().json(&send_message(200, false, "Successfully created your redirect."))),
      _ => Err(Error::DatabaseError),
    }
  } else {
    Err(Error::NotAuthorized)
  }
}
