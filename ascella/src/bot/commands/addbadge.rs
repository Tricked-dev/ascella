use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("addbadge".into(), "Add a badge to a user.".into(), CommandType::ChatInput)
    .option(UserBuilder::new("user".into(), "The user you want to add a badge to.".into()).required(true))
    .option(
      IntegerBuilder::new("badge".into(), "The badge to give".to_owned())
        .choices(vec![
          ("Sponsor".to_owned(), BadgeFlags::SPONSOR.bits() as i64),
          ("Translator".to_owned(), BadgeFlags::TRANSLATOR.bits() as i64),
          ("OG".to_owned(), BadgeFlags::OG.bits() as i64),
          ("Contributor".to_owned(), BadgeFlags::CONTRIBUTOR.bits() as i64),
        ])
        .required(true),
    )
    .build()
}
#[test]
fn test_command() {
  println!("{:?}", command().options)
}
pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let mut cmd_iter = cmd.data.options.iter();
  let badge = get_arg_int(cmd_iter.clone(), "badge").unwrap();
  let id = cmd_iter.find(|e| e.name == "user").unwrap();

  if let CommandOptionValue::User(user) = id.value {
    if let Ok(mut user) = get_user_discord::exec(user.to_string()).await {
      let new_bits = BadgeFlags::from_bits(user.flags as u32).unwrap() | BadgeFlags::from_bits(badge as u32).unwrap();
      set_user_flags::exec(user.id, new_bits.bits() as i32).await?;
      user.flags = new_bits.bits() as i32;
      let flags = get_flags_str(&user);
      let embed = create_embed().title(cmd.lang().await?.user_title(&user.name)).description(format!("Badge added {flags}")).build();

      return Ok(BotResponse::wembed(embed));
    }
  }
  let embed = create_embed().title(user.lang().profile_name()).description(user.lang().user_no_exist()).build();

  Ok(BotResponse::wembed(embed))
}
