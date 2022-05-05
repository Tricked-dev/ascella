use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("user".into(), "View the profile of a user.".into(), CommandType::ChatInput)
    .option(UserBuilder::new("user".into(), "The user you want to view the profile of.".into()).required(true))
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let id = cmd.data.options.iter().find(|e| e.name == "user").unwrap();
  if let CommandOptionValue::User(user) = id.value {
    if let Ok(user) = get_user_discord::exec(user.to_string()).await {
      let images = get_user_image_count::exec(user.id).await?;

      let message = format!(
        "name: `{name}`\ndiscord_id: `{discord}`\n\ndomain: `{domain}`\nimages: `{images}`",
        name = user.name,
        discord = user.discord_id,
        domain = user.domain,
        images = images,
      );

      let embed = create_embed().title(format!("Profile of {}", &user.name)).description(message).build();

      return Ok(BotResponse::new().embed(embed));
    }
  }
  let embed = create_embed().title("User profile").description("User isn't a registered user").build();

  Ok(BotResponse::new().embed(embed))
}
