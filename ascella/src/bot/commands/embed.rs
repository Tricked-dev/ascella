use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("embed".into(), "Set the embed to be used when uploading.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("title".into(), "Title of the embed".into()))
    .option(StringBuilder::new("link".into(), "Link appended to your url.".into()))
    .option(StringBuilder::new("author".into(), "Set the author of the embed".into()))
    .option(StringBuilder::new("description".into(), "Description of the embed.".into()))
    .option(StringBuilder::new("color".into(), "Embed color to be used.".into()))
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let command_args = cmd.data.options.iter();

  let title = get_arg(command_args.clone(), "title");
  let _link = get_arg(command_args.clone(), "link");
  let url = get_arg(command_args.clone(), "url");
  let description = get_arg(command_args.clone(), "description");
  let color = get_arg(command_args.clone(), "color");
  let author = get_arg(command_args.clone(), "author");

  let embed = create_embed()
    .title(user.lang().embed_update_title())
    .description(user.lang().embed_update_desc())
    .build();

  set_embed::exec(user.id, description, title, url, color, author).await?;
  Ok(BotResponse::new().embed(embed))
}
