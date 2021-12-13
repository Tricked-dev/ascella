use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "codedrop".into(),
        "Give everyone a free code".into(),
        CommandType::ChatInput,
    )
    .option(IntegerBuilder::new("codes".into(), "Codes to generate".into()).required(true))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, _: Users) -> Result<()> {
    let code = get_arg_int(cmd.data.options.iter(), "codes").unwrap();
    let users = get_all_users::exec().await?;
    let users_len: i64 = users.len().try_into().unwrap();
    for user in users {
        for _ in 0..code {
            create_code::exec(user.id, ran_str()).await?;
        }
    }

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
                content: Some(format!("Generated **{}** codes", users_len * code)),
                embeds: vec![],
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
