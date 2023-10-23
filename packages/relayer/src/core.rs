#![allow(clippy::upper_case_acronyms)]

use crate::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use ethers::types::{Address, Bytes, U256};
use num_bigint::RandBigInt;
use regex::Regex;
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::AsyncWriteExt,
    sync::mpsc::UnboundedSender,
};

pub(crate) async fn handle_email(
    email: String,
    db: Arc<Database>,
    tx: UnboundedSender<EmailMessage>,
) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let from_address = parsed_email.get_from_addr()?;

    if is_reply_mail(&email) {
        if let Some(account_key) = extract_key_from_subject(&parsed_email.get_subject_all()?) {
            if !db.contains_user(&from_address).await? {
                todo!("TRANSPORT")
            }

            if account_key == db.get_viewing_key(&from_address).await?.unwrap() {
                let input = generate_initialization_input(
                    CIRCUITS_DIR_PATH.get().unwrap(),
                    &email,
                    RELAYER_RAND.get().unwrap(),
                )
                .await?;

                let proof =
                    generate_proof(&input, "generateSendProof", PROVER_ADDRESS.get().unwrap())
                        .await?;

                let relayer_rand = hex2field(RELAYER_RAND.get().unwrap())?;
                let email_addr_pointer = PaddedEmailAddr::from_email_addr(&from_address)
                    .to_pointer(&RelayerRand(relayer_rand))?;
                let email_addr_pointer = Fr::to_bytes(&email_addr_pointer);

                let data = AccountInitialization {
                    email_addr_pointer,
                    email_domain: get_email_domain(&from_address)?,
                    email_timestamp: U256::from(parsed_email.get_timestamp()?),
                    email_nullifier: Fr::to_bytes(&email_nullifier(&parsed_email.signature)?),
                    proof: Bytes::from(proof.into_bytes()),
                };
                let result = call_account_initialization_op(data).await?;

                tx.send(EmailMessage {
                    subject: "Your Account was initialized".to_string(),
                    body: result,
                    to: from_address,
                    message_id: None,
                })
                .unwrap();
            }
        }
    } else {
        let viewing_key = match db.get_viewing_key(&from_address).await? {
            Some(viewing_key) => viewing_key,
            None => {
                db.remove_email(&email).await?;
                bail!(NOT_MY_SENDER);
            }
        };
        let viewing_key = AccountKey::from(hex2field(&viewing_key)?);

        let (amount, token, recipient) =
            match validate_send_subject(&parsed_email.get_subject_all()?) {
                Ok((amount, token, recipient)) => (amount, token, recipient),
                Err(e) => {
                    db.remove_email(&email).await?;
                    bail!(e);
                }
            };

        let input = generate_send_input(
            CIRCUITS_DIR_PATH.get().unwrap(),
            &email,
            RELAYER_RAND.get().unwrap(),
        )
        .await?;

        let proof =
            generate_proof(&input, "generateSendProof", PROVER_ADDRESS.get().unwrap()).await?;

        let random = rand::thread_rng().gen_biguint(253);
        let random_hex = hex2field(&format!("0x{}", random.to_str_radix(16)))?;
        let commitment = calculate_addr_commitment(&recipient, random_hex);
        db.insert_claim(&recipient, &field2hex(&random_hex), &field2hex(&commitment))
            .await?;

        let email_op = EmailOp {
            email_addr_pointer: Fr::to_bytes(&calculate_addr_pointer(&from_address)),
            has_email_recipient: true,
            recipient_email_addr_commit: Fr::to_bytes(&commitment),
            recipient_eth_addr: Address::default(),
            command: String::from("Send"),
            email_nullifier: Fr::to_bytes(&email_nullifier(&parsed_email.signature)?),
            email_domain: get_email_domain(&from_address)?,
            timestamp: parsed_email.get_timestamp()?.into(),
            masked_subject: format!("SEND {} {} to", amount, token),
            fee_token_name: "ETH".to_string(),
            fee_per_gas: U256::from(3 * (10 ^ 9)),
            execute_calldata: Bytes::new(),
            extension_name: String::new(),
            new_wallet_owner: Address::default(),
            new_dkim_registry: Address::default(),
            wallet_params: WalletParams {
                token_name: token,
                amount: amount * (10 ^ 18),
            },
            extension_params: ExtensionParams {
                subject_template_index: 0,
                subject_params: Bytes::new(),
            },
            email_proof: Bytes::from(proof.into_bytes()),
        };

        let result = call_handle_email_op(email_op).await?;

        tx.send(EmailMessage {
            subject: "Transaction were completed".to_string(),
            body: result,
            to: from_address,
            message_id: None,
        })
        .unwrap();
    }

    Ok(())
}

pub(crate) fn extract_key_from_subject(subject: &str) -> Option<String> {
    let re = Regex::new(r"CODE:(0x[0-9a-fA-F]+)").unwrap();
    re.captures(subject).map(|caps| caps[1].to_string())
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

pub(crate) fn get_email_domain(email_address: &str) -> Result<String> {
    let re = Regex::new(r"@([a-zA-Z0-9.-]+)").unwrap();
    let res = re
        .captures(email_address)
        .ok_or(anyhow!(WRONG_SUBJECT_FORMAT))?;

    Ok(res[1].to_string())
}

pub(crate) async fn generate_send_input(
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

    let command_str = format!(
        "--cwd {} gen-email-sender-input --email-file {} --relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(),
        email_file_name,
        relayer_rand,
        input_file_name
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

pub(crate) async fn generate_creation_input(
    circuits_dir_path: &Path,
    email_address: &str,
    relayer_rand: &str,
    viewing_key: &str,
) -> Result<String> {
    let input_file_name = email_address.to_string() + ".input";

    File::create(&input_file_name).await?;
    let current_dir = std::env::current_dir()?;

    let command_str =
        format!(
        "--cwd {} gen-account-creation-input --email-addr {} --relayer-rand {} --account-key {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_address, relayer_rand, viewing_key, input_file_name
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

pub(crate) async fn generate_initialization_input(
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

    let command_str = format!(
        "--cwd {} gen-account-init-input --email-file {} --relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(),
        email_file_name,
        relayer_rand,
        input_file_name
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

pub(crate) async fn generate_transport_input(
    circuits_dir_path: &Path,
    email: &str,
    old_relayer_hash: &str,
    new_relayer_rand: &str,
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
        "--cwd {} gen-account-transport-input --email-file {} --old-relayer-hash {} --new-relayer-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_file_name, old_relayer_hash, new_relayer_rand, input_file_name
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

pub(crate) async fn generate_claim_input(
    circuits_dir_path: &Path,
    email_address: &str,
    relayer_rand: &str,
    email_address_rand: &str,
) -> Result<String> {
    let input_file_name = email_address.to_string() + ".input";

    File::create(&input_file_name).await?;
    let current_dir = std::env::current_dir()?;

    let command_str =
        format!(
        "--cwd {} gen-claim-input --email-addr {} --relayer-rand {} --email-addr-rand {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_address, relayer_rand, email_address_rand, input_file_name
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

pub(crate) fn calculate_addr_pointer(email_address: &str) -> Fr {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap()).unwrap());
    padded_email_address.to_pointer(&relayer_rand).unwrap()
}

pub(crate) fn calculate_addr_commitment(email_address: &str, rand: Fr) -> Fr {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    padded_email_address.to_commitment(&rand).unwrap()
}

pub(crate) async fn generate_proof(input: &str, request: &str, address: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/{}", address, request))
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

pub(crate) fn is_reply_mail(email: &str) -> bool {
    email.contains("In-Reply-To:") || email.contains("References:")
}
