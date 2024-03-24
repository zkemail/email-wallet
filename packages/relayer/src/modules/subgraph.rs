use crate::*;

use std::str::FromStr;

use ::reqwest;

use graphql_client::reqwest::post_graphql;

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct AllRelayersForPSI;

pub struct SubgraphClient {
    pub web_client: reqwest::Client,
    pub subgraph_api: String,
}

impl SubgraphClient {
    pub fn new() -> Self {
        Self {
            web_client: reqwest::Client::new(),
            subgraph_api: SUBGRAPH_URL.get().unwrap().clone(),
        }
    }

    #[named]
    pub async fn get_all_relayers_for_psi(&self) -> Result<Vec<(Address, String)>> {
        let variables = all_relayers_for_psi::Variables {};
        let response_body =
            post_graphql::<AllRelayersForPSI, _>(&self.web_client, &self.subgraph_api, variables)
                .await?;
        info!(LOG, "{:?}", response_body; "func" => function_name!());
        match response_body.data {
            Some(response_data) => {
                let relayers = response_data.relayers.expect("no relayer found");
                let mut relayer_infos = vec![];
                for relayer in relayers.into_iter() {
                    relayer_infos.push((Address::from_str(&relayer.address)?, relayer.hostname));
                }
                Ok(relayer_infos)
            }
            None => Ok(vec![]),
        }
    }
}
