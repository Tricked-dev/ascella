use crate::prelude::Error;
use crate::ratelimit::{Governor, GovernorConfigBuilder};
use routes::v2::*;

use actix_cors::Cors;
use actix_web::{middleware, ResponseError};
use actix_web::{App, HttpServer};
use paperclip::actix::{web, OpenApiExt};
use paperclip::v2::models::{Contact, DefaultApiRaw, Info, License, Tag};
use tokio::sync::Mutex;

pub mod models;
pub mod routes;
#[derive(Clone)]
pub struct ImageCache {
  key: String,
  inner: Vec<u8>,
  content_type: String,
}

impl ImageCache {
  pub fn new() -> Self {
    Self {
      key: String::new(),
      inner: Vec::new(),
      content_type: String::new(),
    }
  }

  pub fn set(&mut self, key: String, content_type: String, inner: Vec<u8>) {
    self.inner = inner;
    self.key = key;
    self.content_type = content_type;
  }
  pub fn get(&self) -> &[u8] {
    &self.inner
  }
  pub fn get_key(&self) -> &str {
    &self.key
  }
  pub fn get_content_type(&self) -> &str {
    &self.content_type
  }
}

pub fn set_endpoints(cfg: &mut web::ServiceConfig) {
  cfg
    .service(config::get)
    .service(domain::post)
    .service(domains::get)
    .service(embed::post)
    .service(reviews::get)
    .service(public::post)
    .service(redirect::post)
    .service(stats::get)
    .service(verify::post)
    .service(view::get)
    .service(images::post)
    .service(ascella_stats::get)
    .service(lang::get)
    .service(upload_style::post)
    .service(
      web::scope("")
        .wrap(Governor::new(&GovernorConfigBuilder::default().per_second(120).burst_size(10).finish().unwrap()))
        .service(upload::post),
    );
}
pub async fn start_actix() -> std::io::Result<()> {
  let data = web::Data::new(Mutex::new(ImageCache::new()));

  HttpServer::new(move || {
    let spec = DefaultApiRaw {
      info: Info {
        version: "2.0".into(),
        title: "Ascella Image uploader".into(),
        description: Some(include_str!("../../api_description.md").into()),
        contact: Some(Contact {
          name: Some("Tricked".into()),
          url: Some("https://tricked.pro".into()),
          email: Some("tricked@tricked.pro".into()),
        }),
        license: Some(License {
          name: Some("AGPL-3.0".into()),
          url: Some("https://github.com/Tricked-dev/ascella/blob/master/LICENSE".into()),
        }),
        ..Default::default()
      },
      tags: vec![
        Tag {
          name: "Images".to_string(),
          description: Some("Stuff related to images".to_string()),
          external_docs: None,
        },
        Tag {
          name: "Dashboard".to_string(),
          description: Some("Stuff related to the Dashboard".to_string()),
          external_docs: None,
        },
        Tag {
          name: "Etc".to_string(),
          description: Some("Stuff not related to the above".to_string()),
          external_docs: None,
        },
      ],
      host: Some("ascella.wtf".into()),
      ..DefaultApiRaw::default()
    };

    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().max_age(3600);

    App::new()
      .service(openapi_3::get)
      .wrap_api_with_spec(spec)
      .app_data(data.clone())
      .wrap(cors)
      .wrap(Governor::new(&GovernorConfigBuilder::default().per_second(60).burst_size(30).finish().unwrap()))
      .wrap(Governor::new(&GovernorConfigBuilder::default().per_second(3600).burst_size(128).finish().unwrap()))
      .wrap(middleware::Logger::default())
      .with_json_spec_at("/v2/ascella/spec/v2")
      .service(web::scope("/v2/ascella").configure(set_endpoints))
      .default_service(web::to(|| async {
        Error::Four04 {
          message: "Path not found.".to_owned(),
        }
        .error_response()
      }))
      .build()
  })
  .bind("0.0.0.0:7878")?
  .run()
  .await
}
