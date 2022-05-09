use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new(Lang::fallback().delete_name().into(), Lang::fallback().delete_desc().into(), CommandType::ChatInput)
    .option(StringBuilder::new("id".into(), "Id of the image you want to delete use vanity if you don't know the id".into()))
    .option(StringBuilder::new("vanity".into(), "Vanity of the image".into()))
    .localize()
    .build()
}
pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let command_args = cmd.data.options.iter();

  let data = if let Some(val) = get_arg(command_args.clone(), "id") {
    let res = get_image_owner::exec(val.parse::<i32>().unwrap_or(0), user.id).await;

    if let Ok(res) = res {
      Some((res.id, res.vanity))
    } else {
      None
    }
  } else if let Some(val) = get_arg(command_args.clone(), "vanity") {
    let res = get_image_vanity::exec(val, user.id).await;

    if let Ok(res) = res {
      Some((res.id, res.vanity))
    } else {
      None
    }
  } else {
    None
  };
  let response = if let Some((id, vanity)) = data {
    delete_image::exec(id).await?;
    let embed = create_embed().title(user.lang().delete_image_title()).description(user.lang().delete_image_desc(id, vanity)).build();
    BotResponse::new().private().embed(embed)
  } else {
    BotResponse::new().private().content(user.lang().delete_image_not_found())
  };

  Ok(response)
}
