use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("help".into(), Lang::En.help_desc().into(), CommandType::ChatInput).localize().build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let commands = get_commands(vec![]);
  let user_lang = cmd.lang().await?;
  let desc = commands
    .iter()
    .map(|x| {
      format!(
        "**{}**: {}",
        &x.name,
        if x.description_localizations.is_some() {
          user_lang.from_str(&format!("{}_desc", &x.name)).unwrap().to_string()
        } else {
          x.description.clone()
        }
      )
    })
    .collect::<Vec<String>>()
    .join("\n");

  Ok(BotResponse::new().content(desc))
}
