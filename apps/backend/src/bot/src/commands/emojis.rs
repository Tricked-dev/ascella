use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "emojis".into(),
        "Enable emoji urls.".into(),
        CommandType::ChatInput,
    )
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
    set_emojis::exec(user.id, !user.emojis.unwrap_or(false)).await?;

    let embed = create_embed()
        .title(format!(
            "Turned emoji urls {}",
            if user.emojis.unwrap_or(false) {
                "off"
            } else {
                "on"
            }
        ))
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
                embeds: vec![embed],
                flags: None,
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
