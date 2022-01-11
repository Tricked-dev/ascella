use crate::routes::prelude::*;

#[derive(Deserialize)]
struct PasteData {
  pub content: String,
}

#[get("/{paste}")]
pub async fn get(paste: web::Path<String>) -> Result<HttpResponse, Error> {
  let paste = get_paste_content::exec(paste.to_string()).await;
  println!("{:#?}", paste);
  match paste {
    Ok(paste) => Ok(HttpResponse::Ok().json(&json!({ "content": &paste }))),
    Err(_) => Err(Error::NotFound),
  }
}

#[post("/")]
pub async fn post(_req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
  let result = from_str(std::str::from_utf8(&body).unwrap());
  let data: PasteData = match result {
    Ok(v) => v,
    _ => return Err(Error::BadRequest),
  };

  if data.content.chars().count() > 50_000 {
    return Err(Error::BadRequest);
  };

  let paste = create_paste::exec(ran_str(), data.content).await;
  match paste {
    Ok(paste) => Ok(HttpResponse::Ok().json(&json!({ "content": &paste.id }))),
    _ => Err(Error::BadRequest),
  }
}
