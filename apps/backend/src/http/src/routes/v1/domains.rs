use crate::queries::get_domains;
use salvo::prelude::*;
use serde_json::json;

#[fn_handler]
pub async fn get(_req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
  let data = get_domains::exec().await.unwrap();

  let data_domains = data
    .iter()
    .map(|domain| {
      json!({
          "apex":domain.apex,
          "owner": domain.owner,
          "domain": domain.domain
      })
    })
    .collect::<Vec<_>>();
  res.render_json(&data_domains);
  Ok(())
}
