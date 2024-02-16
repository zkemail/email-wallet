#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]

use crate::*;
use chrono::{DateTime, Local};
use email_wallet_utils::*;
use ethers::abi::Token;
use ethers::types::{Address, Bytes, U256};
use ethers::utils::hex::FromHex;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::atomic::Ordering;
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::AsyncWriteExt,
    sync::mpsc::UnboundedSender,
};

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

pub(crate) async fn generate_email_sender_input(
    circuits_dir_path: &Path,
    email: &str,
    relayer_rand: &str,
) -> Result<String> {
    let email_hash = calculate_default_hash(email);
    let email_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_hash.to_string() + ".email");
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_hash.to_string() + ".json");

    let mut email_file = File::create(&email_file_name).await?;
    email_file.write_all(email.as_bytes()).await?;

    let command_str = format!(
        "--cwd {} gen-email-sender-input --email-file {} --relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(),
        email_file_name.to_str().unwrap(),
        relayer_rand,
        input_file_name.to_str().unwrap()
    );

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .spawn()?;

    let status = proc.wait().await?;
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;

    remove_file(email_file_name).await?;
    remove_file(input_file_name).await?;

    Ok(result)
}

// pub(crate) async fn generate_account_creation_input(
//     circuits_dir_path: &Path,
//     email_address: &str,
//     relayer_rand: &str,
//     account_key: &str,
// ) -> Result<String> {
//     let input_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(email_address.to_string() + ".json");

//     let current_dir = std::env::current_dir()?;

//     let command_str =
//         format!(
//         "--cwd {} gen-account-creation-input --email-addr {} --relayer-rand {} --account-key {} --input-file {}",
//         circuits_dir_path.to_str().unwrap(), email_address, relayer_rand, account_key, input_file_name.to_str().unwrap()
//     );

//     let mut proc = tokio::process::Command::new("yarn")
//         .args(command_str.split_whitespace())
//         .spawn()?;

//     let status = proc.wait().await?;
//     assert!(status.success());

//     let result = read_to_string(&input_file_name).await?;
//     remove_file(input_file_name).await?;

//     Ok(result)
// }

pub(crate) async fn generate_account_creation_input(
    circuits_dir_path: &Path,
    email: &str,
    relayer_rand: &str,
) -> Result<String> {
    let email_hash = calculate_default_hash(email);
    let email_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_hash.to_string() + ".email");
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_hash.to_string() + ".json");

    let mut email_file = File::create(&email_file_name).await?;
    email_file.write_all(email.as_bytes()).await?;

    // let current_dir = std::env::current_dir()?;

    let command_str = format!(
        "--cwd {} gen-account-init-input --email-file {} --relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(),
        email_file_name.to_str().unwrap(),
        relayer_rand,
        input_file_name.to_str().unwrap()
    );

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .spawn()?;

    let status = proc.wait().await?;
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;

    remove_file(email_file_name).await?;
    remove_file(input_file_name).await?;

    Ok(result)
}

// pub(crate) async fn generate_account_transport_input(
//     circuits_dir_path: &Path,
//     email: &str,
//     old_relayer_hash: &str,
//     new_relayer_rand: &str,
// ) -> Result<String> {
//     let email_hash = calculate_default_hash(email);
//     let email_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(email_hash.to_string() + ".email");
//     let input_file_name = PathBuf::new()
//         .join(INPUT_FILES_DIR.get().unwrap())
//         .join(email_hash.to_string() + ".json");

//     let mut email_file = File::create(&email_file_name).await?;
//     email_file.write_all(email.as_bytes()).await?;

//     // File::create(&input_file_name).await?;
//     // let current_dir = std::env::current_dir()?;

//     let command_str =
//         format!(
//         "--cwd {} gen-account-transport-input --email-file {} --old-relayer-hash {} --new-relayer-rand {} --input-file {}",
//         circuits_dir_path.to_str().unwrap(), email_file_name.to_str().unwrap(), old_relayer_hash, new_relayer_rand, input_file_name.to_str().unwrap()
//     );

//     let mut proc = tokio::process::Command::new("yarn")
//         .args(command_str.split_whitespace())
//         .spawn()?;

//     let status = proc.wait().await?;
//     assert!(status.success());

//     let result = read_to_string(&input_file_name).await?;

//     remove_file(email_file_name).await?;
//     remove_file(input_file_name).await?;

//     Ok(result)
// }

pub(crate) async fn generate_claim_input(
    circuits_dir_path: &Path,
    email_address: &str,
    email_address_rand: &str,
    account_key: &str,
) -> Result<String> {
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_address.to_string() + ".json");

    let command_str =
        format!(
        "--cwd {} gen-claim-input --email-addr {} --email-addr-rand {} --account-key {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_address, email_address_rand, account_key, input_file_name.to_str().unwrap()
    );

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .spawn()?;

    let status = proc.wait().await?;
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;

    remove_file(input_file_name).await?;

    Ok(result)
}

// pub(crate) fn calculate_addr_pointer(email_address: &str) -> Fr {
//     let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
//     let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap()).unwrap());
//     padded_email_address.to_pointer(&relayer_rand).unwrap()
// }

pub(crate) fn calculate_addr_commitment(email_address: &str, rand: Fr) -> Fr {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    padded_email_address.to_commitment(&rand).unwrap()
}

#[named]
pub(crate) async fn generate_proof(
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
    let proof = res_json.proof.to_eth_bytes()?;
    let pub_signals = res_json
        .pub_signals
        .into_iter()
        .map(|str| U256::from_dec_str(&str).expect("pub signal should be u256"))
        .collect();
    Ok((proof, pub_signals))
}
// pub(crate) fn is_reply_mail(email: &str) -> bool {
//     email.contains("In-Reply-To:") || email.contains("References:")
// }

pub(crate) fn get_psi_point_bytes(x: U256, y: U256) -> Bytes {
    Bytes::from(abi::encode(&[Token::Uint(x), Token::Uint(y)]))
}

pub(crate) fn u256_to_bytes32(x: &U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    x.to_big_endian(&mut bytes);
    bytes
}

pub(crate) fn u256_to_hex(x: &U256) -> String {
    "0x".to_string() + &hex::encode(u256_to_bytes32(x))
}

pub(crate) fn hex_to_u256(hex: &str) -> Result<U256> {
    let bytes: Vec<u8> = hex::decode(&hex[2..])?;
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Ok(U256::from_big_endian(&array))
}

pub(crate) fn fr_to_bytes32(fr: &Fr) -> Result<[u8; 32]> {
    let hex = field2hex(fr);
    let bytes = hex::decode(&hex[2..])?;
    let mut result = [0u8; 32];
    result.copy_from_slice(&bytes);
    Ok(result)
}

pub(crate) fn bytes32_to_fr(bytes32: &[u8; 32]) -> Result<Fr> {
    let hex: String = "0x".to_string() + &hex::encode(bytes32);
    let field = hex2field(&hex)?;
    Ok(field)
}

pub(crate) fn now() -> i64 {
    let dt: DateTime<Local> = Local::now();
    dt.timestamp()
}

pub(crate) fn derive_relayer_rand(private_key: &str) -> Result<RelayerRand> {
    let mut seed = hex::decode(&private_key[2..])?;
    seed.append(&mut b"EMAIL WALLET RELAYER RAND".to_vec());
    Ok(RelayerRand::new_from_seed(&seed)?)
}
