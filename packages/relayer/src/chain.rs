use crate::*;
use ethers::prelude::*;
use ethers::signers::Signer;

const CONFIRMATIONS: usize = 1;

#[derive(Default, Debug)]
pub struct AccountCreationInput {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) account_key_commit: [u8; 32],
    pub(crate) wallet_salt: [u8; 32],
    pub(crate) psi_point: Bytes,
    pub(crate) proof: Bytes,
}

#[derive(Default, Debug)]
pub struct AccountInitInput {
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) email_domain: String,
    pub(crate) email_timestamp: U256,
    pub(crate) email_nullifier: [u8; 32],
    pub(crate) proof: Bytes,
}

#[derive(Default, Debug)]
pub struct AccountTransportInput {
    pub(crate) old_account_key_commit: [u8; 32],
    pub(crate) new_email_addr_pointer: [u8; 32],
    pub(crate) new_account_key_commit: [u8; 32],
    pub(crate) new_psi_point: Bytes,
    pub(crate) transport_email_proof: EmailProof,
    pub(crate) account_creation_proof: Bytes,
}

#[derive(Default, Debug)]
pub struct RegisterUnclaimedFundInput {
    pub(crate) email_addr_commit: [u8; 32],
    pub(crate) token_addr: Address,
    pub(crate) amount: U256,
    pub(crate) expiry_time: U256,
    pub(crate) announce_commit_randomness: U256,
    pub(crate) announce_email_addr: String,
}

#[derive(Default, Debug)]
pub struct ClaimInput {
    pub(crate) email_addr_commit: [u8; 32],
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) is_fund: bool,
    pub(crate) proof: Bytes,
}

#[derive(Default, Debug)]
pub struct UnclaimedFund {
    pub(crate) email_addr_commit: Fr,
    pub(crate) sender: Address,
    pub(crate) token_addr: Address,
    pub(crate) amount: U256,
    pub(crate) expire_time: U256,
}

#[derive(Default, Debug)]
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

    pub fn self_eth_addr(&self) -> Address {
        self.client.address()
    }

    pub async fn register_relayer(
        &self,
        rand_hash: Fr,
        email_addr: String,
        hostname: String,
    ) -> Result<String> {
        let call =
            self.relayer_handler
                .register_relayer(fr_to_bytes32(&rand_hash)?, email_addr, hostname);
        let tx = call.send().await?;
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
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
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
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
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
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
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
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
            let receipt = tx
                .log()
                .confirmations(CONFIRMATIONS)
                .await?
                .ok_or(anyhow!("No receipt"))?;
            let tx_hash = receipt.transaction_hash;
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        } else {
            let call = self.unclaims_handler.claim_unclaimed_state(
                data.email_addr_commit,
                data.email_addr_pointer,
                data.proof,
            );
            let tx = call.send().await?;
            let receipt = tx
                .log()
                .confirmations(CONFIRMATIONS)
                .await?
                .ok_or(anyhow!("No receipt"))?;
            let tx_hash = receipt.transaction_hash;
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        }
    }

    pub async fn void(&self, email_addr_commit: [u8; 32], is_fund: bool) -> Result<String> {
        if is_fund {
            let call = self.unclaims_handler.void_unclaimed_fund(email_addr_commit);
            let tx = call.send().await?;
            let receipt = tx
                .log()
                .confirmations(CONFIRMATIONS)
                .await?
                .ok_or(anyhow!("No receipt"))?;
            let tx_hash = receipt.transaction_hash;
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        } else {
            let call = self
                .unclaims_handler
                .void_unclaimed_state(email_addr_commit);
            let tx = call.send().await?;
            let receipt = tx
                .log()
                .confirmations(CONFIRMATIONS)
                .await?
                .ok_or(anyhow!("No receipt"))?;
            let tx_hash = receipt.transaction_hash;
            let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
            Ok(tx_hash)
        }
    }

    pub async fn handle_email_op(&self, email_op: EmailOp) -> Result<String> {
        let value = if !email_op.has_email_recipient {
            U256::zero()
        } else if email_op.command == SEND_COMMAND {
            let gas = self.unclaims_handler.unclaimed_fund_claim_gas().await?;
            let fee = self.unclaims_handler.max_fee_per_gas().await?;
            gas * fee
        } else {
            let gas = self.unclaims_handler.unclaimed_state_claim_gas().await?;
            let fee = self.unclaims_handler.max_fee_per_gas().await?;
            gas * fee
        };
        let call = self.core.handle_email_op(email_op);
        let call = call.value(value);
        let tx = call.send().await?;
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    pub async fn query_account_key_commit(&self, pointer: &Fr) -> Result<Fr> {
        let account_key_commit = self
            .account_handler
            .account_key_commit_of_pointer(fr_to_bytes32(pointer)?)
            .await?;
        bytes32_to_fr(&account_key_commit)
    }

    pub async fn query_account_info(&self, account_key_commit: &Fr) -> Result<AccountKeyInfo> {
        let info = self
            .account_handler
            .get_info_of_account_key_commit(fr_to_bytes32(account_key_commit)?)
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
        let wallet_addr = self.get_wallet_addr_from_salt(&wallet_salt.0).await?;
        let balance = erc20.balance_of(wallet_addr).call().await?;
        Ok(balance)
    }

    pub async fn query_erc20_address(&self, token_name: &str) -> Result<Address> {
        let token_addr = self
            .token_registry
            .get_token_address(token_name.to_string())
            .call()
            .await?;
        Ok(token_addr)
    }

    pub async fn query_decimals_of_erc20(&self, token_name: &str) -> Result<u8> {
        let token_addr = self
            .token_registry
            .get_token_address(token_name.to_string())
            .call()
            .await?;
        self.query_decimals_of_erc20_address(token_addr).await
    }

    pub async fn query_decimals_of_erc20_address(&self, token_addr: Address) -> Result<u8> {
        let erc20 = ERC20::new(token_addr, self.client.clone());
        let decimals = erc20.decimals().call().await?;
        Ok(decimals)
    }

    pub async fn query_token_name(&self, token_addr: Address) -> Result<String> {
        let name = self
            .token_registry
            .get_token_name_of_address(token_addr)
            .call()
            .await?;
        Ok(name)
    }

    pub async fn query_relayer_rand_hash(&self, relayer: Address) -> Result<Fr> {
        let rand_hash = self.relayer_handler.get_rand_hash(relayer).call().await?;
        bytes32_to_fr(&rand_hash)
    }

    pub async fn query_user_extension_for_command(
        &self,
        wallet_salt: &WalletSalt,
        command: &str,
    ) -> Result<Address> {
        let wallet_addr = self.get_wallet_addr_from_salt(&wallet_salt.0).await?;
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

    pub async fn get_wallet_addr_from_salt(&self, wallet_salt: &Fr) -> Result<Address> {
        let wallet_addr = self
            .account_handler
            .get_wallet_of_salt(fr_to_bytes32(wallet_salt)?)
            .call()
            .await?;
        Ok(wallet_addr)
    }

    pub async fn query_rand_hash_of_relayer(&self, relayer: Address) -> Result<Fr> {
        let rand_hash = self.relayer_handler.get_rand_hash(relayer).call().await?;
        bytes32_to_fr(&rand_hash)
    }

    pub async fn query_ak_commit_and_relayer_of_wallet_salt(
        &self,
        wallet_salt: &WalletSalt,
    ) -> Result<(Vec<Fr>, Vec<Address>)> {
        let events: Vec<(AccountCreatedFilter, LogMeta)> = self
            .account_handler
            .event_for_name::<AccountCreatedFilter>("AccountCreated")?
            .from_block(0)
            .topic2(H256::from(fr_to_bytes32(&wallet_salt.0)?))
            .query_with_meta()
            .await?;
        let mut account_key_commits = vec![];
        let mut relayers = vec![];
        for (created, log_meta) in events {
            let account_key_commit = bytes32_to_fr(&created.account_key_commit)?;
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

    pub async fn query_unclaimed_fund(&self, email_addr_commit: [u8; 32]) -> Result<UnclaimedFund> {
        let (email_addr_commit, sender, token_addr, amount, expire_time) = self
            .unclaims_handler
            .unclaimed_fund_of_email_addr_commit(email_addr_commit)
            .await?;
        let unclaimed_fund = UnclaimedFund {
            email_addr_commit: bytes32_to_fr(&email_addr_commit)?,
            sender,
            token_addr,
            amount,
            expire_time,
        };
        Ok(unclaimed_fund)
    }

    pub async fn query_unclaimed_state(
        &self,
        email_addr_commit: [u8; 32],
    ) -> Result<UnclaimedState> {
        let (email_addr_commit, extension_addr, sender, state, expire_time) = self
            .unclaims_handler
            .unclaimed_state_of_email_addr_commit(email_addr_commit)
            .await?;
        let unclaimed_state = UnclaimedState {
            email_addr_commit: bytes32_to_fr(&email_addr_commit)?,
            extension_addr,
            sender,
            state,
            expire_time,
        };
        Ok(unclaimed_state)
    }

    pub async fn stream_unclaim_fund_registration<
        F: FnMut(UnclaimedFundRegisteredFilter, LogMeta) -> Result<()>,
    >(
        &self,
        from_block: U64,
        mut f: F,
    ) -> Result<U64> {
        let ev = self
            .unclaims_handler
            .event_for_name::<UnclaimedFundRegisteredFilter>("UnclaimedFundRegistered")?
            .from_block(from_block);
        let mut stream = ev.stream_with_meta().await?;
        let mut last_block = from_block;
        while let Some(Ok((event, meta))) = stream.next().await {
            last_block = meta.block_number;
            f(event, meta)?;
        }
        Ok(last_block)
    }

    pub async fn stream_unclaim_state_registration<
        F: FnMut(UnclaimedStateRegisteredFilter, LogMeta) -> Result<()>,
    >(
        &self,
        from_block: U64,
        mut f: F,
    ) -> Result<U64> {
        let ev = self
            .unclaims_handler
            .event_for_name::<UnclaimedStateRegisteredFilter>("UnclaimedStateRegistered")?
            .from_block(from_block);
        let mut stream = ev.stream_with_meta().await?;
        let mut last_block = from_block;
        while let Some(Ok((event, meta))) = stream.next().await {
            last_block = meta.block_number;
            f(event, meta)?;
        }
        Ok(last_block)
    }

    pub(crate) async fn check_if_point_registered(&self, point: Point) -> Result<bool> {
        let Point { x, y } = point;
        let x = hex2field(&x)?;
        let y = hex2field(&y)?;
        let x = U256::from_little_endian(&x.to_bytes());
        let y = U256::from_little_endian(&y.to_bytes());
        let res = self
            .account_handler
            .pointer_of_psi_point(get_psi_point_bytes(x, y))
            .call()
            .await?;
        let res = U256::from_little_endian(&res);
        Ok(res == U256::zero())
    }

    pub(crate) async fn check_if_account_initialized_by_account_key(
        &self,
        account_key: &str,
    ) -> Result<bool> {
        todo!()
    }

    pub(crate) async fn check_if_account_initialized_by_point(&self, point: Point) -> Result<bool> {
        let Point { x, y } = point;
        let x = hex2field(&x)?;
        let y = hex2field(&y)?;
        let x = U256::from_little_endian(&x.to_bytes());
        let y = U256::from_little_endian(&y.to_bytes());
        let pointer = self
            .account_handler
            .pointer_of_psi_point(get_psi_point_bytes(x, y))
            .call()
            .await?;
        let account_key_commitment = self
            .account_handler
            .account_key_commit_of_pointer(pointer)
            .call()
            .await?;
        let account_key_info = self
            .account_handler
            .info_of_account_key_commit(account_key_commitment)
            .call()
            .await?;

        Ok(account_key_info.1 == true)
    }

    pub(crate) async fn get_relayers(&self) -> Result<Vec<String>> {
        // TODO: iteration over relayers
        Ok(vec![])
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
