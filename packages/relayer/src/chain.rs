use crate::*;

use ethers::prelude::*;

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

#[derive(Default)]
pub(crate) struct AccountCreation {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) account_key_commit: [u8; 32],
    pub(crate) wallet_salt: [u8; 32],
    pub(crate) psi_point: Bytes,
    pub(crate) proof: Bytes,
}

#[derive(Default)]
pub(crate) struct AccountInitialization {}

pub(crate) async fn call_handle_email_op(email_op: EmailOp) -> Result<String> {
    todo!()
}

pub(crate) async fn call_account_creation_op(data: AccountCreation) -> Result<String> {
    todo!()
}

pub(crate) async fn call_account_initialization_op(data: AccountInitialization) -> Result<String> {
    todo!()
}
