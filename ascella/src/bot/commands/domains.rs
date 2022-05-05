use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("domains".into(), "View the domains ascella has.".into(), CommandType::ChatInput).build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let data = get_domains::exec().await?;

  // data_embed

  let msg = future::join_all(data.into_iter().map(|e| async move {
    let (id, name) = get_discord_id::exec(e.owner).await.unwrap();

    format!("**{domain}**\nApex: {apex}\nOwner: <@{mention}>{id}", domain = e.domain, apex = e.apex, mention = id, id = name)
  }))
  .await;
  let mut message: Vec<String> = vec![];
  for body in msg {
    message.push(body.to_string());
  }

  let embed = create_embed().title("Embed").description(message.join("\n")).build();

  Ok(BotResponse::new().embed(embed))
}
