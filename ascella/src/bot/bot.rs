use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::ActivityType;
use twilight_model::gateway::presence::MinimalActivity;
use twilight_model::gateway::presence::Status;

use crate::bot::commands;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct Comment {
  avatar: String,
  comment: String,
  name: String,
}

pub async fn start_bot() -> Result<()> {
  let domains = get_domains::exec().await?;

  let domain_options: Vec<(String, String)> = domains.iter().map(|d| (d.domain.clone(), d.domain.clone())).collect();

  let token = dotenv::var("DISCORD_TOKEN")?;
  log::info!("Starting bot");
  let (cluster, mut events) = Cluster::builder(token.clone(), Intents::GUILD_MESSAGES)
    .presence(
      UpdatePresencePayload::new(
        vec![
          MinimalActivity {
            kind: ActivityType::Playing,
            name: "Ascella best bot".to_string(),
            url: None,
          }
          .into(),
          MinimalActivity {
            kind: ActivityType::Competing,
            name: "Being the fastest uploader".to_string(),
            url: None,
          }
          .into(),
        ],
        false,
        None,
        Status::Idle,
      )
      .unwrap(),
    )
    .shard_scheme(ShardScheme::Auto)
    .build()
    .await?;

  cluster.up().await;
  let http = Arc::new(Client::new(token));

  HTTP.set(Arc::clone(&http)).unwrap();
  tokio::spawn(async move {
    let commands = get_commands(domain_options);

    HTTP
      .get()
      .unwrap()
      .interaction(Id::new(env::var("APPLICATION_ID").unwrap().parse::<u64>().unwrap()))
      .set_guild_commands(Id::new(748956745409232945), commands.as_ref())
      .exec()
      .await
      .ok();
    log::info!("Set commands!")
  });

  if START_TIME.get().is_none() {
    START_TIME.set(Instant::now()).expect("Failed to set starttime");
  }
  while let Some((_shard_id, event)) = events.next().await {
    match event {
      Event::InteractionCreate(inter) => match inter.0 {
        Interaction::Ping(_) => {}
        Interaction::ApplicationCommand(cmd) => {
          log::info!("slash command called {} {}", cmd.data.name, cmd.member.as_ref().unwrap().user.as_ref().unwrap().name);
          let r = commands::builtin_exec(&http, &cmd).await;
          if let Err(r) = r {
            send_text_webhook(format!(
              "**{}**: {}, ERROR OCCURRED {:?}",
              cmd.data.name.as_str(),
              cmd.member.as_ref().unwrap().user.as_ref().unwrap().name,
              r
            ))
            .await?;
          }
        }
        Interaction::MessageComponent(_) => {}
        _ => {}
      },
      Event::Ready(_) => {
        log::info!("Bot connected");
        if REVIEWS.get().is_none() {
          let pins = http
            .pins(Id::new(937239935545663498))
            .exec()
            .await?
            .models()
            .await?
            .iter()
            .map(|x| {
              let comment = x.content.clone();
              let user = &x.author;
              let name = user.name.clone();

              Comment {
                avatar: if let Some(avatar) = user.avatar {
                  format!("https://cdn.discordapp.com/avatars/{}/{}.png", &user.id, avatar)
                } else {
                  String::new()
                },
                comment,
                name,
              }
            })
            .collect::<Vec<Comment>>();
          REVIEWS.set(pins).ok();
        };
      }
      _ => {}
    }
  }

  Ok(())
}
