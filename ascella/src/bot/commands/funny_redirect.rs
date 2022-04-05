use crate::prelude::*;
lazy_static! {
  static ref FUNNY_WORDS: Vec<&'static str> = vec![
    "fbi",
    "vault",
    "scam",
    "bit",
    "coin",
    "hack",
    "hacker",
    "56",
    "420",
    "69",
    "aa",
    "ja",
    "ha",
    "du",
    "ko",
    "kl",
    "s1",
    "sdf",
    "$",
    "!!",
    "[",
    "869",
    "gov",
    "hacking",
    "steal",
    "st0k",
    "exe",
    "dmg",
    "download",
    "free",
    "download-here",
    "sale",
    ".",
    "..",
    "ransomware",
    "bitcoin",
    "monaro",
    "etherium",
    "miner",
    "bitcoinminer",
    "fast",
    "here",
    "h@cks",
    "d0wnl0ad",
    "fr33",
    "ch3ap",
    "h3r3",
    "f0r-fr33"
  ];
}

pub fn command(domain_options: impl IntoIterator<Item = (String, String)>) -> Command {
  CommandBuilder::new("funny_redirect".into(), "Creates a very long funny redirect".into(), CommandType::ChatInput)
    .option(StringBuilder::new("url".into(), "Url you want to redirect to.".into()).required(true))
    .option(
      StringBuilder::new("domain".into(), "Domain to be used.".into())
        .required(true)
        .choices(domain_options),
    )
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<()> {
  let command_args = cmd.data.options.iter();

  let url = get_arg_default(command_args.clone(), "url", "https//tricked.pro/aethor");
  let domain = get_arg_default(command_args.clone(), "domain", "https//i.tricked.pro");
  let mut clone = FUNNY_WORDS.clone();

  clone.shuffle(&mut rand::thread_rng());

  let vanity = clone.into_iter().take(rand::thread_rng().gen_range(4..20)).collect::<Vec<&str>>().join("");

  let data = format!("https://{}/{}", domain, vanity);
  println!("{}", data);
  let embed = create_embed()
    .title("Successfully created the redirect")
    .url(&data)
    .description(format!("Made a fancy vanity {}", &data))
    .build()?;
  create_redirect::exec(user.id, url, vanity).await?;

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
        content: Some(data),
        embeds: Some(vec![embed]),
        flags: None,
        tts: Some(false),
      }),
    )
    .exec()
    .await?;

  Ok(())
}
