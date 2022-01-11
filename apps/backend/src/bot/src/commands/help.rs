use super::super::prelude::*;

pub fn command() -> Command {
  CommandBuilder::new("help".into(), "Ascella bot help command.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
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
        content: Some(String::from("help help help help help.")),
        embeds: Some(vec![]),
        flags: Some(MessageFlags::EPHEMERAL),
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
