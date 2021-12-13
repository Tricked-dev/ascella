use crate::commands;
use crate::prelude::*;

pub async fn start_bot() -> Result<()> {
    dotenv::dotenv().ok();
    let domains = get_domains::exec().await?;

    let domain_options: Vec<(String, String)> = domains
        .iter()
        .map(|d| (d.domain.clone(), d.domain.clone()))
        .collect();

    let token = dotenv::var("DISCORD_TOKEN")?;

    let (cluster, mut events) = Cluster::builder(&*token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    cluster.up().await;

    let http = Client::new((&*token).to_string());
    http.set_application_id(ApplicationId(
        env::var("APPLICATION_ID")
            .unwrap()
            .parse::<core::num::NonZeroU64>()
            .unwrap(),
    ));

    let commands = get_commands(domain_options);

    let data = http
        .set_guild_commands(
            GuildId(core::num::NonZeroU64::new(748956745409232945).unwrap()),
            commands.as_ref(),
        )
        .unwrap();

    data.exec().await?;
    START_TIME.set(Instant::now()).unwrap();
    while let Some((_shard_id, event)) = events.next().await {
        match event {
            Event::InteractionCreate(inter) => match inter.0 {
                Interaction::Ping(_) => {}
                Interaction::ApplicationCommand(cmd) => {
                    log::info!(
                        "slash command called {} {}",
                        cmd.data.name,
                        cmd.member.as_ref().unwrap().user.as_ref().unwrap().name
                    );
                    let r = commands::builtin_exec(&http, &cmd).await;
                    if let Err(r) = r {
                        http.interaction_callback(
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
                                embeds: vec![],
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
                log::info!("Bot got on")
            }
            _ => {}
        }
    }

    Ok(())
}
