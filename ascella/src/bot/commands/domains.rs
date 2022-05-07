use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("domains".into(), "View the domains ascella has.".into(), CommandType::ChatInput).build()
}

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  let data = get_domains::exec().await?;
  let msg = future::join_all(data.into_iter().map(|e| async move {
    let (id, name) = get_discord_id::exec(e.owner).await.unwrap();
    // format!("**{domain}**\nApex: {apex}\nOwner: <@{id}>{name}", domain = e.domain, apex = e.apex)
    cmd.lang().await.unwrap().domains_list(e.apex, e.domain, &id, &name)
  }))
  .await;
  let mut message: Vec<String> = vec![];
  for body in msg {
    message.push(body.to_string());
  }

  let embed = create_embed().title(cmd.lang().await?.domains()).description(message.join("\n")).build();

  Ok(BotResponse::new().embed(embed))
}
