use crate::prelude::*;
pub fn command() -> Command {
  CommandBuilder::new("adddomain".into(), "Add a domain.".into(), CommandType::ChatInput)
    .option(UserBuilder::new("owner".into(), "The owner of the domain.".into()).required(true))
    .option(StringBuilder::new("domain".into(), "Domain to be added.".into()).required(true))
    .option(BooleanBuilder::new("apex".into(), "Weather or not the domain is a apex domain.".into()).required(true))
    .build()
}

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand, _user: Users) -> Result<()> {
  Ok(())
}
