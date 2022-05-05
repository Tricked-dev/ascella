use crate::prelude::*;
use twilight_model::application::interaction::application_command::CommandOptionValue;
pub fn command() -> Command {
  CommandBuilder::new("autodelete".into(), "auto delete images older than a certain amount of days.".into(), CommandType::ChatInput)
    .option(IntegerBuilder::new("days".into(), "days after which the image will get deleted".into()))
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
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
    create_embed().title(user.lang().auto_delete_title()).description(user.lang().auto_delete_desc(v)).build()
  } else {
    create_embed()
      .title(user.lang().auto_delete_title())
      .description(user.lang().auto_delete_out_of_range(v.err().unwrap()))
      .build()
  };

  Ok(BotResponse::new().embed(embed))
}
