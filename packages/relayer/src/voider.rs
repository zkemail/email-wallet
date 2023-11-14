#![allow(clippy::upper_case_acronyms)]

use crate::*;

use ethers::types::Address;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) async fn void_unclaims(
    claim: Claim,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_sender: UnboundedSender<EmailMessage>,
) -> Result<()> {
    let now = now();
    let commit = hex2field(&claim.commit)?;
    db.delete_claim(&claim.id, claim.is_fund).await?;
    let (reply_msg, sender, tx_hash) = if claim.is_fund {
        let unclaimed_fund = chain_client.query_unclaimed_fund(claim.id).await?;
        if unclaimed_fund.expiry_time.as_u64() > u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim is not expired"));
        }
        let result = chain_client.void(claim.id, true).await?;
        (
            format!("Voided fund: {}", unclaimed_fund.token_addr),
            unclaimed_fund.sender,
            result,
        )
    } else {
        let unclaimed_state = chain_client.query_unclaimed_state(claim.id).await?;
        if unclaimed_state.expiry_time.as_u64() > u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim is not expired"));
        }
        let result = chain_client.void(claim.id, false).await?;
        (
            format!("Voided state: {}", unclaimed_state.extension_addr),
            unclaimed_state.sender,
            result,
        )
    };
    tx_sender
        .send(EmailMessage {
            to: claim.email_address.to_string(),
            email_args: EmailArgs::Void {
                user_email_addr: claim.email_address.to_string(),
                is_fund: claim.is_fund,
                void_msg: reply_msg,
            },
            account_key: None,
            wallet_addr: Some(ethers::utils::to_checksum(&sender, None)),
            tx_hash: Some(tx_hash),
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
