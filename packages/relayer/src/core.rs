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

#[named]
pub(crate) async fn handle_email<P: EmailsPool>(
    email: String,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    emails_pool: P,
    tx_sender: Arc<UnboundedSender<EmailMessage>>,
    tx_claimer: UnboundedSender<Claim>,
    // tx_creator: UnboundedSender<String>,
) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    trace!(LOG, "email: {}", email; "func" => function_name!());
    let from_addr = parsed_email.get_from_addr()?;
    let padded_from_addr = PaddedEmailAddr::from_email_addr(&from_addr);
    trace!(LOG, "From address: {}", from_addr; "func" => function_name!());
    check_and_update_dkim(&email, &parsed_email, &chain_client).await?;
    if let Ok(invitation_code) = parsed_email.get_invitation_code() {
        trace!(LOG, "Email with invitation code"; "func" => function_name!());
        let account_key = AccountKey::from(hex2field(&format!("0x{}", invitation_code))?);
        if db.contains_user(&from_addr).await? {
            let stored_account_key = db.get_account_key(&from_addr).await?.unwrap();
            if stored_account_key != field2hex(&account_key.0) {
                return Err(anyhow!(
                    "Stored account key is not equal to one in the email: {} != {}",
                    stored_account_key,
                    field2hex(&account_key.0)
                ));
            }
        }
        let wallet_salt = WalletSalt::new(&padded_from_addr, account_key)?;
        if !chain_client
            .check_if_account_created_by_account_key(&from_addr, &field2hex(&account_key.0))
            .await?
        {
            let input = generate_account_creation_input(
                CIRCUITS_DIR_PATH.get().unwrap(),
                &from_addr,
                RELAYER_RAND.get().unwrap(),
            )
            .await?;
            let (proof, pub_signals) =
                generate_proof(&input, "account_creation", PROVER_ADDRESS.get().unwrap()).await?;
            let email_proof = EmailProof {
                domain: parsed_email.get_email_domain()?,
                timestamp: pub_signals[DOMAIN_FIELDS + 2],
                nullifier: u256_to_bytes32(&pub_signals[DOMAIN_FIELDS + 1]),
                dkim_public_key_hash: u256_to_bytes32(&pub_signals[DOMAIN_FIELDS + 0]),
                proof: proof,
            };
            let data = AccountCreationInput {
                wallet_salt: u256_to_bytes32(&pub_signals[3]),
                psi_point: get_psi_point_bytes(pub_signals[4], pub_signals[5]),
                proof: email_proof,
            };
            info!(LOG, "Account creation data {:?}", data; "func" => function_name!());
            let res = chain_client.create_account(data).await?;
            info!(LOG, "account creation tx hash: {}", res; "func" => function_name!());
            db.insert_user(&from_addr, &field2hex(&account_key.0), &res, false)
                .await?;
            let wallet_addr = chain_client
                .get_wallet_addr_from_salt(&wallet_salt.0)
                .await?;
            info!(LOG, "Sender wallet address: {}", wallet_addr; "func" => function_name!());
            let token_transfered = chain_client
                .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
                .await?;
            let claims = db.get_claims_by_email_addr(&from_addr).await?;
            for claim in claims {
                tx_claimer.send(claim)?;
            }
            let email_hash = calculate_default_hash(&email);
            emails_pool.insert_email(&email_hash, &email).await?;
            return Ok(());
        }
    }
    trace!(LOG, "Process as a normal email"; "func" => function_name!());
    let account_key_str = db.get_account_key(&from_addr).await?.ok_or(anyhow!(
        "The user of email address {} is not registered.",
        from_addr
    ))?;
    let account_key = AccountKey(hex2field(&account_key_str)?);
    let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap())?);
    let wallet_salt = WalletSalt::new(&padded_from_addr, account_key)?;
    trace!(LOG, "Wallet salt: {}", field2hex(&wallet_salt.0); "func" => function_name!());
    let wallet_addr = chain_client
        .get_wallet_addr_from_salt(&wallet_salt.0)
        .await?;
    info!(LOG, "Sender wallet address: {}", wallet_addr; "func" => function_name!());
    let mut subject = parsed_email.get_subject_all()?;
    trace!(LOG, "Subject: {}", subject; "func" => function_name!());
    let (command, skip_subject_prefix) =
        subject_templates::extract_command_from_subject(&subject, &chain_client, &wallet_salt)
            .await?;
    subject = subject[skip_subject_prefix..].to_string();
    trace!(LOG, "Command: {}", command; "func" => function_name!());
    trace!(LOG, "Skip Subject Prefix: {}", skip_subject_prefix; "func" => function_name!());
    trace!(LOG, "Prefix Skipped Subject: {}", subject; "func" => function_name!());
    let fee_token_name = select_fee_token(&wallet_salt, &chain_client).await?;
    trace!(LOG, "Fee token name: {}", fee_token_name; "func" => function_name!());
    let (template_idx, template_vals) = match command.as_str() {
        SEND_COMMAND => (0, extract_template_vals_send(&subject)?),
        EXECUTE_COMMAND => (0, extract_template_vals_execute(&subject)?),
        INSTALL_COMMAND => (0, extract_template_vals_install(&subject)?),
        UNINSTALL_COMMAND => (0, extract_template_vals_uninstall(&subject)?),
        EXIT_COMMAND => (0, extract_template_vals_exit(&subject)?),
        DKIM_COMMAND => (0, extract_template_vals_dkim(&subject)?),
        _ => {
            let extension_addr = chain_client
                .query_user_extension_for_command(&wallet_salt, command.as_str())
                .await?;
            let subject_templates = chain_client
                .query_subject_templates_of_extension(extension_addr)
                .await?;
            let (idx, vals) = extract_template_vals_and_idx(&subject, subject_templates)?;
            if idx.is_none() {
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

    let mut recipient_email_addr = None;
    // let mut num_recipient_email_addr_bytes = U256::default();
    let mut recipient_eth_addr = Address::default();

    let wallet_params = match command.as_str() {
        SEND_COMMAND => {
            if let TemplateValue::Recipient {
                is_email,
                email_addr,
                eth_addr,
            } = &template_vals[1]
            {
                if *is_email {
                    recipient_email_addr = email_addr.clone();
                    let padded_email_addr =
                        PaddedEmailAddr::from_email_addr(&email_addr.clone().unwrap());
                    let email_addr_commit =
                        padded_email_addr.to_commitment_with_signature(&parsed_email.signature)?;
                    // num_recipient_email_addr_bytes =
                    //     U256::from(email_addr.as_ref().unwrap().len());
                    info!(LOG, "derived commit {:?}", email_addr_commit; "func" => function_name!());
                } else {
                    recipient_eth_addr = eth_addr.unwrap();
                }
            } else {
                bail!(WRONG_SUBJECT_FORMAT)
            }

            if let TemplateValue::TokenAmount { token_name, amount } = &template_vals[0] {
                info!(LOG, "token name: {}", token_name; "func" => function_name!());
                info!(
                    LOG,
                    "token addr: {}",
                    chain_client.query_erc20_address(token_name).await?; "func" => function_name!()
                );
                info!(
                    LOG,
                    "decimal: {}",
                    chain_client
                        .query_decimals_of_erc20_address(
                            chain_client.query_erc20_address(token_name).await?
                        )
                        .await?; "func" => function_name!()
                );
                let decimal_size = chain_client.query_decimals_of_erc20(token_name).await?;
                info!(LOG, "decimal size: {}", decimal_size; "func" => function_name!());
                WalletParams {
                    token_name: token_name.clone(),
                    amount: TemplateValue::amount_to_uint(amount, decimal_size),
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
                match val {
                    TemplateValue::TokenAmount { token_name, amount } => {
                        let decimal_size = chain_client
                            .query_decimals_of_erc20(token_name.as_str())
                            .await?;
                        subject_params.push(val.abi_encode(Some(decimal_size))?);
                    }
                    TemplateValue::Amount(amount) => {
                        subject_params.push(val.abi_encode(Some(18))?);
                    }
                    TemplateValue::Recipient {
                        is_email,
                        email_addr,
                        eth_addr,
                    } => {
                        if !is_email {
                            recipient_eth_addr = eth_addr.unwrap();
                        } else {
                            recipient_email_addr = email_addr.clone();
                        }
                    }
                    _ => {
                        subject_params.push(val.abi_encode(None)?);
                    }
                }
            }
            ExtensionParams {
                subject_template_index: template_idx as u8,
                subject_params,
            }
        }
    };
    trace!(LOG, "parameter constructed"; "func" => function_name!());
    let input = generate_email_sender_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &email,
        RELAYER_RAND.get().unwrap(),
    )
    .await?;
    trace!(LOG, "input generated"; "func" => function_name!());
    let (email_proof, pub_signals) =
        generate_proof(&input, "email_sender", PROVER_ADDRESS.get().unwrap()).await?;
    trace!(LOG, "proof generated"; "func" => function_name!());
    // let email_addr_pointer = u256_to_bytes32(&pub_signals[SUBJECT_FIELDS + DOMAIN_FIELDS + 3]);
    let has_email_recipient = pub_signals[SUBJECT_FIELDS + DOMAIN_FIELDS + 4] == 1u8.into();
    let recipient_email_addr_commit =
        u256_to_bytes32(&pub_signals[SUBJECT_FIELDS + DOMAIN_FIELDS + 5]);
    let email_nullifier = u256_to_bytes32(&pub_signals[DOMAIN_FIELDS + 1]);
    let timestamp = pub_signals[DOMAIN_FIELDS + 2];
    let (masked_subject, num_recipient_email_addr_bytes) = get_masked_subject(&subject)?;
    info!(LOG, "masked_subject {}", masked_subject; "func" => function_name!());
    info!(
        LOG,
        "num_recipient_email_addr_bytes {}", num_recipient_email_addr_bytes; "func" => function_name!()
    );
    info!(
        LOG,
        "commit in pub_signals {:?}",
        bytes32_to_fr(&recipient_email_addr_commit)?; "func" => function_name!()
    );
    let email_op = EmailOp {
        wallet_salt: fr_to_bytes32(&wallet_salt.0)?,
        has_email_recipient,
        recipient_email_addr_commit,
        num_recipient_email_addr_bytes: U256::from(num_recipient_email_addr_bytes),
        recipient_eth_addr,
        command: command.clone(),
        email_nullifier,
        email_domain: parsed_email.get_email_domain()?,
        dkim_public_key_hash: u256_to_bytes32(&pub_signals[DOMAIN_FIELDS + 0]),
        timestamp,
        masked_subject,
        skip_subject_prefix: U256::from(skip_subject_prefix),
        fee_token_name,
        fee_per_gas: *FEE_PER_GAS.get().unwrap(),
        execute_call_data,
        extension_name,
        new_wallet_owner,
        new_dkim_registry,
        wallet_params,
        extension_params,
        email_proof,
    };
    trace!(LOG, "email_op constructed: {:?}", email_op; "func" => function_name!());
    chain_client.validate_email_op(email_op.clone()).await?;
    let (tx_hash, registered_unclaim_id) = chain_client.handle_email_op(email_op).await?;
    info!(LOG, "email_op broadcased to chain: {}", tx_hash; "func" => function_name!());
    if let Some(email_addr) = recipient_email_addr.as_ref() {
        info!(LOG, "recipient email address: {}", email_addr; "func" => function_name!());
        let commit_rand = extract_rand_from_signature(&parsed_email.signature)?;
        // let commit = bytes32_to_fr(&recipient_email_addr_commit)?;
        let is_fund = command == SEND_COMMAND;
        let expiry_time = if is_fund {
            let unclaimed_fund: UnclaimedFund = chain_client
                .query_unclaimed_fund(registered_unclaim_id)
                .await?;
            i64::try_from(unclaimed_fund.expiry_time.as_u64()).unwrap()
        } else {
            let unclaimed_state = chain_client
                .query_unclaimed_state(registered_unclaim_id)
                .await?;
            i64::try_from(unclaimed_state.expiry_time.as_u64()).unwrap()
        };
        let commit = "0x".to_string() + &hex::encode(recipient_email_addr_commit);
        let psi_client = PSIClient::new(
            Arc::clone(&chain_client),
            email_addr.to_string(),
            registered_unclaim_id,
            is_fund,
        )
        .await?;
        psi_client
            .check_and_reveal(db.clone(), chain_client.clone(), &email_addr)
            .await?;

        let claim = Claim {
            id: registered_unclaim_id,
            email_address: email_addr.clone(),
            commit,
            random: field2hex(&commit_rand),
            expiry_time,
            is_fund,
            is_announced: false,
        };
        tx_claimer.send(claim)?;
        trace!(LOG, "claim sent to tx_claimer"; "func" => function_name!());
        (*tx_sender)
            .clone()
            .send(EmailMessage {
                to: email_addr.to_string(),
                email_args: EmailArgs::TxReceived {
                    user_email_addr: email_addr.to_string(),
                },
                account_key: None,
                wallet_addr: None,
                tx_hash: Some(tx_hash.clone()),
            })
            .unwrap();
    }
    let message_id = parsed_email.get_message_id()?;
    (*tx_sender)
        .clone()
        .send(EmailMessage {
            to: from_addr.clone(),
            email_args: EmailArgs::TxComplete {
                user_email_addr: from_addr,
                original_subject: subject,
                reply_to: message_id,
            },
            account_key: Some(field2hex(&account_key.0)),
            wallet_addr: Some(ethers::utils::to_checksum(&wallet_addr, None)),
            tx_hash: Some(tx_hash),
        })
        .unwrap();
    trace!(LOG, "email_op sent to tx_sender"; "func" => function_name!());

    Ok(())
}

pub(crate) fn get_masked_subject(subject: &str) -> Result<(String, usize)> {
    match extract_email_addr_idxes(subject) {
        Ok(extracts) => {
            if extracts.len() != 1 {
                return Err(anyhow!(
                    "Recipient address in the subject must appear only once."
                ));
            }
            let (start, end) = extracts[0];
            if end == subject.len() {
                Ok((subject[0..start].to_string(), 0))
            } else {
                let mut masked_subject_bytes = subject.as_bytes().to_vec();
                masked_subject_bytes[start..end].copy_from_slice(vec![0u8; end - start].as_ref());
                Ok((String::from_utf8(masked_subject_bytes)?, end - start))
            }
        }
        Err(_) => Ok((subject.to_string(), 0)),
    }
}

pub(crate) fn calculate_default_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_code = hasher.finish();

    hash_code.to_string()
}

pub(crate) async fn select_fee_token(
    wallet_salt: &WalletSalt,
    chain_client: &Arc<ChainClient>,
) -> Result<String> {
    let eth_balance = match chain_client
        .query_user_erc20_balance(wallet_salt, "ETH")
        .await
    {
        Ok(balance) => balance,
        Err(_) => U256::from(0),
    };
    let dai_balance = match chain_client
        .query_user_erc20_balance(wallet_salt, "DAI")
        .await
    {
        Ok(balance) => balance,
        Err(_) => U256::from(0),
    };
    let usdc_balance = match chain_client
        .query_user_erc20_balance(wallet_salt, "USDC")
        .await
    {
        Ok(balance) => balance,
        Err(_) => U256::from(0),
    };
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

#[named]
pub(crate) async fn check_and_update_dkim(
    email: &str,
    parsed_email: &ParsedEmail,
    chain_client: &Arc<ChainClient>,
) -> Result<()> {
    let mut public_key_n = parsed_email.public_key.clone();
    public_key_n.reverse();
    let public_key_hash = public_key_hash(&public_key_n)?;
    info!(LOG, "public_key_hash {:?}", public_key_hash; "func" => function_name!());
    let domain = parsed_email.get_email_domain()?;
    info!(LOG, "domain {:?}", domain; "func" => function_name!());
    if chain_client
        .check_if_dkim_public_key_hash_valid(domain.clone(), fr_to_bytes32(&public_key_hash)?)
        .await?
    {
        info!(LOG, "public key registered"; "func" => function_name!());
        return Ok(());
    }
    let selector_decomposed_def =
        serde_json::from_str(include_str!("./selector_def.json")).unwrap();
    let selector = {
        let idxes =
            extract_substr_idxes(&parsed_email.canonicalized_header, &selector_decomposed_def)?[0];
        let str = parsed_email.canonicalized_header[idxes.0..idxes.1].to_string();
        str
    };
    info!(LOG, "selector {}", selector; "func" => function_name!());
    let ic_agent = DkimOracleClient::gen_agent(
        &env::var(PEM_PATH_KEY).unwrap(),
        &env::var(IC_REPLICA_URL_KEY).unwrap(),
    )?;
    let oracle_client = DkimOracleClient::new(&env::var(CANISTER_ID_KEY).unwrap(), &ic_agent)?;
    let oracle_result = oracle_client.request_signature(&selector, &domain).await?;
    info!(LOG, "DKIM oracle result {:?}", oracle_result; "func" => function_name!());
    let public_key_hash = hex::decode(&oracle_result.public_key_hash[2..])?;
    info!(LOG, "public_key_hash from oracle {:?}", public_key_hash; "func" => function_name!());
    let signature = Bytes::from_hex(&oracle_result.signature[2..])?;
    info!(LOG, "signature {:?}", signature; "func" => function_name!());
    let tx_hash = chain_client
        .set_dkim_public_key_hash(
            selector,
            domain,
            TryInto::<[u8; 32]>::try_into(public_key_hash).unwrap(),
            signature,
        )
        .await?;
    info!(LOG, "DKIM registry updated {:?}", tx_hash; "func" => function_name!());
    Ok(())
}
