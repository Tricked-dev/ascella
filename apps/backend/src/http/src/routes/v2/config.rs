use crate::routes::prelude::*;

#[get("/config")]
pub async fn get(req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut query: HashMap<String, String> = HashMap::new();
    let t: Vec<Vec<&str>> = req
        .query_string()
        .split('&')
        .map(|x| x.split('=').collect::<Vec<&str>>())
        .collect();
    for item in t.iter() {
        if item.len() != 2 {
            return Err(Error::BadRequest);
        }
        query.insert(item[0].to_string(), item[1].to_string());
    }

    let id = query.get("id");
    let key = query.get("key");

    let data = match (key, id) {
        (Some(key), Some(id)) => Some((key.to_owned(), id.parse::<i32>().unwrap_or(0))),
        _ => match validate_request(&req).await {
            Ok(user) => Some((user.key, user.id)),
            _ => None,
        },
    };

    if let Some((key, id)) = data {
        Ok(HttpResponse::Ok()
            .append_header(("content-disposition", "attachment;filename=ascella.sxcu"))
            .append_header(("content-type", "application/octet-stream"))
            .json(create_config(id, &key)))
    } else {
        Err(Error::BadRequest)
    }
}
