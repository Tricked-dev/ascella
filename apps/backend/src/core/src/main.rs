// use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, ResponseError};
use ascella_bot::start_bot;
use ascella_http::{routes::v2::*, Error};
use ascella_ratelimit::{Governor, GovernorConfigBuilder};
// use ascella_ratelimit::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();

    tokio::spawn(start_bot());
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
                            .service(public::post)
                            .service(redirect::post)
                            .service(stats::get)
                            .service(verify::post)
                            .service(view::get)
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
        // .default_service(web::route().to(|| {
        //     Error::Four04 {
        //         message: "Path not found.".to_owned(),
        //     }
        //     .error_response()
        // }))
    })
    .bind("0.0.0.0:7878")?
    .run()
    .await
}
