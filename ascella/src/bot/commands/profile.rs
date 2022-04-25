use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("profile".into(), "View your profile.".into(), CommandType::ChatInput).build()
}

#[allow(clippy::or_fun_call)]

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
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
    Some(get_user_image_count::exec(user.id).await?),
    Some(user.upload_key.as_ref().unwrap_or(&new_key.unwrap_or("please wait 120 seconds".to_owned())).to_string()),
  );

  let embed = create_embed().title("User profile").description(message).build()?;

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
        flags: Some(MessageFlags::EPHEMERAL),
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
