use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("redeem".into(), "Redeem a code.".into(), CommandType::ChatInput)
    .option(StringBuilder::new("code".into(), "Code your looking to redeem.".into()).required(true))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
  let code = get_arg_default(cmd.data.options.iter(), "code", "none");
  let discord_user = cmd.member.as_ref().unwrap().user.as_ref().unwrap();
  let id = discord_user.id.to_string();
  let data = get_user_discord::exec(id.clone()).await;

  if data.is_err() {
    //Ownership is aids please don't hate me
    let code_data = get_unclaimed_code::exec(code.clone()).await;

    if code_data.is_ok() {
      client
        .add_guild_member_role(cmd.guild_id.unwrap(), discord_user.id, RoleId(core::num::NonZeroU64::new(878730206024720404).unwrap()))
        .exec()
        .await?;
      let name = discord_user.name.clone();
      let user = create_user::exec("https://ascella.host", id, ran_str(7), name, ulid::Ulid::new().to_string()).await?;
      claim_code::exec(code, &user.id).await?;

      let message = format!(
        r"dashboard: https://dash.ascella.host
            id: `{id}`
            name: `{name}`
            discord_id: `{discord}`
            password: `{pass}`
            
            domain: `{domain}`
            images: `0`
            
            download config [here](https://ascella.wtf/v2/ascella/config?auth={upload_key})
            ```json
            {config}
            ```",
        id = user.id,
        name = user.name,
        discord = user.discord_id,
        pass = user.key,
        domain = user.domain,
        config = serde_json::to_string_pretty(&create_config(&user.upload_key.as_ref().unwrap())).unwrap(),
        upload_key = user.upload_key.as_ref().unwrap()
      );

      let embed = create_embed().title("Code redeemed!").description(message).build()?;

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
    } else {
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
            content: Some(String::from("Invalid code")),
            embeds: Some(vec![]),
            flags: Some(MessageFlags::EPHEMERAL),
            tts: Some(false),
          }),
        )
        .exec()
        .await?;
    }
  } else {
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
          content: Some(String::from("Your already a image uploader user keep staying awesome")),
          embeds: Some(vec![]),
          flags: Some(MessageFlags::EPHEMERAL),
          tts: Some(false),
        }),
      )
      .exec()
      .await?;
  }

  Ok(())
}
