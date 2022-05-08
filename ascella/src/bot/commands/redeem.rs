use twilight_http::request::AuditLogReason;

use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("redeem".into(), Lang::En.redeem_desc().into(), CommandType::ChatInput)
    .option(StringBuilder::new("code".into(), "Code your looking to redeem.".into()).required(true))
    .localize()
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let code = get_arg_default(cmd.data.options.iter(), "code", "none");
  let discord_user = cmd.member.as_ref().unwrap().user.as_ref().unwrap();
  let id = discord_user.id.to_string();
  let data = get_user_discord::exec(id.clone()).await;

  let response = if data.is_err() {
    //Ownership is aids please don't hate me
    let code_data = get_user_code::exec(code.clone()).await;

    if let Ok(user) = code_data {
      client
        .add_guild_member_role(cmd.guild_id.unwrap(), discord_user.id, Id::new(878730206024720404))
        .reason("Became a ascella user")?
        .exec()
        .await?;
      let name = discord_user.name.clone();
      let user = create_user::exec("https://ascella.host", id, ran_str(7), name, ulid::Ulid::new().to_string(), ran_str(7), user.id).await?;

      let message = format_profile(&user, 0, 0, None);

      let embed = create_embed().title(cmd.lang().await?.redeem_claimed()).description(message).build();

      BotResponse::new().embed(embed).private()
    } else {
      BotResponse::new().content(cmd.lang().await?.redeem_invalid()).private()
    }
  } else if let Ok(user) = data {
    BotResponse::new().content(user.lang().redeem_exists()).private()
  } else {
    BotResponse::new().content(cmd.lang().await?.redeem_error()).private()
  };

  Ok(response)
}
