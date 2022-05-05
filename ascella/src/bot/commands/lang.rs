use crate::prelude::*;
/// Upload styles
/// 0 = default
/// 1 = ulid id
/// 2 = gfycat
/// 3 = zws
/// 4 = hacker

pub fn command() -> Command {
  CommandBuilder::new("language".into(), "Change the bots language".into(), CommandType::ChatInput)
    .option(
      StringBuilder::new("lang".into(), "The language to change to".to_owned())
        .choices(vec![
          ("English".to_owned(), "en".into()),
          ("Spanish".to_owned(), "es".into()),
          ("Dutch".to_owned(), "nl".into()),
          ("Polish".to_owned(), "pl".into()),
          ("Fr*nch".to_owned(), "fr".into()),
          ("Bulgarian".to_owned(), "bg".into()),
        ])
        .required(true),
    )
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let style = get_arg(cmd.data.options.iter(), "lang");

  set_language::exec(user.id, style.unwrap()).await?;

  let embed = create_embed().title(user.lang().language_updated()).build();

  Ok(BotResponse::new().embed(embed))
}
