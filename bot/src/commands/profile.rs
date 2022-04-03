use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "profile".into(),
        "View your profile.".into(),
        CommandType::ChatInput,
    )
    .build()
}

#[allow(clippy::or_fun_call)]

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
    let new_key = if !user.upload_key.is_some() {
        let key = ulid::Ulid::new().to_string();
        set_upload_key::exec(user.id, &key).await?;
        Some(key)
    } else {
        None
    };
    let images = get_user_image_count::exec(user.id).await?;

    let message = format!(
        "dashboard: https://dash.ascella.host\nid: `{id}`\nname: `{name}`\ndiscord_id: `{discord}`\npassword: `{pass}`\nautodelete images: `{auto}`\n\ndomain: `{domain}`\nimages: `{images}`\n\ndownload config [here](https://ascella.wtf/v2/ascella/config?id={id}&key={pass})\n```json\n{config}\n```",
            id = user.id,
            name = user.name,
            discord = user.discord_id,
            pass = user.key,
            domain = user.domain,
            auto = user.autodelete.map(|x| { x.to_string() }).unwrap_or("Images wont get deleted automatically".to_owned()),
            images = images,
            config = serde_json::to_string_pretty(&create_config(&user.upload_key.unwrap_or(new_key.unwrap()))).unwrap()
        );

    let embed = create_embed()
        .title("User profile")
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
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
