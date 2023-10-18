#![allow(clippy::upper_case_acronyms)]

use crate::*;

use ethers::types::{Address, Bytes, U256};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) struct WalletParams {
    pub(crate) token_name: String,
    pub(crate) amount: U256,
}

pub(crate) struct ExtensionParams {
    pub(crate) subject_template_index: u8,
    pub(crate) subject_params: Bytes,
}

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

    db.remove_email(&email).await?;

    Ok(())
}

pub(crate) async fn generate_proof(input: &str) -> Result<String> {
    todo!()
}
