use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new(Lang::fallback().user_name().into(), Lang::fallback().user_desc().into(), CommandType::ChatInput)
    .option(UserBuilder::new("user".into(), "The user you want to view the profile of.".into()).required(true))
    .localize()
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let id = cmd.data.options.iter().find(|e| e.name == "user").unwrap();
  if let CommandOptionValue::User(user) = id.value {
    if let Ok(user) = get_user_discord::exec(user.to_string()).await {
      let images = get_user_image_count::exec(user.id).await?;

      let message = format!(
        "name: `{name}`\ndiscord_id: `{discord}`\nInvites: `{invites}`\n\ndomain: `{domain}`\nimages: `{images}`",
        name = user.name,
        invites = get_invite_count::exec(user.id).await?,
        discord = user.discord_id,
        domain = user.domain,
        images = images,
      );

      let embed = create_embed().title(cmd.lang().await?.user_title(&user.name)).description(message).build();

      return Ok(BotResponse::wembed(embed));
    }
  }
  let embed = create_embed().title(cmd.lang().await?.profile_name()).description(cmd.lang().await?.user_no_exist()).build();

  Ok(BotResponse::wembed(embed))
}
