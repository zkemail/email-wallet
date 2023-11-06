#![allow(clippy::upper_case_acronyms)]

use crate::*;

use ethers::types::Address;
use ethers::utils::format_units;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) struct Claim {
    pub email_address: String,
    pub random: String,
    pub commit: String,
    pub expire_time: i64,
    pub is_fund: bool,
    pub is_announced: bool,
}

pub(crate) async fn claim_unclaims(
    claim: Claim,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_creator: UnboundedSender<String>,
    tx_sender: UnboundedSender<EmailMessage>,
) -> Result<()> {
    if !db.contains_user(&claim.email_address).await.unwrap() {
        tx_creator.send(claim.email_address.clone())?;
        db.insert_claim(
            &claim.email_address,
            &claim.random,
            &claim.commit,
            claim.expire_time,
            claim.is_fund,
            claim.is_announced,
        )
        .await?;
        return Ok(());
    }
    let account_key_str = db
        .get_account_key(&claim.email_address)
        .await?
        .ok_or(anyhow!(
            "Account key not found for email address: {}",
            claim.email_address
        ))?;
    let account_key = AccountKey(hex2field(&account_key_str)?);
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&claim.email_address);
    let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap())?);
    let account_key_commit =
        account_key.to_commitment(&padded_email_addr, &relayer_rand.hash()?)?;
    let info = chain_client
        .query_account_info(&account_key_commit)
        .await
        .unwrap();
    if !info.initialized {
        info!("Account of {} is not intialized", claim.email_address);
        tx_sender
            .send(EmailMessage {
                subject: format!("New Account - CODE:{}", account_key_str),
                // TODO: get transaction hash
                body: email_create_message(&account_key_str, "TransactionHash").await?,
                to: claim.email_address.to_string(),
                message_id: Some(account_key_str),
            })
            .unwrap();
        return Ok(());
    }
    let now = now();
    let wallet_salt = WalletSalt(bytes32_to_fr(&info.wallet_salt)?);
    let reply_msg = if claim.is_fund {
        let unclaimed_fund = chain_client
            .query_unclaimed_fund(fr_to_bytes32(&hex2field(&claim.commit)?)?)
            .await?;
        if unclaimed_fund.expire_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        let mut token_name = chain_client
            .query_token_name(unclaimed_fund.token_addr)
            .await?;
        if token_name == "WETH" {
            token_name = "ETH".to_string();
        }
        let decimals = chain_client
            .query_decimals_of_erc20_address(unclaimed_fund.token_addr)
            .await?;
        let token_amount_str = format_units(unclaimed_fund.amount, decimals as u32)?;
        format!(
            "You received {} {} from {}",
            token_amount_str, token_name, unclaimed_fund.sender
        )
    } else {
        let unclaimed_state = chain_client
            .query_unclaimed_state(fr_to_bytes32(&hex2field(&claim.commit)?)?)
            .await?;
        if unclaimed_state.expire_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        if claim.is_announced
            && is_installed_extension(unclaimed_state.extension_addr, &wallet_salt, &chain_client)
                .await?
        {
            return Err(anyhow!(
                "Unclaimed state anounces the email address but its extension is not installed."
            ));
        }
        format!(
            "You got new data of {} from {}",
            unclaimed_state.extension_addr, unclaimed_state.sender
        )
    };
    let input = generate_claim_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &claim.email_address,
        field2hex(&relayer_rand.0).as_str(),
        &claim.random,
    )
    .await?;
    let (proof, pub_signals) =
        generate_proof(&input, "claim", PROVER_ADDRESS.get().unwrap()).await?;
    info!("original commit {}", claim.commit);
    info!("original randomness {}", claim.random);
    info!("commit in pub signals: {}", pub_signals[2]);
    let data = ClaimInput {
        email_addr_pointer: u256_to_bytes32(pub_signals[1]),
        email_addr_commit: u256_to_bytes32(pub_signals[2]),
        is_fund: claim.is_fund,
        proof,
    };
    let result = chain_client.claim(data).await?;
    db.delete_claim(&claim.commit, claim.is_fund).await?;
    // TODO: eml template for unclaim
    tx_sender
        .send(EmailMessage {
            subject: reply_msg.to_string(),
            body: format!("{} Transaction: {}", reply_msg, result),
            to: claim.email_address.to_string(),
            message_id: None,
        })
        .unwrap();
    Ok(())
}

async fn is_installed_extension(
    extension_addr: Address,
    wallet_salt: &WalletSalt,
    chain_client: &Arc<ChainClient>,
) -> Result<bool> {
    let subject_templates = chain_client
        .query_subject_templates_of_extension(extension_addr)
        .await?;
    let command = subject_templates[0][0].as_str();
    let installed_extension = chain_client
        .query_user_extension_for_command(wallet_salt, command)
        .await?;
    Ok(installed_extension == extension_addr)
}
