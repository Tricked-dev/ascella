use super::super::prelude::*;

pub fn command() -> Command {
  CommandBuilder::new("codes".into(), "View your codes.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let codes = get_codes::exec(user.id).await?;

  let message = if !codes.is_empty() {
    codes
      .iter()
      .map(|r| {
        if r.claimed_by.is_some() {
          format!("~~{}~~", r.key)
        } else {
          r.key.to_string()
        }
      })
      .collect::<Vec<String>>()
      .join("\n")
  } else {
    "You don't have any codes".to_owned()
  };

  let embed = create_embed().title("Codes owned").description(message).build()?;

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
