use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "eval".into(),
        "Eval postgresql queries".into(),
        CommandType::ChatInput,
    )
    .option(StringBuilder::new("code".into(), "Postgresql Query".into()).required(true))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
    let code = get_arg(cmd.data.options.iter(), "code");
    let res = get_tokio_postgres()
        .await
        .query(&code.unwrap(), &[])
        .await?;

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
                content: Some(format!(
                    "{}\n```js\n{:?}\n```",
                    user.name,
                    res.iter().map(|row| { row.columns() })
                )),
                embeds: vec![],
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
