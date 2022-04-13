use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("gencodes".into(), "Give codes to boss :sunglasses:".into(), CommandType::ChatInput)
    .option(IntegerBuilder::new("codes".into(), "Codes to generate".into()).required(true))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let codes = get_arg_int(cmd.data.options.iter(), "codes").unwrap();

  for _ in 0..codes {
    create_code::exec(user.id, ran_str(6)).await?;
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
        content: Some(format!("Generated **{}** codes", codes)),
        embeds: Some(vec![]),
        flags: Some(MessageFlags::EPHEMERAL),
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
