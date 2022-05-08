use crate::prelude::*;
/// Upload styles
/// 0 = default
/// 1 = ulid id
/// 2 = gfycat
/// 3 = zws
/// 4 = hacker

pub fn command() -> Command {
  CommandBuilder::new("url_style".into(), Lang::En.url_style_desc().into(), CommandType::ChatInput)
    .option(
      IntegerBuilder::new("style".into(), "Change your url style!".to_owned())
        .choices(vec![
          ("default".to_owned(), 0),
          ("ulid".to_owned(), 1),
          ("gfycat".to_owned(), 2),
          ("zws".to_owned(), 3),
          ("hacker".to_owned(), 4),
        ])
        .required(true),
    )
    .localize()
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let style = get_arg_int(cmd.data.options.iter(), "style");

  set_url_style::exec(user.id, style.unwrap().try_into().unwrap()).await?;

  let embed = create_embed().title(user.lang().url_style_success()).build();

  Ok(BotResponse::new().embed(embed))
}
