use ascella_ratelimit::{Governor, GovernorConfigBuilder};
pub use ascella_util::*;
pub mod routes;

use paperclip::actix::web;
use routes::v2::*;

pub fn set_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(config::get)
        .service(domain::post)
        .service(domains::get)
        .service(embed::post)
        .service(reviews::get)
        .service(public::post)
        .service(redirect::post)
        .service(stats::get)
        .service(verify::post)
        .service(view::getpng)
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
        );
}
