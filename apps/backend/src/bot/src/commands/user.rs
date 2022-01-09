use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "user".into(),
        "View the profile of a user.".into(),
        CommandType::ChatInput,
    )
    .option(
        UserBuilder::new(
            "user".into(),
            "The user you want to view the profile of.".into(),
        )
        .required(true),
    )
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
    let id = cmd.data.options.iter().find(|e| e.name == "user").unwrap();
    if let CommandOptionValue::User(user) = id.value {
        if let Ok(user) = get_user_discord::exec(user.to_string()).await {
            let images = get_user_image_count::exec(user.id).await?;

            let message = format!(
                "name: `{name}`\ndiscord_id: `{discord}`\n\ndomain: `{domain}`\nimages: `{images}`",
                name = user.name,
                discord = user.discord_id,
                domain = user.domain,
                images = images,
            );

            let embed = create_embed()
                .title(format!("Profile of {}", &user.name))
                .description(message)
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
            return Ok(());
        }
    }
    let embed = create_embed()
        .title("User profile")
        .description("User isn't a registered user")
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
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
