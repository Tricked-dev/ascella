use crate::prelude::*;
pub fn command(domain_options: impl IntoIterator<Item = (String, String)>) -> Command {
  CommandBuilder::new("redirect".into(), "Create a redirect.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("url".into(), "Url you want to redirect to.".into()).required(true))
    .option(StringBuilder::new("vanity".into(), "Vanity to be created for the url.".into()).required(true))
    .option(StringBuilder::new("domain".into(), "Domain to be used.".into()).required(true).choices(domain_options))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let command_args = cmd.data.options.iter();

  let url = get_arg_default(command_args.clone(), "url", "https//tricked.pro/aethor");
  let vanity = get_arg_default(command_args.clone(), "vanity", "https//tricked.pro/aethor");
  let domain = get_arg_default(command_args.clone(), "domain", "https//i.tricked.pro");

  let data = format!("https://{}/{}", domain, vanity);
  println!("{}", data);
  let embed = create_embed()
    .title("Successfully created the redirect")
    .url(&data)
    .description(format!("Made a fancy vanity {}", &data))
    .build()?;
  create_redirect::exec(user.id, url, vanity).await?;

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
        content: Some(data),
        embeds: Some(vec![embed]),
        flags: None,
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
