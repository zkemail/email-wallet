use crate::*;

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
pub(crate) struct AccountInitialization {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) email_domain: String,
    pub(crate) email_timestamp: U256,
    pub(crate) email_nullifier: [u8; 32],
    pub(crate) proof: Bytes,
}

pub(crate) async fn call_handle_email_op(email_op: EmailOp) -> Result<String> {
    let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;

    let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(*CHAIN_ID.get().unwrap()),
    ));

    let contract_address: Address = CORE_CONTRACT_ADDRESS.get().unwrap().parse()?;

    let abi_source = "./packages/contracts/artifacts/EmailWalletCore.sol/EmailWalletCore.json";

    // client.send_transaction(tx, block)

    todo!()
}

pub(crate) async fn call_account_creation_op(data: AccountCreation) -> Result<String> {
    let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;

    let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(*CHAIN_ID.get().unwrap()));

    // client.se

    todo!()
}

pub(crate) async fn call_account_initialization_op(data: AccountInitialization) -> Result<String> {
    todo!()
}
