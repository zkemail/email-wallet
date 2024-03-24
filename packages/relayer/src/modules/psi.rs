use crate::*;

use std::path::Path;

use axum::Json;
use ff::Field;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, remove_file};
use tokio::sync::mpsc::UnboundedSender;

const DELAY: u64 = 300;

pub enum UnclaimType {
    Fund(UnclaimedFund),
    State(UnclaimedState),
}

#[derive(Serialize, Deserialize)]
pub struct CheckRequest {
    pub point: Point,
    pub id: U256,
    pub is_fund: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RevealRequest {
    pub tx_hash: String,
    pub id: U256,
    pub is_fund: bool,
    pub randomness: String,
    pub email_address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: String,
    pub y: String,
}

pub struct PSIClient {
    pub tx_hash: String,
    pub point: Point,
    pub random: String,
    pub email_addr: String,
    pub id: U256,
    pub is_fund: bool,
    pub db: Arc<Database>,
    pub chain_client: Arc<ChainClient>,
}

impl PSIClient {
    pub async fn new(
        db: Arc<Database>,
        chain_client: Arc<ChainClient>,
        email_addr: String,
        tx_hash: String,
        id: U256,
        is_fund: bool,
    ) -> Result<Self> {
        let rng = rand::rngs::OsRng;
        let random = Fr::random(OsRng);
        let random = field2hex(&random);

        let point = psi_step1(CIRCUITS_DIR_PATH.get().unwrap(), &email_addr, &random).await?;

        Ok(Self {
            tx_hash,
            email_addr,
            id,
            is_fund,
            random,
            point,
            db,
            chain_client,
        })
    }

    pub async fn check_and_reveal(&self) -> Result<bool> {
        if let Some(account_key) = self.db.get_account_key(&self.email_addr).await? {
            if self
                .chain_client
                .check_if_account_created_by_account_key(&self.email_addr, &account_key)
                .await?
            {
                return Ok(false);
            }
        }
        let (created_relayers, inited_relayers) = self.find().await?;
        if inited_relayers.len() > 0 {
            self.reveal(inited_relayers).await?;
            Ok(false)
        } else {
            self.reveal(created_relayers).await?;
            Ok(true)
        }
    }

    pub async fn check(&self, address: &str) -> Result<(bool, bool)> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/serveCheck/", address))
            .json(&CheckRequest {
                point: self.point.clone(),
                id: self.id,
                is_fund: self.is_fund,
            })
            .send()
            .await?
            .error_for_status()?;

        let response_point = res.json::<Point>().await?;

        let result_point = psi_step3(
            CIRCUITS_DIR_PATH.get().unwrap(),
            response_point,
            &self.random,
        )
        .await?;

        let is_point_registered = self
            .chain_client
            .check_if_point_registered(result_point.clone())
            .await?;
        let is_account_created = self
            .chain_client
            .check_if_account_created_by_point(result_point)
            .await?;

        Ok((is_point_registered, is_account_created))
    }

    pub async fn find<'a>(&self) -> Result<(Vec<String>, Vec<String>)> {
        let subgraph_client = SubgraphClient::new();
        let relayers = subgraph_client.get_all_relayers_for_psi().await?;
        let mut created_hosts = vec![];
        let mut inited_hosts = vec![];

        for relayer in relayers {
            let (is_point_registered, is_account_created) = self.check(&relayer.1).await?;
            if is_account_created {
                inited_hosts.push(relayer.1.to_string());
            } else if is_point_registered {
                created_hosts.push(relayer.1.to_string());
            }
        }

        Ok((created_hosts, inited_hosts))
    }

    pub async fn reveal(&self, addresses: Vec<String>) -> Result<()> {
        let client = reqwest::Client::new();
        for address in addresses.into_iter() {
            let res = client
                .post(format!("{}/serveReveal/", address))
                .json(&RevealRequest {
                    tx_hash: self.tx_hash.clone(),
                    id: self.id,
                    email_address: self.email_addr.clone(),
                    randomness: self.random.clone(),
                    is_fund: self.is_fund,
                })
                .send()
                .await?
                .error_for_status()?;
        }

        Ok(())
    }
}

pub async fn serve_check_request(
    payload: CheckRequest,
    chain_client: Arc<ChainClient>,
) -> Result<Json<Point>> {
    check_unclaim_valid(Arc::clone(&chain_client), &payload.id, payload.is_fund).await?;

    let res = psi_step2(
        CIRCUITS_DIR_PATH.get().unwrap(),
        payload.point,
        RELAYER_RAND.get().unwrap(),
    )
    .await?;

    Ok(axum::response::Json(res))
}

pub async fn serve_reveal_request(
    payload: RevealRequest,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<String> {
    match check_unclaim_valid(Arc::clone(&chain_client), &payload.id, payload.is_fund).await? {
        UnclaimType::Fund(unclaimed_fund) => {
            // TODO: local check of recipient_commit = hash(random, email_addr)
            tx_claimer.send(Claim {
                tx_hash: payload.tx_hash,
                id: payload.id,
                email_address: payload.email_address.clone(),
                random: payload.randomness,
                commit: "0x".to_string() + &hex::encode(unclaimed_fund.email_addr_commit),
                expiry_time: unclaimed_fund.expiry_time.as_u64() as i64,
                is_fund: true,
                is_announced: false,
                is_seen: false,
            })?;
            Ok(format!(
                "Unclaimed fund for {} is accepted",
                payload.email_address
            ))
        }
        UnclaimType::State(unclaimed_state) => {
            // TODO: local check of recipient_commit = hash(random, email_addr)
            tx_claimer.send(Claim {
                tx_hash: payload.tx_hash,
                id: payload.id,
                email_address: payload.email_address.clone(),
                random: payload.randomness,
                commit: "0x".to_string() + &hex::encode(unclaimed_state.email_addr_commit),
                expiry_time: unclaimed_state.expiry_time.as_u64() as i64,
                is_fund: false,
                is_announced: false,
                is_seen: false,
            })?;
            Ok(format!(
                "Unclaimed state for {} is accepted",
                payload.email_address
            ))
        }
    }
}

pub async fn check_unclaim_valid(
    chain_client: Arc<ChainClient>,
    id: &U256,
    is_fund: bool,
) -> Result<UnclaimType> {
    let current_time = U256::from(now());
    let current_time_delayed = current_time + U256::from(DELAY);
    let unclaim = if is_fund {
        let fund = chain_client.query_unclaimed_fund(*id).await?;
        if fund.expiry_time < current_time_delayed {
            bail!("Unclaimed fund is expired");
        }
        UnclaimType::Fund(fund)
    } else {
        let state = chain_client.query_unclaimed_state(*id).await?;
        if state.expiry_time < current_time_delayed {
            bail!("Unclaimed state is expired");
        }
        UnclaimType::State(state)
    };

    Ok(unclaim)
}

pub async fn psi_step1(
    circuits_dir_path: &Path,
    email_addr: &str,
    client_rand: &str,
) -> Result<Point> {
    compute_psi_point(circuits_dir_path, email_addr, client_rand).await
}

pub async fn psi_step2(
    circuits_dir_path: &Path,
    point: Point,
    relayer_rand: &str,
) -> Result<Point> {
    let input_file_name = calculate_default_hash(&point.x);
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(input_file_name + ".json");

    let command_str = format!(
        "--cwd {} psi-step2 --x {} --y {} --relayer-rand {} --output {}",
        circuits_dir_path.to_str().unwrap(),
        point.x,
        point.y,
        relayer_rand,
        input_file_name.to_str().unwrap()
    );

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .spawn()?;

    let status = proc.wait().await?;
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;
    remove_file(input_file_name).await?;

    let point: Point = serde_json::from_str(&result)?;

    Ok(point)
}

pub async fn psi_step3(circuits_dir_path: &Path, point: Point, client_rand: &str) -> Result<Point> {
    let input_file_name = calculate_default_hash(&point.x);
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(input_file_name + ".json");

    let command_str = format!(
        "--cwd {} psi-step3 --x {} --y {} --client-rand {} --output {}",
        circuits_dir_path.to_str().unwrap(),
        point.x,
        point.y,
        client_rand,
        input_file_name.to_str().unwrap()
    );

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .spawn()?;

    let status = proc.wait().await?;
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;
    remove_file(input_file_name).await?;

    let point: Point = serde_json::from_str(&result)?;

    Ok(point)
}
