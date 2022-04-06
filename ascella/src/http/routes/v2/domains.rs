use crate::prelude::*;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetDomainsResponse(Vec<Domain>);

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Domain {
  apex: bool,
  owner: i32,
  domain: String,
}

#[api_v2_operation(
  tags(Etc),
  summary = "get domains",
  description = "Returns all ascella domains",
  consumes = "application/json",
  produces = "application/json"
)]
#[get("/domains")]
pub async fn get() -> Result<GetDomainsResponse, Error> {
  let data = get_domains::exec().await.unwrap();

  let data_domains = data
    .into_iter()
    .map(|domain| Domain {
      apex: domain.apex,
      owner: domain.owner,
      domain: domain.domain,
    })
    .collect::<Vec<_>>();
  Ok(GetDomainsResponse(data_domains))
}
apply_responders!(GetDomainsResponse);
