#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]

use crate::*;
use chrono::{DateTime, Local};
use email_wallet_utils::*;
use ethers::abi::Token;
use ethers::types::{Bytes, U256};

use ::serde::Deserialize;

use std::path::Path;

use tokio::fs::{read_to_string, remove_file};

const DOMAIN_FIELDS: usize = 9;
const SUBJECT_FIELDS: usize = 17;
const EMAIL_ADDR_FIELDS: usize = 9;

#[derive(Debug, Clone, Deserialize)]
pub struct ProverRes {
    proof: ProofJson,
    pub_signals: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProofJson {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
}

impl ProofJson {
    pub fn to_eth_bytes(&self) -> Result<Bytes> {
        let pi_a = Token::FixedArray(vec![
            Token::Uint(U256::from_dec_str(self.pi_a[0].as_str())?),
            Token::Uint(U256::from_dec_str(self.pi_a[1].as_str())?),
        ]);
        let pi_b = Token::FixedArray(vec![
            Token::FixedArray(vec![
                Token::Uint(U256::from_dec_str(self.pi_b[0][1].as_str())?),
                Token::Uint(U256::from_dec_str(self.pi_b[0][0].as_str())?),
            ]),
            Token::FixedArray(vec![
                Token::Uint(U256::from_dec_str(self.pi_b[1][1].as_str())?),
                Token::Uint(U256::from_dec_str(self.pi_b[1][0].as_str())?),
            ]),
        ]);
        let pi_c = Token::FixedArray(vec![
            Token::Uint(U256::from_dec_str(self.pi_c[0].as_str())?),
            Token::Uint(U256::from_dec_str(self.pi_c[1].as_str())?),
        ]);
        Ok(Bytes::from(abi::encode(&[pi_a, pi_b, pi_c])))
    }
}

pub async fn compute_psi_point(
    circuits_dir_path: &Path,
    email_addr: &str,
    rand: &str,
) -> Result<Point> {
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_addr.to_string() + "psi" + ".json");

    let command_str = format!(
        "--cwd {} psi-step1 --email-addr {} --client-rand {} --output {}",
        circuits_dir_path.to_str().unwrap(),
        email_addr,
        rand,
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

pub fn calculate_addr_commitment(email_address: &str, rand: Fr) -> Fr {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    padded_email_address.to_commitment(&rand).unwrap()
}

#[named]
pub async fn generate_proof(
    input: &str,
    request: &str,
    address: &str,
) -> Result<(Bytes, Vec<U256>)> {
    let client = reqwest::Client::new();
    info!(LOG, "prover input {}", input; "func" => function_name!());
    let res = client
        .post(format!("{}/prove/{}", address, request))
        .json(&serde_json::json!({ "input": input }))
        .send()
        .await?
        .error_for_status()?;
    let res_json = res.json::<ProverRes>().await?;
    info!(LOG, "prover response {:?}", res_json; "func" => function_name!());
    let proof = res_json.proof.to_eth_bytes()?;
    let pub_signals = res_json
        .pub_signals
        .into_iter()
        .map(|str| U256::from_dec_str(&str).expect("pub signal should be u256"))
        .collect();
    Ok((proof, pub_signals))
}

pub fn get_psi_point_bytes(x: U256, y: U256) -> Bytes {
    Bytes::from(abi::encode(&[Token::Uint(x), Token::Uint(y)]))
}

pub fn now() -> i64 {
    let dt: DateTime<Local> = Local::now();
    dt.timestamp()
}

pub fn derive_relayer_rand(private_key: &str) -> Result<RelayerRand> {
    let mut seed = hex::decode(&private_key[2..])?;
    seed.append(&mut b"EMAIL WALLET RELAYER RAND".to_vec());
    Ok(RelayerRand::new_from_seed(&seed)?)
}
