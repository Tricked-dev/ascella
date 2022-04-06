use crate::prelude::*;
/// Upload styles
/// 0 = default
/// 1 = ulid id
/// 2 = gfycat
/// 3 = zws

pub fn command() -> Command {
  CommandBuilder::new("url_style".into(), "Change the url style of your uploads".into(), CommandType::ChatInput)
    .option(
      IntegerBuilder::new("style".into(), "Change your url style!".to_owned())
        .choices(vec![("default".to_owned(), 0), ("ulid".to_owned(), 1), ("gfycat".to_owned(), 2), ("zws".to_owned(), 3)])
        .required(true),
    )
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let style = get_arg_int(cmd.data.options.iter(), "style");

  set_url_style::exec(user.id, style.unwrap().try_into().unwrap()).await?;

  let embed = create_embed().title("Succesfully changed url style!".to_owned()).build()?;

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
