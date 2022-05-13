use crate::prelude::*;
/// Upload styles
/// 0 = default
/// 1 = ulid id
/// 2 = gfycat
/// 3 = zws
/// 4 = hacker

pub fn command() -> Command {
  CommandBuilder::new(Lang::fallback().language_name().into(), Lang::fallback().language_desc().into(), CommandType::ChatInput)
    .option(
      StringBuilder::new("lang".into(), "The language to change to".to_owned())
        .choices(vec![
          ("English".to_owned(), "en".into()),
          ("Spanish".to_owned(), "es".into()),
          ("Dutch".to_owned(), "nl".into()),
          ("Polish".to_owned(), "pl".into()),
          ("Fr*nch".to_owned(), "fr".into()),
          ("Bulgarian".to_owned(), "bg".into()),
          ("Portuguese".to_owned(), "pt".into()),
        ])
        .required(true),
    )
    .localize()
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, mut user: Users) -> Result<BotResponse> {
  let language = get_arg(cmd.data.options.iter(), "lang").unwrap();

  set_language::exec(user.id, language.clone()).await?;
  user.set_lang(language);
  let embed = create_embed().title(user.lang().language_updated()).build();

  Ok(BotResponse::wembed(embed))
}
