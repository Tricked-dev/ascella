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
  let (cluster, mut events) = Cluster::builder(&*token, Intents::GUILD_MESSAGES)
    .presence(
      UpdatePresencePayload::new(
        vec![MinimalActivity {
          kind: ActivityType::Playing,
          name: "Ascella best bot".to_string(),
          url: None,
        }
        .into()],
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
  log::info!("Bot started");
  let http = Arc::new(Client::new((&*token).to_string()));
  http.set_application_id(ApplicationId(env::var("APPLICATION_ID").unwrap().parse::<core::num::NonZeroU64>().unwrap()));

  let commands = get_commands(domain_options);

  HTTP.set(Arc::clone(&http)).unwrap();

  let data = http.set_guild_commands(GuildId(core::num::NonZeroU64::new(748956745409232945).unwrap()), commands.as_ref()).unwrap();

  data.exec().await?;
  while let Some((_shard_id, event)) = events.next().await {
    match event {
      Event::InteractionCreate(inter) => match inter.0 {
        Interaction::Ping(_) => {}
        Interaction::ApplicationCommand(cmd) => {
          log::info!("slash command called {} {}", cmd.data.name, cmd.member.as_ref().unwrap().user.as_ref().unwrap().name);
          let r = commands::builtin_exec(&http, &cmd).await;
          if let Err(r) = r {
            http
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
                  content: Some(String::from("Not a user of the image uploader")),
                  embeds: Some(vec![]),
                  flags: Some(MessageFlags::EPHEMERAL),
                  tts: Some(false),
                }),
              )
              .exec()
              .await?;
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
        if REVIEWS.get().is_none() {
          let pins = http
            .pins(ChannelId(core::num::NonZeroU64::new(937239935545663498).unwrap()))
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
                avatar: format!("https://cdn.discordapp.com/avatars/{}/{}.png", &user.id, &user.avatar.as_ref().unwrap_or(&"".to_owned())),
                comment,
                name,
              }
            })
            .collect::<Vec<Comment>>();
          REVIEWS.set(pins).ok();
        };

        log::info!("Bot connected")
      }
      _ => {}
    }
  }

  Ok(())
}
