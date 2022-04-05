use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("embed".into(), "Set the embed to be used when uploading.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("title".into(), "Title of the embed".into()))
    .option(StringBuilder::new("link".into(), "Link appended to your url.".into()))
    .option(StringBuilder::new("description".into(), "Description of the embed.".into()))
    .option(StringBuilder::new("color".into(), "Embed color to be used.".into()))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let command_args = cmd.data.options.iter();

  let title = get_arg(command_args.clone(), "title");
  let _link = get_arg(command_args.clone(), "link");
  let url = get_arg(command_args.clone(), "url");
  let description = get_arg(command_args.clone(), "description");
  let color = get_arg(command_args.clone(), "color");

  let embed = create_embed()
        .title("Updated the embed")
        .description("Your domain has been updated, Take a new screenshot to test the embed out.\n\n*please wait up to 2 minutes for your embed to update this is due to caching*")
        .build()?;

  set_embed::exec(user.id, description, title, url, color).await?;

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
