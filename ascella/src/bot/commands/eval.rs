use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("eval".into(), "Eval postgresql queries".into(), CommandType::ChatInput)
    .option(StringBuilder::new("code".into(), "Postgresql Query".into()).required(true))
    .build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand, user: Users) -> Result<BotResponse> {
  let code = get_arg(cmd.data.options.iter(), "code");
  let res = get_tokio_postgres().await.query(&code.unwrap(), &[]).await?;

  Ok(BotResponse::wcontent(format!("{}\n```js\n{:?}\n```", user.name, res.iter().map(|row| { row.columns() }))).private())
}
