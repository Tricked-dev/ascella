use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("delete_latest".into(), "Delete your latest upload".into(), CommandType::ChatInput).build()
}

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let img = get_latest_image::exec(user.id).await?;
  delete_image::exec(img.id).await?;

  let embed = create_embed()
    .title("Deleted your latest image ;)")
    .description(format!("Deleted image {vanity} with id {id}", id = &img.id, vanity = &img.vanity))
    .build();

  Ok(BotResponse::new().private().embed(embed))
}
