use std::time::Instant;

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tsunami::bot::start_bot;
use tsunami::cron::start_cron;
use tsunami::http::start_actix;
use tsunami::prelude::START_TIME;

fn main() -> std::io::Result<()> {
  if let Ok(url) = dotenv::var("SENTRY_URL") {
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).with(sentry_tracing::layer()).try_init().unwrap();
    let _guard = sentry::init((
      url,
      sentry::ClientOptions {
        release: sentry::release_name!(),
        attach_stacktrace: true,

        ..Default::default()
      },
    ));

    log::info!("Sentry is enabled");
  } else {
    tracing_subscriber::fmt().init();
  }
  if START_TIME.get().is_none() {
    START_TIME.set(Instant::now()).expect("Failed to set starttime");
  }
  let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("ascella-rt")
    .thread_stack_size(3 * 1024 * 1024)
    .enable_all()
    .build()
    .unwrap();

  rt.spawn(start_bot());
  rt.spawn(start_cron());
  rt.block_on(start_actix())
}
