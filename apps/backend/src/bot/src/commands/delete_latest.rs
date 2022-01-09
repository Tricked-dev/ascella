use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "delete_latest".into(),
        "Delete your latest upload".into(),
        CommandType::ChatInput,
    )
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
    let img = get_latest_image::exec(user.id).await?;
    delete_image::exec(img.id).await?;

    let embed = create_embed()
        .title("Deleted your latest image ;)")
        .description(format!(
            "Deleted image {vanity} with id {id}",
            id = &img.id,
            vanity = &img.vanity
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
                embeds: Some(vec![embed]),
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
