use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new(Lang::fallback().delete_latest_name().into(), Lang::fallback().delete_latest_desc().into(), CommandType::ChatInput)
    .localize()
    .build()
}

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let img = get_latest_image::exec(user.id).await?;
  delete_image::exec(img.id).await?;

  let embed = create_embed()
    .title(user.lang().delete_image_title())
    .description(user.lang().delete_image_desc(&img.id, img.vanity))
    .build();

  Ok(BotResponse::new().private().embed(embed))
}
