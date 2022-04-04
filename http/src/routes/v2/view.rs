use crate::routes::prelude::*;
use ascella_database::s3::{get_file, S3};
use cached::proc_macro::cached;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, OpenApiExt,
};
use std::io;

#[api_v2_operation]
#[get("/view/{image}")]
pub async fn get(image: web::Path<String>) -> Result<HttpResponse, Error> {
    let data = get_image_vanity_only::exec(image.to_string()).await;
    if let Ok(image) = data {
        let data = get_file(format!("{}/{}", image.owner, image.id)).await;
        match data {
            Ok(data) => Ok(HttpResponse::Ok()
                .append_header(("content-type", image.content_type))
                .append_header(("cache-control", "public, max-age=604800, immutable"))
                .body(data)),
            _ => Err(Error::NotFound),
        }
    } else {
        Err(Error::NotFound)
    }
}
#[api_v2_operation]
#[get("/view/{image}.png")]
pub async fn getpng(image: web::Path<String>) -> Result<HttpResponse, Error> {
    let data = get_image_vanity_only::exec(image.to_string()).await;
    if let Ok(image) = data {
        let data = get_file(format!("{}/{}", image.owner, image.id)).await;
        match data {
            Ok(data) => Ok(HttpResponse::Ok()
                .append_header(("content-type", image.content_type))
                .append_header(("cache-control", "public, max-age=604800, immutable"))
                .body(data)),
            _ => Err(Error::NotFound),
        }
    } else {
        Err(Error::NotFound)
    }
}
