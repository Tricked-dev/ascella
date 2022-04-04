use crate::routes::prelude::*;

#[derive(Deserialize)]
struct DomainData {
    domain: String,
}

#[api_v2_operation]
#[post("/domain")]
pub async fn post(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
    if let Ok(data) = validate_request(&req).await {
        let result = from_str(std::str::from_utf8(&body).unwrap()); // return Result
        let domain_info: DomainData = match result {
            Ok(v) => v,
            _ => return Err(Error::BadRequest),
        };
        set_domain::exec(data.id, domain_info.domain)
            .await
            .map_err(|_| Error::BadRequest)?;
        Ok(HttpResponse::Ok().json(&send_message(
            200,
            true,
            "Successfully updated your domain.",
        )))
    } else {
        Err(Error::NotAuthorized)
    }
}
