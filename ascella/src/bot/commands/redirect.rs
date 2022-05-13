use crate::prelude::*;
pub fn command(domain_options: impl IntoIterator<Item = (String, String)>) -> Command {
  CommandBuilder::new(Lang::fallback().redirect_name().into(), Lang::fallback().redeem_desc().into(), CommandType::ChatInput)
    .option(StringBuilder::new("url".into(), "Url you want to redirect to.".into()).required(true))
    .option(StringBuilder::new("vanity".into(), "Vanity to be created for the url.".into()).required(true))
    .option(StringBuilder::new("domain".into(), "Domain to be used.".into()).required(true).choices(domain_options))
    .localize()
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let command_args = cmd.data.options.iter();

  let url = get_arg_default(command_args.clone(), "url", "https//tricked.pro/aethor");
  let vanity = get_arg_default(command_args.clone(), "vanity", "https//tricked.pro/aethor");
  let domain = get_arg_default(command_args.clone(), "domain", "https//i.tricked.pro");

  let data = format!("https://{}/{}", domain, vanity);

  let embed = create_embed()
    .title(user.lang().redirect_create_title())
    .url(&data)
    .description(user.lang().redirect_create_desc(&data))
    .build();
  create_redirect::exec(user.id, url, vanity).await?;
  Ok(BotResponse::wembed(embed).content(data))
}
