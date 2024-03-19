#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]

use crate::*;
use chrono::{DateTime, Local};
use email_wallet_utils::*;
use ethers::abi::Token;
use ethers::types::{Bytes, U256};

use ::serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct EmailSenderInput {
    in_padded: Vec<String>,
    pubkey: Vec<String>,
    signature: Vec<String>,
    in_padded_len: String,
    sender_account_key: String,
    sender_email_idx: usize,
    subject_idx: usize,
    recipient_email_idx: usize,
    domain_idx: usize,
    timestamp_idx: usize,
}

#[derive(Serialize, Deserialize)]
struct AccountCreationInput {
    in_padded: Vec<String>,
    pubkey: Vec<String>,
    signature: Vec<String>,
    in_padded_len: String,
    relayer_rand: String,
    sender_email_idx: usize,
    code_idx: usize,
    domain_idx: usize,
    timestamp_idx: usize,
}

#[derive(Serialize, Deserialize)]
struct ClaimInput {
    email_addr: Vec<u8>,
    cm_rand: String,
    account_key: String,
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

#[named]
pub async fn generate_email_sender_input(
    circuits_dir_path: &Path,
    email: &str,
    account_key: &str,
) -> Result<String> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let circuit_input_params = circuit::CircuitInputParams::new(
        vec![],
        parsed_email.canonicalized_header.as_bytes().to_vec(),
        "".to_string(),
        vec_u8_to_bigint(parsed_email.clone().signature),
        vec_u8_to_bigint(parsed_email.clone().public_key),
        None,
        Some(1024),
        Some(64),
        Some(true),
    );
    let email_circuit_inputs = circuit::generate_circuit_inputs(circuit_input_params);

    let sender_email_idx = parsed_email.get_from_addr_idxes().unwrap();
    let domain_idx = parsed_email.get_email_domain_idxes().unwrap();
    let subject_idx = parsed_email.get_subject_all_idxes().unwrap();
    let mut recipient_email_idx = 0;
    match parsed_email.get_email_addr_in_subject_idxes() {
        Ok(idx) => recipient_email_idx = idx.0,
        Err(_) => error!(LOG, "no email addr in subject"; "func" => function_name!()),
    }
    let timestamp_idx = parsed_email.get_timestamp_idxes().unwrap();

    let email_sender_input = EmailSenderInput {
        in_padded: email_circuit_inputs.in_padded,
        pubkey: email_circuit_inputs.pubkey,
        signature: email_circuit_inputs.signature,
        in_padded_len: email_circuit_inputs.in_len_padded_bytes,
        sender_account_key: account_key.to_string(),
        sender_email_idx: sender_email_idx.0,
        subject_idx: subject_idx.0,
        recipient_email_idx: recipient_email_idx,
        domain_idx: domain_idx.0,
        timestamp_idx: timestamp_idx.0,
    };

    Ok(serde_json::to_string(&email_sender_input)?)
}

pub async fn generate_account_creation_input(
    circuits_dir_path: &Path,
    email: &str,
    relayer_rand: &str,
) -> Result<String> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let circuit_input_params = circuit::CircuitInputParams::new(
        vec![],
        parsed_email.canonicalized_header.as_bytes().to_vec(),
        "".to_string(),
        vec_u8_to_bigint(parsed_email.clone().signature),
        vec_u8_to_bigint(parsed_email.clone().public_key),
        None,
        Some(1024),
        Some(64),
        Some(true),
    );
    let email_circuit_inputs = circuit::generate_circuit_inputs(circuit_input_params);

    let sender_email_idx = parsed_email.get_from_addr_idxes().unwrap();
    let domain_idx = parsed_email.get_email_domain_idxes().unwrap();
    let subject_idx = parsed_email.get_subject_all_idxes().unwrap();
    let code_idx = parsed_email.get_invitation_code_idxes().unwrap();
    let timestamp_idx = parsed_email.get_timestamp_idxes().unwrap();

    let account_creation_input = AccountCreationInput {
        in_padded: email_circuit_inputs.in_padded,
        pubkey: email_circuit_inputs.pubkey,
        signature: email_circuit_inputs.signature,
        in_padded_len: email_circuit_inputs.in_len_padded_bytes,
        relayer_rand: relayer_rand.to_string(),
        sender_email_idx: sender_email_idx.0,
        code_idx: code_idx.0,
        domain_idx: domain_idx.0,
        timestamp_idx: timestamp_idx.0,
    };

    Ok(serde_json::to_string(&account_creation_input)?)
}

pub async fn generate_claim_input(
    circuits_dir_path: &Path,
    email_address: &str,
    email_address_rand: &str,
    account_key: &str,
) -> Result<String> {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    let mut padded_email_addr_bytes = vec![];

    for byte in padded_email_address.padded_bytes.into_iter() {
        padded_email_addr_bytes.push(byte);
    }

    let claim_input = ClaimInput {
        email_addr: padded_email_addr_bytes,
        cm_rand: email_address_rand.to_string(),
        account_key: account_key.to_string(),
    };

    Ok(serde_json::to_string(&claim_input)?)
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

// pub fn calculate_addr_pointer(email_address: &str) -> Fr {
//     let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
//     let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap()).unwrap());
//     padded_email_address.to_pointer(&relayer_rand).unwrap()
// }

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
// pub fn is_reply_mail(email: &str) -> bool {
//     email.contains("In-Reply-To:") || email.contains("References:")
// }

pub fn get_psi_point_bytes(x: U256, y: U256) -> Bytes {
    Bytes::from(abi::encode(&[Token::Uint(x), Token::Uint(y)]))
}

pub fn u256_to_bytes32(x: &U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    x.to_big_endian(&mut bytes);
    bytes
}

pub fn u256_to_hex(x: &U256) -> String {
    "0x".to_string() + &hex::encode(u256_to_bytes32(x))
}

pub fn hex_to_u256(hex: &str) -> Result<U256> {
    let bytes: Vec<u8> = hex::decode(&hex[2..])?;
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Ok(U256::from_big_endian(&array))
}

pub fn fr_to_bytes32(fr: &Fr) -> Result<[u8; 32]> {
    let hex = field2hex(fr);
    let bytes = hex::decode(&hex[2..])?;
    let mut result = [0u8; 32];
    result.copy_from_slice(&bytes);
    Ok(result)
}

pub fn bytes32_to_fr(bytes32: &[u8; 32]) -> Result<Fr> {
    let hex: String = "0x".to_string() + &hex::encode(bytes32);
    let field = hex2field(&hex)?;
    Ok(field)
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
