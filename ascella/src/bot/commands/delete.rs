use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("delete".into(), "Delete a image".into(), CommandType::ChatInput)
    .option(StringBuilder::new("id".into(), "Id of the image you want to delete use vanity if you don't know the id".into()))
    .option(StringBuilder::new("vanity".into(), "Vanity of the image".into()))
    .build()
}
pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
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
    let embed = create_embed()
      .title("Deleted your image image ;)")
      .description(format!("Deleted image {vanity} with id {id}", id = id, vanity = vanity))
      .build();
    BotResponse::new().private().embed(embed)
  } else {
    BotResponse::new().private().content("Could not find specified image")
  };

  Ok(response)
}
