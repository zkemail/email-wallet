#![allow(clippy::upper_case_acronyms)]

use crate::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use ethers::types::{Address, Bytes, U256};
use regex::Regex;
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::AsyncWriteExt,
    sync::mpsc::UnboundedSender,
};

#[derive(Default)]
pub(crate) struct WalletParams {
    pub(crate) token_name: String,
    pub(crate) amount: U256,
}

#[derive(Default)]
pub(crate) struct ExtensionParams {
    pub(crate) subject_template_index: u8,
    pub(crate) subject_params: Bytes,
}

#[derive(Default)]
pub(crate) struct EmailOp {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) has_email_recipient: bool,
    pub(crate) recipient_email_addr_commit: [u8; 32],
    pub(crate) recipient_eth_addr: Address,
    pub(crate) command: String,
    pub(crate) email_nullifier: [u8; 32],
    pub(crate) email_domain: String,
    pub(crate) timestamp: U256,
    pub(crate) masked_subject: String,
    pub(crate) fee_token_name: String,
    pub(crate) fee_per_gas: U256,
    pub(crate) execute_calldata: Bytes,
    pub(crate) extension_name: String,
    pub(crate) new_wallet_owner: Address,
    pub(crate) new_dkim_registry: Address,
    pub(crate) wallet_params: WalletParams,
    pub(crate) extension_params: ExtensionParams,
    pub(crate) email_proof: Bytes,
}

pub(crate) async fn handle_email(
    email: String,
    db: Arc<Database>,
    tx: UnboundedSender<(String, String)>,
) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;

    let from_address = parsed_email.get_from_addr()?;
    let viewing_key = match db.get_viewing_key(&from_address).await? {
        Some(viewing_key) => viewing_key,
        None => {
            db.remove_email(&email).await?;
            bail!(NOT_MY_SENDER);
        }
    };
    let viewing_key = AccountKey::from(hex2field(&viewing_key)?);

    let (amount, token, recipient) = match validate_send_subject(&parsed_email.get_subject_all()?) {
        Ok((amount, token, recipient)) => (amount, token, recipient),
        Err(e) => {
            db.remove_email(&email).await?;
            bail!(e);
        }
    };

    let input = generate_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &email,
        RELAYER_RAND.get().unwrap(),
    )
    .await?;

    let proof = generate_coordinator_proof(&input, COORDINATOR_ADDRESS.get().unwrap()).await?;

    let email_op = EmailOp::default();

    Ok(())
}

pub(crate) fn validate_send_subject(subject: &str) -> Result<(U256, String, String)> {
    let re = Regex::new(
        r"(?i)send\s+(\d+(\.\d+)?)\s+(\w+)\s+to\s+([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})",
    )
    .unwrap();
    let caps = re.captures(subject).ok_or(anyhow!(WRONG_SUBJECT_FORMAT))?;

    let amount = U256::from_dec_str(&caps[1])?;
    let token = caps[3].to_string();
    let recipient = caps[4].to_string();

    Ok((amount, token, recipient))
}

pub(crate) async fn generate_input(
    circuits_dir_path: &Path,
    email: &str,
    relayer_rand: &str,
) -> Result<String> {
    let email_hash = calculate_hash(email);
    let email_file_name = email_hash.clone() + ".email";
    let input_file_name = email_hash + ".input";

    let mut email_file = File::create(&email_file_name).await?;
    email_file.write_all(email.as_bytes()).await?;

    File::create(&input_file_name).await?;
    let current_dir = std::env::current_dir()?;

    let command_str =
        format!(
        "yarn --cwd {} gen-email-sender-input --email-file {} --relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_file_name, relayer_rand, input_file_name
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

pub(crate) async fn generate_coordinator_proof(input: &str, address: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/generate_proof", address))
        .json(&serde_json::json!({"input": input}))
        .send()
        .await?
        .error_for_status()?;

    Ok(res.text().await?)
}

pub(crate) fn calculate_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_code = hasher.finish();

    hash_code.to_string()
}
