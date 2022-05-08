use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("help".into(), "Ascella bot help command.".into(), CommandType::ChatInput).localize().build()
}

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand) -> Result<BotResponse> {
  let commands = get_commands(vec![]);
  let desc = commands.iter().map(|x| format!("**{}**: {}", &x.name, &x.description)).collect::<Vec<String>>().join("\n");

  Ok(BotResponse::new().content(desc))
}
