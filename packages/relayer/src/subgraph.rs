use crate::*;

use std::path::Path;
use std::str::FromStr;

use ::reqwest;
use axum::Json;
use ethers::utils::to_checksum;
use graphql_client::reqwest::post_graphql;
use graphql_client::*;
use graphql_client::{GraphQLQuery, Response};
use num_bigint::RandBigInt;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::fs::{read_to_string, remove_file};
use tokio::sync::mpsc::UnboundedSender;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetRelayers;

pub struct SubgraphClient {
    pub(crate) web_client: reqwest::Client,
    pub(crate) subgraph_api: String,
}

impl SubgraphClient {
    pub fn new() -> Self {
        Self {
            web_client: reqwest::Client::new(),
            subgraph_api: SUBGRAPH_URL.get().unwrap().clone(),
        }
    }

    pub async fn get_relayers_by_wallet_addr(
        &self,
        wallet_addr: &Address,
    ) -> Result<Vec<(Address, [u8; 32])>> {
        let variables = get_relayers::Variables {
            wallet_addr: Some(to_checksum(wallet_addr, CHAIN_ID.get().map(|v| *v as u8))),
        };
        let response_body =
            post_graphql::<GetRelayers, _>(&self.web_client, &self.subgraph_api, variables).await?;
        info!("{:?}", response_body);
        let response_data: get_relayers::ResponseData =
            response_body.data.expect("missing response data");
        let relayer_accounts = response_data
            .account
            .expect("no account in response_data")
            .relayer_accounts;
        let mut relayer_infos = vec![];
        for relayer_account in relayer_accounts {
            relayer_infos.push((
                Address::from_str(&relayer_account.relayer.address)?,
                u256_to_bytes32(&hex_to_u256(&relayer_account.relayer.rand_hash)?),
            ));
        }
        Ok(relayer_infos)
    }
}

// const DELAY: u64 = 300;

// pub(crate) struct SubgraphClient {
//     pub(crate) web_client: reqwest::Client,
//     pub(crate) subgraph_base: String,
// }

// impl SubgraphClient {
//     pub(crate) fn new() -> Self {
//         Self {
//             web_client: reqwest::Client::new(),
//             subgraph_base: SUBGRAPH_URL.get().unwrap().clone(),
//         }
//     }

//     pub(crate) fn get_relayer_by_wallet_addr(&self, wallet_addr: &Address) -> Address {}
// }

// impl PSIClient {
//     pub(crate) async fn new(
//         chain_client: Arc<ChainClient>,
//         email_addr: String,
//         id: U256,
//         is_fund: bool,
//     ) -> Result<Self> {
//         let mut rng = rand::rngs::OsRng;
//         let random = rng.gen_biguint(253);
//         let random = Fr::from_bytes(&random.to_bytes_le().try_into().unwrap()).unwrap();
//         let random = field2hex(&random);

//         let point = psi_step1(CIRCUITS_DIR_PATH.get().unwrap(), &email_addr, &random).await?;

//         Ok(Self {
//             email_addr,
//             id,
//             is_fund,
//             random,
//             point,
//             chain_client,
//         })
//     }

//     pub(crate) async fn check(&self, client: reqwest::Client, address: &str) -> Result<bool> {
//         let res = client
//             .post(format!("{}/serveCheck/", address))
//             .json(&CheckRequest {
//                 point: self.point.clone(),
//                 id: self.id,
//                 is_fund: self.is_fund,
//             })
//             .send()
//             .await?
//             .error_for_status()?;

//         let response_point = res.json::<Point>().await?;

//         let result_point = psi_step3(
//             CIRCUITS_DIR_PATH.get().unwrap(),
//             response_point,
//             &self.random,
//         )
//         .await?;

//         Ok(self
//             .chain_client
//             .check_if_point_registered(result_point.clone())
//             .await?
//             && self
//                 .chain_client
//                 .check_if_account_initialized_by_point(result_point)
//                 .await?)
//     }

//     pub(crate) async fn find<'a>(&self) -> Result<Vec<String>> {
//         let client = reqwest::Client::new();
//         let relayers = self.chain_client.get_relayers().await?;
//         let mut result = vec![];

//         for relayer in relayers {
//             if self.check(client.clone(), &relayer).await? {
//                 result.push(relayer);
//             }
//         }

//         Ok(result)
//     }

//     pub(crate) async fn reveal(&self, addresses: &[&str]) -> Result<()> {
//         let client = reqwest::Client::new();
//         for &address in addresses {
//             let res = client
//                 .post(format!("{}/serveReveal/", address))
//                 .json(&RevealRequest {
//                     id: self.id,
//                     email_address: self.email_addr.clone(),
//                     randomness: self.random.clone(),
//                     is_fund: self.is_fund,
//                 })
//                 .send()
//                 .await?
//                 .error_for_status()?;
//         }

//         Ok(())
//     }
// }

// pub(crate) async fn serve_check_request(
//     payload: CheckRequest,
//     chain_client: Arc<ChainClient>,
// ) -> Result<Json<Point>> {
//     check_unclaim_valid(Arc::clone(&chain_client), &payload.id, payload.is_fund).await?;

//     let res = psi_step2(
//         CIRCUITS_DIR_PATH.get().unwrap(),
//         payload.point,
//         RELAYER_RAND.get().unwrap(),
//     )
//     .await?;

//     Ok(axum::response::Json(res))
// }

// pub(crate) async fn serve_reveal_request(
//     payload: RevealRequest,
//     chain_client: Arc<ChainClient>,
//     tx_claimer: UnboundedSender<Claim>,
// ) -> Result<String> {
//     match check_unclaim_valid(Arc::clone(&chain_client), &payload.id, payload.is_fund).await? {
//         UnclaimType::Fund(unclaimed_fund) => {
//             // TODO: local check of recipient_commit = hash(random, email_addr)
//             tx_claimer.send(Claim {
//                 id: payload.id,
//                 email_address: payload.email_address.clone(),
//                 random: payload.randomness,
//                 commit: "0x".to_string() + &hex::encode(unclaimed_fund.email_addr_commit),
//                 expiry_time: unclaimed_fund.expiry_time.as_u64() as i64,
//                 is_fund: true,
//                 is_announced: false,
//             })?;
//             Ok(format!(
//                 "Unclaimed fund for {} is accepted",
//                 payload.email_address
//             ))
//         }
//         UnclaimType::State(unclaimed_state) => {
//             // TODO: local check of recipient_commit = hash(random, email_addr)
//             tx_claimer.send(Claim {
//                 id: payload.id,
//                 email_address: payload.email_address.clone(),
//                 random: payload.randomness,
//                 commit: "0x".to_string() + &hex::encode(unclaimed_state.email_addr_commit),
//                 expiry_time: unclaimed_state.expiry_time.as_u64() as i64,
//                 is_fund: false,
//                 is_announced: false,
//             })?;
//             Ok(format!(
//                 "Unclaimed state for {} is accepted",
//                 payload.email_address
//             ))
//         }
//     }
// }

// pub(crate) async fn check_unclaim_valid(
//     chain_client: Arc<ChainClient>,
//     id: &U256,
//     is_fund: bool,
// ) -> Result<UnclaimType> {
//     let current_time = U256::from(now());
//     let current_time_delayed = current_time + U256::from(DELAY);
//     let unclaim = if is_fund {
//         let fund = chain_client.query_unclaimed_fund(*id).await?;
//         if fund.expiry_time < current_time_delayed {
//             bail!("Unclaimed fund is expired");
//         }
//         UnclaimType::Fund(fund)
//     } else {
//         let state = chain_client.query_unclaimed_state(*id).await?;
//         if state.expiry_time < current_time_delayed {
//             bail!("Unclaimed state is expired");
//         }
//         UnclaimType::State(state)
//     };

//     Ok(unclaim)
// }

// pub(crate) async fn psi_step1(
//     circuits_dir_path: &Path,
//     email_addr: &str,
//     client_rand: &str,
// ) -> Result<Point> {
//     let input_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(email_addr.to_string() + "psi" + ".json");

//     let command_str = format!(
//         "--cwd {} psi-step1 --email-addr {} --client-rand {} --output {}",
//         circuits_dir_path.to_str().unwrap(),
//         email_addr,
//         client_rand,
//         input_file_name.to_str().unwrap()
//     );

//     let mut proc = tokio::process::Command::new("yarn")
//         .args(command_str.split_whitespace())
//         .spawn()?;

//     let status = proc.wait().await?;
//     assert!(status.success());

//     let result = read_to_string(&input_file_name).await?;
//     remove_file(input_file_name).await?;

//     let point: Point = serde_json::from_str(&result)?;

//     Ok(point)
// }

// pub(crate) async fn psi_step2(
//     circuits_dir_path: &Path,
//     point: Point,
//     relayer_rand: &str,
// ) -> Result<Point> {
//     let input_file_name = calculate_default_hash(&point.x);
//     let input_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(input_file_name + ".json");

//     let command_str = format!(
//         "--cwd {} psi-step2 --x {} --y {} --relayer-rand {} --output {}",
//         circuits_dir_path.to_str().unwrap(),
//         point.x,
//         point.y,
//         relayer_rand,
//         input_file_name.to_str().unwrap()
//     );

//     let mut proc = tokio::process::Command::new("yarn")
//         .args(command_str.split_whitespace())
//         .spawn()?;

//     let status = proc.wait().await?;
//     assert!(status.success());

//     let result = read_to_string(&input_file_name).await?;
//     remove_file(input_file_name).await?;

//     let point: Point = serde_json::from_str(&result)?;

//     Ok(point)
// }

// pub(crate) async fn psi_step3(
//     circuits_dir_path: &Path,
//     point: Point,
//     client_rand: &str,
// ) -> Result<Point> {
//     let input_file_name = calculate_default_hash(&point.x);
//     let input_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(input_file_name + ".json");

//     let command_str = format!(
//         "--cwd {} psi-step3 --x {} --y {} --client-rand {} --output {}",
//         circuits_dir_path.to_str().unwrap(),
//         point.x,
//         point.y,
//         client_rand,
//         input_file_name.to_str().unwrap()
//     );

//     let mut proc = tokio::process::Command::new("yarn")
//         .args(command_str.split_whitespace())
//         .spawn()?;

//     let status = proc.wait().await?;
//     assert!(status.success());

//     let result = read_to_string(&input_file_name).await?;
//     remove_file(input_file_name).await?;

//     let point: Point = serde_json::from_str(&result)?;

//     Ok(point)
// }
