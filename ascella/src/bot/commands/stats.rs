#![allow(deprecated)]

use crate::prelude::*;
use tokio::join;
use twilight_embed_builder::EmbedFieldBuilder;

async fn get_count(part: &'static str) -> i64 {
  let data = get_tokio_postgres().await.query_one(format!("SELECT count(*) FROM {}", part).as_str(), &[]).await.unwrap();
  data.get("count")
}

fn bytes_to(bytes: u128) -> String {
  let bytes = Byte::from_bytes(bytes);
  let adjusted_byte = bytes.get_appropriate_unit(false);
  adjusted_byte.to_string()
}

pub fn command() -> twilight_model::application::command::Command {
  CommandBuilder::new(Lang::fallback().stats_name().into(), Lang::fallback().stats_desc().into(), CommandType::ChatInput).build()
}

#[derive(Serialize, Deserialize, Apiv2Schema)]

pub struct AscellaStats {
  /// Total amount of images uploade to ascella  (includes redirects)
  #[openapi(example = "6441")]
  pub(crate) total_uploads: i64,
  /// Total amount of views a image has
  #[openapi(example = "87545")]
  pub(crate) total_views: i64,
  /// The amount of domains ascella has use the domains endpoint to find out which
  #[openapi(example = "17")]
  pub(crate) total_domains: i64,
  /// Amount of userst that ran /redeem
  #[openapi(example = "100")]
  pub(crate) total_users: i64,
  /// The unix timestamp ascella was created
  #[openapi(example = "1629305469")]
  pub(crate) created_date: i64,
  /// Memory usage by the rust process
  #[openapi(example = "56.89 MB")]
  pub(crate) usage: String,
  /// Size of uploads currently broken :(
  pub(crate) upload_size: String,
  /// The api version twilight uses
  #[openapi(example = "10")]
  pub(crate) discord_api_version: i8,
  /// Ascella uptime - since last restart for update
  #[openapi(example = "1day 10h 7m 42s 572ms 387us 449ns")]
  pub(crate) uptime: String,
  /// If ascella is fast obviously
  pub(crate) fast: bool,
  /// Rustc info
  #[openapi(example = "rustc 1.62.0-nightly (4c60a0ea5 2022-05-04)")]
  pub(crate) rustc: String,
  /// THe commit hash used by ascella
  #[openapi(example = "https://github.com/Tricked-dev/ascella/commit/e968eb666fa3b0ea974248c30931a9210919fd44")]
  pub(crate) commit_hash: String,
}

impl AscellaStats {
  pub fn new(total_uploads: i64, total_domains: i64, total_users: i64, total_views: i64) -> Self {
    let mem = ProcessCommand::new("ps").args(vec!["-o", "rss="]).arg(format!("{}", process::id())).output().unwrap();
    let usage = std::str::from_utf8(&mem.stdout).unwrap();
    Self {
      total_uploads,
      total_domains,
      total_users,
      total_views,
      created_date: 1629305469,
      usage: bytes_to(if let Ok(r) = usage.trim_end().parse::<u128>().map(|x| x * 1024) { r } else { 100000 }),
      upload_size: "0".into(),
      discord_api_version: twilight_http::API_VERSION as i8,
      uptime: format_duration(Instant::now().duration_since(*START_TIME.get().unwrap_or(&Instant::now()))).to_string(),
      fast: true,
      rustc: env!("RUST_DATA").to_owned(),
      commit_hash: format!("https://github.com/Tricked-dev/ascella/commit/{hash}", hash = env!("GIT_HASH")),
    }
  }
  pub async fn new_with_stats() -> Self {
    let (image_count, domains_count, users_count, total_views) = join!(get_count("images"), get_count("domains"), get_count("users"), get_total_views::exec());
    AscellaStats::new(image_count, domains_count, users_count, total_views.expect("Failed to get total views"))
  }
}

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand) -> Result<BotResponse> {
  let stats = AscellaStats::new_with_stats().await;

  let embed = create_embed()
    .title("Cool stats")
    .field(EmbedFieldBuilder::new("Uploads", &stats.total_uploads.to_string()).inline())
    .field(EmbedFieldBuilder::new("Domains", &stats.total_domains.to_string()).inline())
    .field(EmbedFieldBuilder::new("Users", &stats.total_users.to_string()).inline())
    .field(EmbedFieldBuilder::new("Views", &stats.total_views.to_string()).inline())
    .field(EmbedFieldBuilder::new("Created", format!("<t:{}:R>", &stats.created_date)).inline())
    .field(EmbedFieldBuilder::new("Memory Usage", &stats.usage).inline())
    // .field(EmbedFieldBuilder::new("Upload's Size", &stats.upload_size).inline())
    .field(EmbedFieldBuilder::new("Discord-API version", &stats.discord_api_version.to_string()).inline())
    .field(EmbedFieldBuilder::new("Uptime", &stats.uptime).inline())
    .field(EmbedFieldBuilder::new("Fast", &stats.fast.to_string()).inline())
    .field(EmbedFieldBuilder::new("Rustc info", &stats.rustc).inline())
    .field(EmbedFieldBuilder::new("Commit Hash", format!("[{}]({})", &env!("GIT_HASH")[..7], &stats.commit_hash)).inline())
    .build();

  Ok(BotResponse::new().embed(embed))
}
