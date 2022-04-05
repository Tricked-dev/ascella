use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("domains".into(), "View the domains ascella has.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
  let data = get_domains::exec().await?;

  // data_embed

  let msg = future::join_all(data.into_iter().map(|e| async move {
    let (id, name) = get_discord_id::exec(e.owner).await.unwrap();

    format!(
      "**{domain}**\nApex: {apex}\nOwner: {mention}{id}",
      domain = e.domain,
      apex = e.apex,
      mention = format!("<@{}>", id),
      id = name
    )
  }))
  .await;
  let mut message: Vec<String> = vec![];
  for body in msg {
    message.push(body.to_string());
  }

  let embed = create_embed().title("Embed").description(message.join("\n")).build()?;

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

  Ok(())
}
