use actix_cors::Cors;
use actix_web::{middleware, ResponseError};
use actix_web::{App, HttpServer};
use futures::executor::block_on;
use paperclip::actix::{web, OpenApiExt};
use paperclip::v2::models::{Contact, DefaultApiRaw, Info, License};

use tsunami::bot::{bot::HTTP, start_bot, utils::create_embed};
use tsunami::http::set_endpoints;
use tsunami::prelude::get_images::delete_all;
use tsunami::prelude::{get_users_autodelete, ChannelId, Error};
use tsunami::ratelimit::{Governor, GovernorConfigBuilder};

async fn init() -> std::io::Result<()> {
  tracing_subscriber::fmt().init();

  HttpServer::new(|| {
    let spec = DefaultApiRaw {
      info: Info {
        version: "2.0".into(),
        title: "Ascella Image uploader".into(),
        description: Some("Ascella is the fastest image uploader utilizing rust to bring you the fastest upload speeds".into()),
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
      host: Some("https://ascella.wtf".into()),
      ..DefaultApiRaw::default()
    };

    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().max_age(3600);

    App::new()
      .wrap_api_with_spec(spec)
      .wrap(cors)
      .wrap(Governor::new(&GovernorConfigBuilder::default().per_second(60).burst_size(30).finish().unwrap()))
      .wrap(Governor::new(
        &GovernorConfigBuilder::default().per_second(3600).burst_size(128).finish().unwrap(),
      ))
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

fn main() -> std::io::Result<()> {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("ascella-rt")
    .thread_stack_size(3 * 1024 * 1024)
    .enable_all()
    .build()
    .unwrap();

  rt.spawn(start_bot());
  rt.spawn(async {
    let mut sched = tokio_cron_scheduler::JobScheduler::new();

    sched
      .add(
        tokio_cron_scheduler::Job::new_repeated(
          //one day
          core::time::Duration::from_secs(86400000),
          |_, _| {
            tokio::spawn(async {
              //This removes the startup run but it isnt needed anyway!
              if let Some(client) = HTTP.get() {
                let users = get_users_autodelete::exec().await.unwrap();
                let summary: Vec<String> = users
                  .iter()
                  .filter_map(|x| {
                    block_on(async {
                      let amount = delete_all(x.0, x.1).await.unwrap();
                      if amount == 0 {
                        None
                      } else {
                        Some(format!("{}: `{}`", x.2, amount))
                      }
                    })
                  })
                  .collect();
                if summary.is_empty() {
                  return;
                }

                let embed = create_embed().title("Deleted images summary").description(&summary.join("\n")).build().unwrap();
                client
                  .create_message(ChannelId::new(929698255300882522u64).unwrap())
                  .embeds(&vec![embed])
                  .unwrap()
                  .exec()
                  .await
                  .unwrap();
              }
            });
          },
        )
        .unwrap(),
      )
      .unwrap();

    sched.start().await.unwrap();
  });
  rt.block_on(init())
}
