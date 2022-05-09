use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new(Lang::fallback().profile_name().into(), Lang::fallback().profile_desc().into(), CommandType::ChatInput)
    .localize()
    .build()
}

#[allow(clippy::or_fun_call)]

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let new_key = if user.upload_key.is_none() {
    let key = ulid::Ulid::new().to_string();
    set_upload_key::exec(user.id, &key).await?;
    Some(key)
  } else {
    None
  };

  if user.invite_code.is_none() {
    let key = ran_str(6);
    set_invite_code::exec(user.id, &key).await?;
  };

  let message = format_profile(
    &user,
    get_invite_count::exec(user.id).await?,
    get_user_image_count::exec(user.id).await?,
    Some(user.upload_key.as_ref().unwrap_or(&new_key.unwrap_or("please wait 120 seconds".to_owned())).to_string()),
  );

  let embed = create_embed().title(user.lang().profile_embed_name()).description(message).build();
  Ok(BotResponse::new().embed(embed).private())
}
