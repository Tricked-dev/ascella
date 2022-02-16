use actix_web::{middleware, web, App, HttpServer, ResponseError};
use ascella_bot::{
    bot::HTTP,
    prelude::{get_images::delete_all, get_users_autodelete, ChannelId},
    start_bot,
    utils::create_embed,
};

use ascella_http::{routes::v2::*, Error};
use ascella_ratelimit::{Governor, GovernorConfigBuilder};
use futures::future;

async fn init() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    // tokio::spawn();

    HttpServer::new(|| {
        App::new()
            // .wrap(
            //     Cors::default()
            //         .allowed_origin_fn(|_, _| true)
            //         .allowed_methods(vec!["GET", "POST"])
            //         .supports_credentials(),
            // )
            .wrap(Governor::new(
                &GovernorConfigBuilder::default()
                    .per_second(60)
                    .burst_size(30)
                    .finish()
                    .unwrap(),
            ))
            .wrap(Governor::new(
                &GovernorConfigBuilder::default()
                    .per_second(3600)
                    .burst_size(128)
                    .finish()
                    .unwrap(),
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v2")
                    .service(
                        web::scope("/ascella")
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
                            .service(
                                web::scope("")
                                    .wrap(Governor::new(
                                        &GovernorConfigBuilder::default()
                                            .per_second(120)
                                            .burst_size(10)
                                            .finish()
                                            .unwrap(),
                                    ))
                                    .service(upload::post),
                            ),
                    )
                    .service(
                        web::scope("/paste")
                            .service(paste::get)
                            .service(paste::post),
                    ),
            )
            .default_service(web::route().to(|| {
                Error::Four04 {
                    message: "Path not found.".to_owned(),
                }
                .error_response()
            }))
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
                                let summary: Vec<String> =
                                    future::join_all(users.iter().map(|x| async {
                                        let amount = delete_all(x.0, x.1).await;
                                        format!("{}: `{}`", x.2, amount.unwrap())
                                    }))
                                    .await;
                                if summary.is_empty() {
                                    return;
                                }

                                let embed = create_embed()
                                    .title("Deleted images summary")
                                    .description(&summary.join("\n"))
                                    .build()
                                    .unwrap();
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
