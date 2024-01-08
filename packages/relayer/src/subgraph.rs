use crate::*;

use std::str::FromStr;

use ::reqwest;

use ethers::utils::to_checksum;
use graphql_client::reqwest::post_graphql;

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub(crate) struct GetRelayers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub(crate) struct AllRelayersForPSI;

pub struct SubgraphClient {
    pub(crate) web_client: reqwest::Client,
    pub(crate) subgraph_api: String,
}

impl SubgraphClient {
    pub(crate) fn new() -> Self {
        Self {
            web_client: reqwest::Client::new(),
            subgraph_api: SUBGRAPH_URL.get().unwrap().clone(),
        }
    }

    #[named]
    pub(crate) async fn get_relayers_by_wallet_addr(
        &self,
        wallet_addr: &Address,
    ) -> Result<Vec<(Address, [u8; 32], String)>> {
        let variables = get_relayers::Variables {
            wallet_addr: Some(to_checksum(wallet_addr, CHAIN_ID.get().map(|v| *v as u8))),
        };
        let response_body =
            post_graphql::<GetRelayers, _>(&self.web_client, &self.subgraph_api, variables).await?;
        info!(LOG, "{:?}", response_body; "func" => function_name!());
        if response_body.data.is_none() {
            return Ok(vec![]);
        }
        let response_data: get_relayers::ResponseData = response_body.data.unwrap();
        let relayer_accounts = response_data
            .account
            .expect("no account in response_data")
            .relayer_accounts;
        let mut relayer_infos = vec![];
        for relayer_account in relayer_accounts {
            relayer_infos.push((
                Address::from_str(&relayer_account.relayer.address)?,
                u256_to_bytes32(&hex_to_u256(&relayer_account.relayer.rand_hash)?),
                relayer_account.relayer.hostname,
            ));
        }
        Ok(relayer_infos)
    }

    #[named]
    pub(crate) async fn get_all_relayers_for_psi(&self) -> Result<Vec<(Address, String)>> {
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
