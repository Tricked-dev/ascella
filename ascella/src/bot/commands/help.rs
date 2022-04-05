use super::super::prelude::*;

pub fn command() -> Command {
  CommandBuilder::new("help".into(), "Ascella bot help command.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
  let commands = get_commands(vec![]);
  let desc = commands
    .iter()
    .map(|x| format!("**{}**: {}", &x.name, &x.description))
    .collect::<Vec<String>>()
    .join("\n");
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
        content: Some(desc),
        embeds: Some(vec![]),
        flags: Some(MessageFlags::EPHEMERAL),
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
