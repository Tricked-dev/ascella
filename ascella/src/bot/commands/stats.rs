use crate::prelude::*;
fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
  fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
    dir.try_fold(0, |acc, file| {
      let file = file?;
      let size = match file.metadata()? {
        data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
        data => data.len(),
      };
      Ok(acc + size)
    })
  }

  dir_size(fs::read_dir(path.into())?)
}

async fn get_count(part: &'static str) -> i64 {
  let data = get_tokio_postgres()
    .await
    .query_one(format!("SELECT count(*) FROM {}", part).as_str(), &[])
    .await
    .unwrap();
  data.get("count")
}

fn bytes_to(bytes: u128) -> String {
  let bytes = Byte::from_bytes(bytes);
  let adjusted_byte = bytes.get_appropriate_unit(false);
  adjusted_byte.to_string()
}

pub fn command() -> twilight_model::application::command::Command {
  CommandBuilder::new("stats".into(), "View some stats about the uploader.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
  let image_count = get_count("images").await;
  let domains_count = get_count("domains").await;
  let users_count = get_count("users").await;

  let mem = ProcessCommand::new("ps")
    .args(vec!["-o", "rss="])
    .arg(format!("{}", process::id()))
    .output()
    .unwrap();
  let usage = std::str::from_utf8(&mem.stdout).unwrap();

  let now = Instant::now();
  let time = format_duration(now.duration_since(*START_TIME.get().unwrap())).to_string();
  let embed = create_embed()
    .title("Cool stats")
    .field(EmbedFieldBuilder::new("Uploads", image_count.to_string()).inline())
    .field(EmbedFieldBuilder::new("Domains", domains_count.to_string()).inline())
    .field(EmbedFieldBuilder::new("Users", users_count.to_string()).inline())
    .field(EmbedFieldBuilder::new("Created", "<t:1629305469:R>").inline())
    .field(
      EmbedFieldBuilder::new(
        "Memory Usage",
        bytes_to(if let Ok(r) = usage.trim_end().parse::<u128>().map(|x| x * 1024) {
          r
        } else {
          100000
        }),
      )
      .inline(),
    )
    .field(EmbedFieldBuilder::new("Upload's Size", bytes_to(dir_size("images").unwrap().into())).inline())
    .field(EmbedFieldBuilder::new("Discord-API version", "9").inline())
    .field(EmbedFieldBuilder::new("Uptime", time.to_string()).inline())
    .field(EmbedFieldBuilder::new("Fast", "true, as always").inline())
    .field(EmbedFieldBuilder::new("Rustc info", env!("RUST_DATA")).inline())
    .field(
      EmbedFieldBuilder::new(
        "Commit Hash",
        format!(
          "[{short_hash}](https://github.com/Tricked-dev/ascella-backend/commit/{hash})",
          short_hash = &env!("GIT_HASH")[..7],
          hash = env!("GIT_HASH")
        ),
      )
      .inline(),
    )
    .build()?;

  client
    .interaction_callback(
      cmd.id,
      &cmd.token,
      &ChannelMessageWithSource(CallbackData {
        allowed_mentions: Some(AllowedMentions {
          parse: vec![],
          users: vec![],
          roles: vec![],
          replied_user: true,
        }),
        components: None,
        content: None,
        embeds: Some(vec![embed]),
        flags: None,
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
