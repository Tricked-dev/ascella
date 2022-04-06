use crate::prelude::*;
pub fn command(domain_options: impl IntoIterator<Item = (String, String)>) -> Command {
  CommandBuilder::new("domain".into(), "Select a domain to be used when uploading.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("domain".into(), "Select the domain here".into()).required(true).choices(domain_options))
    .option(StringBuilder::new("subdomain".into(), "Funny subdomain to add.".into()).required(false))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let command_args = cmd.data.options.iter();

  let domain = get_arg(command_args.clone(), "domain").unwrap();

  let sub_domain = get_arg(command_args.clone(), "subdomain").map(|a| format!("{}.", a));

  let data = format!("https://{}{}", sub_domain.unwrap_or_else(|| "".to_owned()), domain);
  println!("{}", &data);
  set_domain_discord::exec(user.discord_id, data.clone()).await?;

  let embed = create_embed().title("Embed").description(format!("Updated your domain to {}", data)).build()?;

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
