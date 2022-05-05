use crate::prelude::*;
pub fn command(domain_options: impl IntoIterator<Item = (String, String)>) -> Command {
  CommandBuilder::new("domain".into(), "Select a domain to be used when uploading.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("domain".into(), "Select the domain here".into()).required(true).choices(domain_options))
    .option(StringBuilder::new("subdomain".into(), "Funny subdomain to add.".into()).required(false))
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let command_args = cmd.data.options.iter();

  let domain = get_arg(command_args.clone(), "domain").unwrap();

  let sub_domain = get_arg(command_args.clone(), "subdomain").map(|a| format!("{}.", a));

  let data = format!("https://{}{}", sub_domain.unwrap_or_else(|| "".to_owned()), domain);

  let embed = create_embed().title("Domain").description(user.lang().domain_update(data.clone())).build();

  set_domain_discord::exec(user.discord_id, data).await?;

  Ok(BotResponse::new().embed(embed))
}
