use std::str::FromStr;

use crate::*;
use ethers::abi::RawLog;
use ethers::middleware::Middleware;
use ethers::prelude::*;
use ethers::signers::Signer;

const CONFIRMATIONS: usize = 1;

#[derive(Default, Debug)]
pub struct AccountCreationInput {
    pub wallet_salt: [u8; 32],
    pub psi_point: Bytes,
    pub proof: EmailProof,
}

#[derive(Default, Debug)]
pub struct RegisterUnclaimedFundInput {
    pub email_addr_commit: [u8; 32],
    pub token_addr: Address,
    pub amount: U256,
    pub expiry_time: U256,
    pub announce_commit_randomness: U256,
    pub announce_email_addr: String,
}

#[derive(Default, Debug)]
pub struct ClaimInput {
    pub id: U256,
    pub recipient_wallet_salt: [u8; 32],
    pub is_fund: bool,
    pub proof: Bytes,
}

type SignerM = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ChainClient {
    pub client: Arc<SignerM>,
    pub core: EmailWalletCore<SignerM>,
    pub token_registry: TokenRegistry<SignerM>,
    pub account_handler: AccountHandler<SignerM>,
    pub extension_handler: ExtensionHandler<SignerM>,
    pub relayer_handler: RelayerHandler<SignerM>,
    pub unclaims_handler: UnclaimsHandler<SignerM>,
    pub ecdsa_owned_dkim_registry: ECDSAOwnedDKIMRegistry<SignerM>,
    pub test_erc20: TestERC20<SignerM>,
    pub nft_extension: NFTExtension<SignerM>,
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
        let nft_extension = NFTExtension::new(
            extension_handler
                .default_extension_of_command("NFT".to_string())
                .call()
                .await?,
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
            nft_extension,
        })
    }

    pub fn self_eth_addr(&self) -> Address {
        self.client.address()
    }

    pub async fn register_relayer(&self, email_addr: String, hostname: String) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.relayer_handler.register_relayer(email_addr, hostname);
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

    pub async fn register_psi_point(
        &self,
        point: &Point,
        wallet_salt: &WalletSalt,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.account_handler.register_psi_point(
            get_psi_point_bytes(
                U256::from_str_radix(&point.x, 10)?,
                U256::from_str_radix(&point.y, 10)?,
            ),
            fr_to_bytes32(&wallet_salt.0)?,
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

    pub async fn create_account(&self, data: AccountCreationInput) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call =
            self.account_handler
                .create_account(data.wallet_salt, data.psi_point, data.proof);
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
                data.recipient_wallet_salt,
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
                data.recipient_wallet_salt,
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

    pub async fn register_unclaimed_fund(
        &self,
        email_addr_commit: Fr,
        token_addr: Address,
        amount: U256,
        expiry_time: U256,
        announce_commit_randomness: Option<U256>,
        announce_email_addr: Option<String>,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.unclaims_handler.register_unclaimed_fund(
            fr_to_bytes32(&email_addr_commit)?,
            token_addr,
            amount,
            expiry_time,
            announce_commit_randomness.unwrap_or(U256::zero()),
            announce_email_addr.unwrap_or(String::new()),
        );
        let fee = {
            let gas = self.unclaims_handler.unclaimed_fund_claim_gas().await?;
            let fee = self.unclaims_handler.max_fee_per_gas().await?;
            gas * fee
        };
        let call = call.value(fee);
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

    pub async fn register_unclaimed_state(
        &self,
        email_addr_commit: Fr,
        extension_addr: Address,
        state: Bytes,
        expiry_time: U256,
        announce_commit_randomness: Option<U256>,
        announce_email_addr: Option<String>,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.unclaims_handler.register_unclaimed_state(
            fr_to_bytes32(&email_addr_commit)?,
            extension_addr,
            state,
            expiry_time,
            announce_commit_randomness.unwrap_or(U256::zero()),
            announce_email_addr.unwrap_or(String::new()),
        );
        let fee = {
            let gas = self.unclaims_handler.unclaimed_state_claim_gas().await?;
            let fee = self.unclaims_handler.max_fee_per_gas().await?;
            gas * fee
        };
        let call = call.value(fee);
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

    pub async fn transfer_onboarding_tokens(&self, wallet_addr: H160) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let erc20 = ERC20::new(
            ONBOARDING_TOKEN_ADDR.get().unwrap().to_owned(),
            self.client.clone(),
        );
        let call = erc20.transfer(
            wallet_addr,
            ONBOARDING_TOKEN_AMOUNT.get().unwrap().to_owned(),
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

    pub async fn approve_erc721(
        &self,
        token_addr: Address,
        to: Address,
        token_id: U256,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let erc721 = ERC721::new(token_addr, self.client.clone());
        let call = erc721.approve(to, token_id);
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

    pub async fn query_default_extension_for_command(&self, command: &str) -> Result<Address> {
        let extension_addr = self
            .extension_handler
            .default_extension_of_command(command.to_string())
            .call()
            .await?;
        Ok(extension_addr)
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

    pub async fn query_nft_name_of_address(&self, nft_addr: Address) -> Result<String> {
        let name = self
            .nft_extension
            .name_of_nft_address(nft_addr)
            .call()
            .await?;
        Ok(name)
    }

    pub async fn query_erc721_owner_of_token(
        &self,
        nft_addr: Address,
        token_id: U256,
    ) -> Result<Address> {
        let erc721 = ERC721::new(nft_addr, self.client.clone());
        let owner = erc721.owner_of(token_id).call().await?;
        Ok(owner)
    }

    pub async fn query_erc721_token_uri_of_token(
        &self,
        nft_addr: Address,
        token_id: U256,
    ) -> Result<String> {
        let erc721 = ERC721::new(nft_addr, self.client.clone());
        let uri = erc721.token_uri(token_id).call().await?;
        Ok(uri)
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

    pub async fn check_if_point_registered(&self, point: Point) -> Result<bool> {
        let Point { x, y } = point;
        let x = hex2field(&x)?;
        let y = hex2field(&y)?;
        let x = U256::from_little_endian(&x.to_bytes());
        let y = U256::from_little_endian(&y.to_bytes());
        let wallet_salt = self
            .account_handler
            .wallet_salt_of_psi_point(get_psi_point_bytes(x, y))
            .call()
            .await?;
        let wallet_salt = U256::from_little_endian(&wallet_salt);
        Ok(wallet_salt != U256::zero())
    }

    pub async fn check_if_account_created_by_point(&self, point: Point) -> Result<bool> {
        let Point { x, y } = point;
        let x = hex2field(&x)?;
        let y = hex2field(&y)?;
        let x = U256::from_little_endian(&x.to_bytes());
        let y = U256::from_little_endian(&y.to_bytes());
        let wallet_salt = self
            .account_handler
            .wallet_salt_of_psi_point(get_psi_point_bytes(x, y))
            .call()
            .await?;
        let is_deployed = self
            .account_handler
            .is_wallet_salt_deployed(wallet_salt)
            .call()
            .await?;
        Ok(is_deployed)
    }

    pub async fn check_if_account_created_by_account_key(
        &self,
        email_addr: &str,
        account_key: &str,
    ) -> Result<bool> {
        let account_key = AccountKey(hex2field(account_key)?);
        let padded_email_addr = PaddedEmailAddr::from_email_addr(email_addr);
        let wallet_salt = WalletSalt::new(&padded_email_addr, account_key)?;
        let is_deployed = self
            .account_handler
            .is_wallet_salt_deployed(fr_to_bytes32(&wallet_salt.0)?)
            .call()
            .await?;
        Ok(is_deployed)
    }

    #[named]
    pub async fn check_if_dkim_public_key_hash_valid(
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

    pub async fn get_latest_block_number(&self) -> U64 {
        self.client.get_block_number().await.unwrap()
    }
}
