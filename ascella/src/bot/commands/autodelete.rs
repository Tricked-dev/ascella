use crate::prelude::*;
use twilight_model::application::interaction::application_command::CommandOptionValue;
pub fn command() -> Command {
  CommandBuilder::new("autodelete".into(), "auto delete images older than a certain amount of days.".into(), CommandType::ChatInput)
    .option(IntegerBuilder::new("days".into(), "days after which the image will get deleted".into()))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  // let command_args = cmd.data.options[0];

  let v = if let CommandOptionValue::Integer(v) = cmd.data.options[0].value {
    match v {
      1..=365 => Ok(v),
      _ => Err(v),
    }
  } else {
    unreachable!()
  };
  let embed = if let Ok(v) = v {
    set_autodelete::exec(user.id, v.try_into().unwrap()).await?;
    create_embed()
      .title("Auto image deletion")
      .description(format!("Your images will now be automatically deleted after {} days", v))
      .build()?
  } else {
    create_embed()
      .title("Auto image deletion")
      .description(format!("{} is not a valid amount of days please choose a time between 1 and 365 days", v.err().unwrap()))
      .build()?
  };

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
