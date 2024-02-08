use std::str::FromStr;

use crate::shared;
use shared::SHARED_MUTEX;

use crate::*;
use ethers::abi::RawLog;
use ethers::middleware::Middleware;
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
    pub(crate) dkim_public_key_hash: [u8; 32],
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
    pub(crate) id: U256,
    pub(crate) email_addr_pointer: [u8; 32],
    pub(crate) is_fund: bool,
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
    pub(crate) relayer_handler: RelayerHandler<SignerM>,
    pub(crate) unclaims_handler: UnclaimsHandler<SignerM>,
    pub(crate) ecdsa_owned_dkim_registry: ECDSAOwnedDKIMRegistry<SignerM>,
    pub(crate) test_erc20: TestERC20<SignerM>,
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
        let extension_handler =
            ExtensionHandler::new(core.extension_handler().call().await?, client.clone());
        let relayer_handler =
            RelayerHandler::new(core.relayer_handler().call().await.unwrap(), client.clone());
        let unclaims_handler =
            UnclaimsHandler::new(core.unclaims_handler().call().await?, client.clone());
        let ecdsa_owned_dkim_registry = ECDSAOwnedDKIMRegistry::new(
            account_handler.default_dkim_registry().await?,
            client.clone(),
        );
        let test_erc20 = TestERC20::new(
            token_registry.get_token_address("TEST".to_string()).await?,
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
            ecdsa_owned_dkim_registry,
            test_erc20,
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
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

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
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

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
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.account_handler.initialize_account(
            data.email_addr_pointer,
            data.email_domain,
            data.email_timestamp,
            data.email_nullifier,
            data.dkim_public_key_hash,
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
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

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
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        if data.is_fund {
            let call = self.unclaims_handler.claim_unclaimed_fund(
                data.id,
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
                data.id,
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

    pub async fn void(&self, id: U256, is_fund: bool) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        if is_fund {
            let call = self.unclaims_handler.void_unclaimed_fund(id);
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
            let call = self.unclaims_handler.void_unclaimed_state(id);
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

    #[named]
    pub async fn handle_email_op(&self, email_op: EmailOp) -> Result<(String, U256)> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

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
        for log in receipt.logs.into_iter() {
            if let Ok(decoded) = EmailWalletEventsEvents::decode_log(&RawLog::from(log)) {
                match decoded {
                    EmailWalletEventsEvents::EmailOpHandledFilter(event) => {
                        info!(LOG, "event {:?}", event; "func" => function_name!());
                        return Ok((tx_hash, event.registered_unclaim_id));
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        Err(anyhow!("no EmailOpHandled event found in the receipt"))
    }

    pub async fn set_dkim_public_key_hash(
        &self,
        selector: String,
        domain_name: String,
        public_key_hash: [u8; 32],
        signature: Bytes,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.ecdsa_owned_dkim_registry.set_dkim_public_key_hash(
            selector,
            domain_name,
            public_key_hash,
            signature,
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

    pub async fn free_mint_test_erc20(&self, wallet_addr: Address, amount: U256) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.test_erc20.free_mint_with_to(wallet_addr, amount);
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

    pub(crate) async fn transfer_onboarding_tokens(&self, wallet_addr: H160) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let erc20 = ERC20::new(
            ONBOARDING_TOKEN_ADDR.get().unwrap().to_owned(),
            self.client.clone(),
        );
        let tx = erc20.transfer(
            wallet_addr,
            ONBOARDING_TOKEN_AMOUNT.get().unwrap().to_owned(),
        );
        let tx = tx.send().await?;

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

    // pub async fn query_ak_commit_and_relayer_of_wallet_salt(
    //     &self,
    //     wallet_salt: &WalletSalt,
    // ) -> Result<(Vec<Fr>, Vec<Address>)> {
    //     let events: Vec<(email_wallet_events::AccountCreatedFilter, LogMeta)> = self
    //         .account_handler
    //         .event_for_name::<email_wallet_events::AccountCreatedFilter>("AccountCreated")?
    //         .from_block(0)
    //         .topic2(H256::from(fr_to_bytes32(&wallet_salt.0)?))
    //         .query_with_meta()
    //         .await?;
    //     let mut account_key_commits = vec![];
    //     let mut relayers = vec![];
    //     for (created, log_meta) in events {
    //         let account_key_commit = bytes32_to_fr(&created.account_key_commit)?;
    //         account_key_commits.push(account_key_commit);
    //         let tx_hash = log_meta.transaction_hash;
    //         let tx = self.client.get_transaction(tx_hash).await?;
    //         if let Some(tx) = tx {
    //             let relayer = tx.from;
    //             relayers.push(relayer);
    //         }
    //     }
    //     Ok((account_key_commits, relayers))
    // }

    pub async fn query_unclaimed_fund(&self, id: U256) -> Result<UnclaimedFund> {
        let unclaimed_fund = self.unclaims_handler.get_unclaimed_fund(id).await?;
        Ok(unclaimed_fund)
    }

    pub async fn query_unclaimed_state(&self, id: U256) -> Result<UnclaimedState> {
        let unclaimed_state = self.unclaims_handler.get_unclaimed_state(id).await?;
        Ok(unclaimed_state)
    }

    #[named]
    pub async fn get_unclaim_id_from_tx_hash(&self, tx_hash: &str, is_fund: bool) -> Result<U256> {
        let receipt: TransactionReceipt = self
            .client
            .get_transaction_receipt(H256::from_str(tx_hash)?)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        info!(LOG, "receipt {:?}", receipt; "func" => function_name!());

        for log in receipt.logs.into_iter() {
            info!(LOG, "log {:?}", log; "func" => function_name!());
            if let Ok(decoded) = EmailWalletEventsEvents::decode_log(&RawLog::from(log)) {
                info!(LOG, "decoded {:?}", decoded; "func" => function_name!());
                match decoded {
                    EmailWalletEventsEvents::UnclaimedFundRegisteredFilter(event) => {
                        if !is_fund {
                            return Err(anyhow!(
                                "the transaction does not register an unclaimed fund"
                            ));
                        }
                        return Ok(event.id);
                    }
                    EmailWalletEventsEvents::UnclaimedStateRegisteredFilter(event) => {
                        if is_fund {
                            return Err(anyhow!(
                                "the transaction does not register an unclaimed state"
                            ));
                        }
                        return Ok(event.id);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        Err(anyhow!(
            "the transaction registers neither an unclaim fund nor state"
        ))
    }

    pub async fn validate_email_op(&self, email_op: EmailOp) -> Result<()> {
        let call = self.core.validate_email_op(email_op);
        call.call().await?;
        Ok(())
    }

    pub async fn stream_unclaim_fund_registration<
        F: FnMut(email_wallet_events::UnclaimedFundRegisteredFilter, LogMeta) -> Result<()>,
    >(
        &self,
        from_block: U64,
        mut f: F,
    ) -> Result<U64> {
        let ev = self
            .unclaims_handler
            .event_for_name::<email_wallet_events::UnclaimedFundRegisteredFilter>(
                "UnclaimedFundRegistered",
            )?
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
        F: FnMut(email_wallet_events::UnclaimedStateRegisteredFilter, LogMeta) -> Result<()>,
    >(
        &self,
        from_block: U64,
        mut f: F,
    ) -> Result<U64> {
        let ev = self
            .unclaims_handler
            .event_for_name::<email_wallet_events::UnclaimedStateRegisteredFilter>(
                "UnclaimedStateRegistered",
            )?
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
        email_addr: &str,
        account_key: &str,
    ) -> Result<bool> {
        let account_key = AccountKey(hex2field(account_key)?);
        let padded_email_addr = PaddedEmailAddr::from_email_addr(email_addr);
        let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap())?);
        let account_key_commitment =
            account_key.to_commitment(&padded_email_addr, &relayer_rand.hash()?)?;

        let account_key_info = self
            .account_handler
            .info_of_account_key_commit(Fr::to_bytes(&account_key_commitment))
            .call()
            .await?;

        Ok(account_key_info.1)
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

        Ok(account_key_info.1)
    }

    #[named]
    pub(crate) async fn check_if_dkim_public_key_hash_valid(
        &self,
        domain_name: ::std::string::String,
        public_key_hash: [u8; 32],
    ) -> Result<bool> {
        let is_valid = self
            .ecdsa_owned_dkim_registry
            .is_dkim_public_key_hash_valid(domain_name.clone(), public_key_hash)
            .call()
            .await?;
        info!(
            LOG,
            "{:?} for {} is already registered: {}", public_key_hash, domain_name, is_valid; "func" => function_name!()
        );
        Ok(is_valid)
    }
}
