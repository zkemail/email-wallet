#![allow(clippy::upper_case_acronyms)]

use crate::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use email_wallet_utils::*;
use ethers::types::{Address, Bytes, U256};
use ethers::utils::hex::FromHex;
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
    chain_client: Arc<ChainClient>,
    tx: UnboundedSender<EmailMessage>,
) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let from_address = parsed_email.get_from_addr()?;

    if is_reply_mail(&email) {
        let account_key = extract_account_key_from_subject(&parsed_email.get_subject_all()?)?;
        if !db.contains_user(&from_address).await? {
            todo!("TRANSPORT")
        }
        if account_key != db.get_account_key(&from_address).await?.unwrap() {
            return Err(anyhow!(
                "from_address {} is known but the account key {} is wrong",
                from_address,
                account_key
            ));
        }
        let input = generate_account_init_input(
            CIRCUITS_DIR_PATH.get().unwrap(),
            &email,
            RELAYER_RAND.get().unwrap(),
        )
        .await?;
        let proof = generate_proof(&input, "account_init", PROVER_ADDRESS.get().unwrap()).await?;
        let relayer_rand = hex2field(RELAYER_RAND.get().unwrap())?;
        let email_addr_pointer = PaddedEmailAddr::from_email_addr(&from_address)
            .to_pointer(&RelayerRand(relayer_rand))?
            .to_bytes();
        let data = AccountInitInput {
            email_addr_pointer,
            email_domain: parsed_email.get_email_domain()?,
            email_timestamp: U256::from(parsed_email.get_timestamp()?),
            email_nullifier: email_nullifier(&parsed_email.signature)?.to_bytes(),
            proof: Bytes::from(proof.into_bytes()),
        };
        let result = chain_client.init_account(data).await?;
        tx.send(EmailMessage {
            subject: "Your Account was initialized".to_string(),
            body: result,
            to: from_address,
            message_id: None,
        })
        .unwrap();
    } else {
        let padded_from_address = PaddedEmailAddr::from_email_addr(&from_address);
        let relayer_rand = RelayerRand(hex2field(&RELAYER_RAND.get().unwrap())?);
        let email_addr_pointer = padded_from_address.to_pointer(&relayer_rand)?.to_bytes();
        let email_nullifier = email_nullifier(&parsed_email.signature)?.to_bytes();
        let subject = parsed_email.get_subject_all()?;
        let command = subject_templates::extract_command_from_subject(&subject)?;
        let account_key = db
            .get_account_key(&from_address)
            .await?
            .ok_or(anyhow!("Account key not found"))?;
        let account_key = AccountKey::from(hex2field(&account_key)?);
        let wallet_salt = account_key.to_wallet_salt()?;
        let fee_token_name = select_fee_token(&wallet_salt, &chain_client).await?;

        let (template_idx, template_vals) = match command.as_str() {
            SEND_COMMAND => (0, extract_template_vals_send(&subject)?),
            EXECUTE_COMMAND => (0, extract_template_vals_execute(&subject)?),
            INSTALL_COMMAND => (0, extract_template_vals_install(&subject)?),
            UNINSTALL_COMMAND => (0, extract_template_vals_uninstall(&subject)?),
            EXIT_COMMAND => (0, extract_template_vals_exit(&subject)?),
            DKIM_COMMAND => (0, extract_template_vals_dkim(&subject)?),
            _ => {
                // [TODO] Get `templates_array` from on-chain data.
                let extension_addr = chain_client
                    .query_user_extension_for_command(&wallet_salt, command.as_str())
                    .await?;
                let subject_templates = chain_client
                    .query_extension_templates_of_extension(extension_addr)
                    .await?;
                let (idx, vals) = extract_template_vals_and_idx(&subject, subject_templates)?;
                if idx == None {
                    bail!(WRONG_SUBJECT_FORMAT);
                }
                (idx.unwrap(), vals)
            }
        };
        let execute_call_data = if command == EXECUTE_COMMAND {
            if let TemplateValue::String(hex) = &template_vals[0] {
                Bytes::from_hex(&hex[2..])?
            } else {
                bail!(WRONG_SUBJECT_FORMAT)
            }
        } else {
            Bytes::new()
        };
        let extension_name = match command.as_str() {
            INSTALL_COMMAND | UNINSTALL_COMMAND => {
                if let TemplateValue::String(name) = &template_vals[0] {
                    name.clone()
                } else {
                    bail!(WRONG_SUBJECT_FORMAT)
                }
            }
            _ => String::new(),
        };
        let new_wallet_owner = if command == EXIT_COMMAND {
            if let TemplateValue::Address(addr) = template_vals[0] {
                addr
            } else {
                bail!(WRONG_SUBJECT_FORMAT)
            }
        } else {
            Address::default()
        };
        let new_dkim_registry = if command == DKIM_COMMAND {
            if let TemplateValue::Address(addr) = template_vals[0] {
                addr
            } else {
                bail!(WRONG_SUBJECT_FORMAT)
            }
        } else {
            Address::default()
        };

        let mut has_email_recipient = false;
        let mut recipient_email_addr_commit = [0u8; 32];
        let mut num_recipient_email_addr_bytes = U256::default();
        let mut recipient_eth_addr = Address::default();

        let wallet_params = match command.as_str() {
            SEND_COMMAND => {
                if let TemplateValue::Recipient {
                    is_email,
                    email_addr,
                    eth_addr,
                } = &template_vals[1]
                {
                    has_email_recipient = *is_email;
                    if *is_email {
                        let padded_email_addr =
                            PaddedEmailAddr::from_email_addr(&email_addr.clone().unwrap());
                        let email_addr_commit = padded_email_addr
                            .to_commitment_with_signature(&parsed_email.signature)?;
                        recipient_email_addr_commit = email_addr_commit.to_bytes();
                        num_recipient_email_addr_bytes =
                            U256::from(email_addr.as_ref().unwrap().len());
                    } else {
                        recipient_eth_addr = eth_addr.unwrap();
                    }
                } else {
                    bail!(WRONG_SUBJECT_FORMAT)
                }

                if let TemplateValue::TokenAmount { token_name, amount } = &template_vals[0] {
                    let decimal_size = chain_client.query_decimals_of_erc20(token_name).await?;
                    WalletParams {
                        token_name: token_name.clone(),
                        amount: TemplateValue::amount_to_uint(&amount, decimal_size),
                    }
                } else {
                    bail!(WRONG_SUBJECT_FORMAT)
                }
            }
            _ => WalletParams::default(),
        };
        let extension_params = match command.as_str() {
            SEND_COMMAND | EXECUTE_COMMAND | INSTALL_COMMAND | UNINSTALL_COMMAND | EXIT_COMMAND
            | DKIM_COMMAND => ExtensionParams::default(),
            _ => {
                let mut subject_params = vec![];
                for val in template_vals.iter() {
                    let amount_decimal = if let TemplateValue::TokenAmount { token_name, amount } =
                        val
                    {
                        let decimal_size = chain_client.query_decimals_of_erc20(token_name).await?;
                        Some(decimal_size)
                    } else if let TemplateValue::Amount(amount) = val {
                        Some(18)
                    } else if let TemplateValue::Recipient {
                        is_email,
                        email_addr,
                        eth_addr,
                    } = val
                    {
                        has_email_recipient = *is_email;
                        if *is_email {
                            let padded_email_addr =
                                PaddedEmailAddr::from_email_addr(&email_addr.clone().unwrap());
                            let email_addr_commit = padded_email_addr
                                .to_commitment_with_signature(&parsed_email.signature)?;
                            recipient_email_addr_commit = email_addr_commit.to_bytes();
                            num_recipient_email_addr_bytes =
                                U256::from(email_addr.as_ref().unwrap().len());
                        } else {
                            recipient_eth_addr = eth_addr.unwrap();
                        }
                        None
                    } else {
                        None
                    };
                    subject_params.push(val.abi_encode(amount_decimal)?);
                }
                ExtensionParams {
                    subject_template_index: template_idx as u8,
                    subject_params,
                }
            }
        };

        let input = generate_email_sender_input(
            CIRCUITS_DIR_PATH.get().unwrap(),
            &email,
            RELAYER_RAND.get().unwrap(),
        )
        .await?;
        let proof = generate_proof(&input, "email_sender", PROVER_ADDRESS.get().unwrap()).await?;

        let email_op = EmailOp {
            email_addr_pointer: email_addr_pointer,
            has_email_recipient,
            recipient_email_addr_commit: recipient_email_addr_commit,
            num_recipient_email_addr_bytes,
            recipient_eth_addr: recipient_eth_addr,
            command: command,
            email_nullifier: email_nullifier,
            email_domain: parsed_email.get_email_domain()?,
            timestamp: parsed_email.get_timestamp()?.into(),
            masked_subject: get_masked_subject(&subject)?,
            fee_token_name,
            fee_per_gas: FEE_PER_GAS.get().unwrap().clone(),
            execute_call_data,
            extension_name,
            new_wallet_owner,
            new_dkim_registry,
            wallet_params,
            extension_params,
            email_proof: Bytes::from(proof.into_bytes()),
        };

        let result = chain_client.handle_email_op(email_op).await?;

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

pub(crate) fn extract_account_key_from_subject(subject: &str) -> Result<String> {
    let regex_config = serde_json::from_str(include_str!(
        "../../circuits/src/regexes/invitation_code.json"
    ))
    .unwrap();
    let substr_idxes = extract_substr_idxes(subject, &regex_config)?;
    Ok(subject[substr_idxes[0].0..substr_idxes[0].1].to_string())
}

pub(crate) fn get_masked_subject(subject: &str) -> Result<String> {
    let (start, end) = extract_email_addr_idxes(subject)?[0];
    let mut masked_subject_bytes = subject.as_bytes().to_vec();
    masked_subject_bytes[start..end].copy_from_slice(vec![0u8; end - start].as_ref());
    Ok(String::from_utf8(masked_subject_bytes)?)
}
// pub(crate) fn validate_send_subject(subject: &str) -> Result<(U256, String, String)> {
//     let re = Regex::new(
//         r"(?i)send\s+(\d+(\.\d+)?)\s+(\w+)\s+to\s+([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})",
//     )
//     .unwrap();
//     let caps = re.captures(subject).ok_or(anyhow!(WRONG_SUBJECT_FORMAT))?;

//     let amount = U256::from_dec_str(&caps[1])?;
//     let token = caps[3].to_string();
//     let recipient = caps[4].to_string();

//     Ok((amount, token, recipient))
// }

// pub(crate) fn get_email_domain(email_address: &str) -> Result<String> {
//     let re = Regex::new(r"@([a-zA-Z0-9.-]+)").unwrap();
//     let res = re
//         .captures(email_address)
//         .ok_or(anyhow!(WRONG_SUBJECT_FORMAT))?;

//     Ok(res[1].to_string())
// }

pub(crate) async fn generate_email_sender_input(
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

pub(crate) async fn generate_account_creation_input(
    circuits_dir_path: &Path,
    email_address: &str,
    relayer_rand: &str,
    account_key: &str,
) -> Result<String> {
    let input_file_name = email_address.to_string() + ".input";

    File::create(&input_file_name).await?;
    let current_dir = std::env::current_dir()?;

    let command_str =
        format!(
        "--cwd {} gen-account-creation-input --email-addr {} --relayer-rand {} --account-key {} --input-file {}",
        circuits_dir_path.to_str().unwrap(), email_address, relayer_rand, account_key, input_file_name
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

pub(crate) async fn generate_account_init_input(
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
        .json(&serde_json::json!({ "input": input }))
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

pub(crate) fn extract_old_relayer_email(email_body: &str) -> Result<String> {
    let re = Regex::new(r"Old Relayer: ([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})").unwrap();
    let caps = re
        .captures(email_body)
        .ok_or(anyhow!(WRONG_SUBJECT_FORMAT))?;

    Ok(caps[1].to_string())
}

pub(crate) async fn select_fee_token(
    wallet_salt: &WalletSalt,
    chain_client: &Arc<ChainClient>,
) -> Result<String> {
    let eth_balance = chain_client
        .query_user_erc20_balance(wallet_salt, "ETH")
        .await?;
    let dai_balance = chain_client
        .query_user_erc20_balance(wallet_salt, "DAI")
        .await?;
    let usdc_balance = chain_client
        .query_user_erc20_balance(wallet_salt, "USDC")
        .await?;
    let usdc_balance = usdc_balance * (10u64.pow(18 - 6));
    let max = eth_balance.max(dai_balance).max(usdc_balance);
    if max == eth_balance {
        Ok("ETH".to_string())
    } else if max == dai_balance {
        Ok("DAI".to_string())
    } else {
        Ok("USDC".to_string())
    }
}
