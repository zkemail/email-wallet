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

#[derive(Default)]
pub struct AccountTransportInput {
    pub(crate) old_account_key_commit: [u8; 32],
    pub(crate) new_email_addr_pointer: [u8; 32],
    pub(crate) new_account_key_commit: [u8; 32],
    pub(crate) new_psi_point: Bytes,
    pub(crate) transport_email_proof: EmailProof,
    pub(crate) account_creation_proof: Bytes,
}

#[derive(Default)]
pub struct ClaimInput {
    pub(crate) email_addr_commit: [u8; 32],
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) is_fund: bool,
    pub(crate) proof: Bytes,
}

#[derive(Default)]
pub struct UnclaimedFund {
    pub(crate) email_addr_commit: Fr,
    pub(crate) sender: Address,
    pub(crate) token_addr: Address,
    pub(crate) amount: U256,
    pub(crate) expire_time: U256,
}

#[derive(Default)]
pub struct UnclaimedState {
    pub(crate) email_addr_commit: Fr,
    pub(crate) extension_addr: Address,
    pub(crate) sender: Address,
    pub(crate) state: Bytes,
    pub(crate) expire_time: U256,
}

type SignerM = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ChainClient {
    pub client: Arc<SignerM>,
    pub(crate) core: EmailWalletCore<SignerM>,
    pub(crate) token_registry: TokenRegistry<SignerM>,
    pub(crate) account_handler: AccountHandler<SignerM>,
    pub(crate) extension_handler: ExtensionHandler<SignerM>,
    pub(crate) relayer_handler: RelayerHandler<SignerM>,
    pub(crate) unclaims_handler: UnclaimsHandler<SignerM>,
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
        let relayer_handler =
            RelayerHandler::new(core.relayer_handler().call().await.unwrap(), client.clone());
        let unclaims_handler = UnclaimsHandler::new(
            core.unclaims_handler().call().await.unwrap(),
            client.clone(),
        );
        Ok(Self {
            client,
            core,
            token_registry,
            account_handler,
            extension_handler,
            relayer_handler,
            unclaims_handler,
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

    pub async fn transport_account(&self, data: AccountTransportInput) -> Result<String> {
        let call = self.account_handler.transport_account(
            data.old_account_key_commit,
            data.new_email_addr_pointer,
            data.new_account_key_commit,
            data.new_psi_point,
            data.transport_email_proof,
            data.account_creation_proof,
        );
        let tx = call.send().await?;
        let tx_hash = tx.tx_hash();
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn claim(&self, data: ClaimInput) -> Result<String> {
        if data.is_fund {
            let call = self.unclaims_handler.claim_unclaimed_fund(
                data.email_addr_commit,
                data.email_addr_pointer,
                data.proof,
            );
            let tx = call.send().await?;
            let tx_hash = tx.tx_hash();
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        } else {
            let call = self.unclaims_handler.claim_unclaimed_state(
                data.email_addr_commit,
                data.email_addr_pointer,
                data.proof,
            );
            let tx = call.send().await?;
            let tx_hash = tx.tx_hash();
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        }
    }

    pub async fn handle_email_op(&self, email_op: EmailOp) -> Result<String> {
        let call = self.core.handle_email_op(email_op);
        let tx = call.send().await?;
        let tx_hash = tx.tx_hash();
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn query_account_key_commit(&self, pointer: &Fr) -> Result<Fr> {
        let account_key_commit = self
            .account_handler
            .account_key_commit_of_pointer(pointer.to_bytes())
            .await?;
        Ok(Fr::from_bytes(&account_key_commit).expect("account_key_commit is not 32 bytes"))
    }

    pub async fn query_account_info(&self, account_key_commit: &Fr) -> Result<AccountKeyInfo> {
        let info = self
            .account_handler
            .get_info_of_account_key_commit(account_key_commit.to_bytes())
            .await?;
        Ok(info)
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

    pub async fn query_subject_templates_of_extension(
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

    pub async fn query_rand_hash_of_relayer(&self, relayer: Address) -> Result<Fr> {
        let rand_hash = self.relayer_handler.get_rand_hash(relayer).call().await?;
        Ok(Fr::from_bytes(&rand_hash).expect("rand_hash is not 32 bytes"))
    }

    pub async fn query_ak_commit_and_relayer_of_wallet_salt(
        &self,
        wallet_salt: &WalletSalt,
    ) -> Result<(Vec<Fr>, Vec<Address>)> {
        let events: Vec<(AccountCreatedFilter, LogMeta)> = self
            .account_handler
            .event_for_name::<AccountCreatedFilter>("AccountCreated")?
            .from_block(0)
            .topic2(H256::from(wallet_salt.0.to_bytes()))
            .query_with_meta()
            .await?;
        let mut account_key_commits = vec![];
        let mut relayers = vec![];
        for (created, log_meta) in events {
            let account_key_commit = Fr::from_bytes(&created.account_key_commit)
                .expect("account_key_commit in the event is not 32 bytes");
            account_key_commits.push(account_key_commit);
            let tx_hash = log_meta.transaction_hash;
            let tx = self.client.get_transaction(tx_hash).await?;
            if let Some(tx) = tx {
                let relayer = tx.from;
                relayers.push(relayer);
            }
        }
        Ok((account_key_commits, relayers))
    }

    pub async fn query_unclaimed_fund(&self, email_addr_commit: &Fr) -> Result<UnclaimedFund> {
        let (email_addr_commit, sender, token_addr, amount, expire_time) = self
            .unclaims_handler
            .unclaimed_fund_of_email_addr_commit(email_addr_commit.to_bytes())
            .await?;
        let unclaimed_fund = UnclaimedFund {
            email_addr_commit: Fr::from_bytes(&email_addr_commit)
                .expect("email_addr_commit is not 32 bytes"),
            sender,
            token_addr,
            amount,
            expire_time,
        };
        Ok(unclaimed_fund)
    }

    pub async fn query_unclaimed_state(&self, email_addr_commit: &Fr) -> Result<UnclaimedState> {
        let (email_addr_commit, extension_addr, sender, state, expire_time) = self
            .unclaims_handler
            .unclaimed_state_of_email_addr_commit(email_addr_commit.to_bytes())
            .await?;
        let unclaimed_state = UnclaimedState {
            email_addr_commit: Fr::from_bytes(&email_addr_commit)
                .expect("email_addr_commit is not 32 bytes"),
            extension_addr,
            sender,
            state,
            expire_time,
        };
        Ok(unclaimed_state)
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
