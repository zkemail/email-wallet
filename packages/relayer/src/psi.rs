use crate::*;

use std::path::Path;

use axum::Json;
use num_bigint::RandBigInt;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, remove_file};

#[derive(Serialize, Deserialize)]
pub(crate) struct CheckRequest {
    pub(crate) point: Point,
    pub(crate) tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RevealRequest {
    pub(crate) randomness: String,
    pub(crate) tx_hash: String,
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
    pub(crate) unclaimed_tx_hash: String,
    pub(crate) chain_client: Arc<ChainClient>,
}

impl PSIClient {
    pub(crate) async fn new(
        chain_client: Arc<ChainClient>,
        email_addr: &str,
        unclaimed_tx_hash: String,
    ) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let random = rng.gen_biguint(253);
        let random = Fr::from_bytes(&random.to_bytes_le().try_into().unwrap()).unwrap();
        let random = field2hex(&random);

        let point = psi_step1(CIRCUITS_DIR_PATH.get().unwrap(), email_addr, &random).await?;

        Ok(Self {
            email_addr: email_addr.to_string(),
            unclaimed_tx_hash,
            random,
            point,
            chain_client,
        })
    }

    pub(crate) async fn check(&self, client: reqwest::Client, address: &str) -> Result<bool> {
        let res = client
            .post(format!("{}/serveCheck/", address))
            .json(&serde_json::json!({ "point": self.point.clone(), "tx_hash": &self.unclaimed_tx_hash }))
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
        unclaimed_tx_hash: &str,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/serveReveal/", address))
            .json(&serde_json::json!({ "randomness": randomness, "tx_hash": unclaimed_tx_hash }))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

async fn serve_check_request(payload: CheckRequest) -> Result<Json<Point>> {
    todo!("check if tx confirmed onchain and unclaimed state/fund is not processed");

    let res = psi_step2(
        CIRCUITS_DIR_PATH.get().unwrap(),
        payload.point,
        RELAYER_RAND.get().unwrap(),
    )
    .await?;

    Ok(axum::response::Json(res))
}

pub(crate) async fn serve_reveal_request() -> Result<String> {
    todo!("Check that data is valid and process unclaimed state/fund")
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
