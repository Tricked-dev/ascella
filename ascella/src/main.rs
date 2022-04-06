use tsunami::bot::start_bot;
use tsunami::cron::start_cron;
use tsunami::http::start_actix;

fn main() -> std::io::Result<()> {
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
