use crate::*;

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use num_bigint::RandBigInt;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, remove_file};
use tokio::sync::mpsc::UnboundedSender;

const DELAY: u64 = 300;

pub(crate) enum UnclaimType {
    Fund {
        email_addr_commit: [u8; 32],
        sender: Address,
        token_addr: Address,
        amount: U256,
        expiry_time: U256,
    },
    State {
        email_addr_commit: [u8; 32],
        sender: Address,
        extension_addr: Address,
        state: Bytes,
        expiry_time: U256,
    },
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CheckRequest {
    pub(crate) point: Point,
    pub(crate) recipient_commitment: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RevealRequest {
    pub(crate) randomness: String,
    pub(crate) recipient_commitment: String,
    pub(crate) email_address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Point {
    pub(crate) x: String,
    pub(crate) y: String,
}

struct PSIClient {
    pub(crate) point: Point,
    pub(crate) random: String,
    pub(crate) email_addr: String,
    pub(crate) recipient_commitment: String,
    pub(crate) chain_client: Arc<ChainClient>,
}

impl PSIClient {
    pub(crate) async fn new(
        chain_client: Arc<ChainClient>,
        email_addr: &str,
        recipient_commitment: String,
    ) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let random = rng.gen_biguint(253);
        let random = Fr::from_bytes(&random.to_bytes_le().try_into().unwrap()).unwrap();
        let random = field2hex(&random);

        let point = psi_step1(CIRCUITS_DIR_PATH.get().unwrap(), email_addr, &random).await?;

        Ok(Self {
            email_addr: email_addr.to_string(),
            recipient_commitment,
            random,
            point,
            chain_client,
        })
    }

    pub(crate) async fn check(&self, client: reqwest::Client, address: &str) -> Result<bool> {
        let res = client
            .post(format!("{}/serveCheck/", address))
            .json(&serde_json::json!({ "point": self.point.clone(), "tx_hash": &self.recipient_commitment }))
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

        self.chain_client
            .check_if_point_registered(result_point)
            .await
    }

    pub(crate) async fn find<'a>(&self) -> Result<Option<String>> {
        let client = reqwest::Client::new();
        let relayers = self.chain_client.get_relayers().await?;

        for relayer in relayers {
            if self.check(client.clone(), &relayer).await? {
                return Ok(Some(relayer));
            }
        }

        Ok(None)
    }

    pub(crate) async fn reveal(
        address: &str,
        randomness: &str,
        recipient_commitment: &str,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/serveReveal/", address))
            .json(&serde_json::json!({ "randomness": randomness, "recipient_commitment": recipient_commitment }))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

pub(crate) async fn serve_check_request(
    chain_client: Arc<ChainClient>,
    payload: CheckRequest,
) -> Result<Json<Point>> {
    check_unclaim_valid(Arc::clone(&chain_client), &payload.recipient_commitment).await?;

    let res = psi_step2(
        CIRCUITS_DIR_PATH.get().unwrap(),
        payload.point,
        RELAYER_RAND.get().unwrap(),
    )
    .await?;

    Ok(axum::response::Json(res))
}

pub(crate) async fn serve_reveal_request(
    payload: RevealRequest,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<String> {
    match check_unclaim_valid(Arc::clone(&chain_client), &payload.recipient_commitment).await? {
        UnclaimType::Fund {
            email_addr_commit,
            sender,
            token_addr,
            amount,
            expiry_time,
        } => {
            tx_claimer.send(Claim {
                email_address: payload.email_address.clone(),
                random: payload.randomness,
                commit: payload.recipient_commitment,
                expire_time: expiry_time.as_u64() as i64,
                is_fund: true,
                is_announced: false,
            })?;
            Ok(format!(
                "Unclaimed fund for {} is accepted",
                payload.email_address
            ))
        }
        UnclaimType::State {
            email_addr_commit,
            sender,
            extension_addr,
            state,
            expiry_time,
        } => {
            tx_claimer.send(Claim {
                email_address: payload.email_address.clone(),
                random: payload.randomness,
                commit: payload.recipient_commitment,
                expire_time: expiry_time.as_u64() as i64,
                is_fund: false,
                is_announced: false,
            })?;
            Ok(format!(
                "Unclaimed state for {} is accepted",
                payload.email_address
            ))
        }
    }
}

pub(crate) async fn check_unclaim_valid(
    chain_client: Arc<ChainClient>,
    commitment: &str,
) -> Result<UnclaimType> {
    let recipient_commitment = hex2field(commitment)?;
    let unclaimed_fund = chain_client
        .unclaims_handler
        .unclaimed_fund_of_email_addr_commit(recipient_commitment.to_bytes())
        .call()
        .await?;
    let unclaimed_state = chain_client
        .unclaims_handler
        .unclaimed_state_of_email_addr_commit(recipient_commitment.to_bytes())
        .call()
        .await?;
    let current_time = U256::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    );
    let current_time_delayed = current_time + U256::from(DELAY);
    if unclaimed_fund.4 < current_time_delayed && unclaimed_state.4 < current_time_delayed {
        bail!("Unclaimed state/fund is expired");
    }

    if unclaimed_fund.3.is_zero() {
        Ok(UnclaimType::State {
            email_addr_commit: unclaimed_state.0,
            sender: unclaimed_state.1,
            extension_addr: unclaimed_state.2,
            state: unclaimed_state.3,
            expiry_time: unclaimed_state.4,
        })
    } else {
        Ok(UnclaimType::Fund {
            email_addr_commit: unclaimed_fund.0,
            sender: unclaimed_fund.1,
            token_addr: unclaimed_fund.2,
            amount: unclaimed_fund.3,
            expiry_time: unclaimed_fund.4,
        })
    }
}

pub(crate) async fn psi_step1(
    circuits_dir_path: &Path,
    email_addr: &str,
    client_rand: &str,
) -> Result<Point> {
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_addr.to_string() + "psi" + ".json");

    let command_str = format!(
        "--cwd {} psi-step1 --email-addr {} --client-rand {} --output {}",
        circuits_dir_path.to_str().unwrap(),
        email_addr,
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

pub(crate) async fn psi_step2(
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

pub(crate) async fn psi_step3(
    circuits_dir_path: &Path,
    point: Point,
    client_rand: &str,
) -> Result<Point> {
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
