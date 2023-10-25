use crate::*;
use ethers::prelude::*;
use ethers::signers::Signer;
use std::str::FromStr;

#[derive(Default)]
pub struct AccountCreationInput {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) account_key_commit: [u8; 32],
    pub(crate) wallet_salt: [u8; 32],
    pub(crate) psi_point: Bytes,
    pub(crate) proof: Bytes,
}

#[derive(Default)]
pub struct AccountInitInput {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) email_domain: String,
    pub(crate) email_timestamp: U256,
    pub(crate) email_nullifier: [u8; 32],
    pub(crate) proof: Bytes,
}

type SignerM = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ChainClient {
    pub client: Arc<SignerM>,
    pub(crate) core: EmailWalletCore<SignerM>,
    pub(crate) token_registry: TokenRegistry<SignerM>,
    pub(crate) account_handler: AccountHandler<SignerM>,
    pub(crate) extension_handler: ExtensionHandler<SignerM>,
}

impl ChainClient {
    pub async fn setup() -> Result<Self> {
        let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
        let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(*CHAIN_ID.get().unwrap()),
        ));
        let core = EmailWalletCore::new(
            CORE_CONTRACT_ADDRESS.get().unwrap().parse::<Address>()?,
            client.clone(),
        );
        let token_registry_addr = core.token_registry().call().await.unwrap();
        let token_registry = TokenRegistry::new(token_registry_addr, client.clone());
        let account_handler_addr = core.account_handler().call().await.unwrap();
        let account_handler = AccountHandler::new(account_handler_addr, client.clone());
        let extension_handler = ExtensionHandler::new(
            core.extension_handler().call().await.unwrap(),
            client.clone(),
        );
        Ok(Self {
            client,
            core,
            token_registry,
            account_handler,
            extension_handler,
        })
    }

    pub async fn create_account(&self, data: AccountCreationInput) -> Result<String> {
        let call = self.account_handler.create_account(
            data.email_addr_pointer,
            data.account_key_commit,
            data.wallet_salt,
            data.psi_point,
            data.proof,
        );
        let tx = call.send().await?;
        let tx_hash = tx.tx_hash();
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn init_account(&self, data: AccountInitInput) -> Result<String> {
        let call = self.account_handler.initialize_account(
            data.email_addr_pointer,
            data.email_domain,
            data.email_timestamp,
            data.email_nullifier,
            data.proof,
        );
        let tx = call.send().await?;
        let tx_hash = tx.tx_hash();
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn handle_email_op(&self, email_op: EmailOp) -> Result<String> {
        let call = self.core.handle_email_op(email_op);
        let tx = call.send().await?;
        let tx_hash = tx.tx_hash();
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn query_user_erc20_balance(
        &self,
        wallet_salt: &WalletSalt,
        token_name: &str,
    ) -> Result<U256> {
        let token_addr = self
            .token_registry
            .get_token_address(token_name.to_string())
            .call()
            .await?;
        let erc20 = ERC20::new(token_addr, self.client.clone());
        let wallet_addr = self.get_wallet_addr_from_salt(wallet_salt.0).await?;
        let balance = erc20.balance_of(wallet_addr).call().await?;
        Ok(balance)
    }

    pub async fn query_decimals_of_erc20(&self, token_name: &str) -> Result<u8> {
        let token_addr = self
            .token_registry
            .get_token_address(token_name.to_string())
            .call()
            .await?;
        let erc20 = ERC20::new(token_addr, self.client.clone());
        let decimals = erc20.decimals().call().await?;
        Ok(decimals)
    }

    pub async fn query_user_extension_for_command(
        &self,
        wallet_salt: &WalletSalt,
        command: &str,
    ) -> Result<Address> {
        let wallet_addr = self.get_wallet_addr_from_salt(wallet_salt.0).await?;
        let extension_addr = self
            .extension_handler
            .get_extension_for_command(wallet_addr, command.to_string())
            .call()
            .await?;
        Ok(extension_addr)
    }

    pub async fn query_extension_templates_of_extension(
        &self,
        extension_addr: Address,
    ) -> Result<Vec<Vec<String>>> {
        let templates = self
            .extension_handler
            .get_subject_templates_of_extension(extension_addr)
            .call()
            .await?;
        Ok(templates)
    }

    pub async fn get_wallet_addr_from_salt(&self, wallet_salt: Fr) -> Result<Address> {
        let wallet_addr = self
            .account_handler
            .get_wallet_of_salt(wallet_salt.to_bytes())
            .call()
            .await?;
        Ok(wallet_addr)
    }
}

// pub(crate) async fn call_handle_email_op(email_op: EmailOp) -> Result<String> {
//     let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;

//     let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
//     let client = Arc::new(SignerMiddleware::new(
//         provider,
//         wallet.with_chain_id(*CHAIN_ID.get().unwrap()),
//     ));

//     let contract_address: Address = CORE_CONTRACT_ADDRESS.get().unwrap().parse()?;

//     let abi_source = "./packages/contracts/artifacts/EmailWalletCore.sol/EmailWalletCore.json";

//     // client.send_transaction(tx, block)

//     todo!()
// }

// pub(crate) async fn call_account_creation_op(data: AccountCreationInput) -> Result<String> {
//     let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;

//     let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
//     let client = SignerMiddleware::new(provider, wallet.with_chain_id(*CHAIN_ID.get().unwrap()));

//     // client.se

//     todo!()
// }

// pub(crate) async fn call_account_initialization_op(data: AccountInitInput) -> Result<String> {
//     todo!()
// }
