#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub mod abis;
pub mod chain;
pub mod config;
pub mod core;
pub mod database;
pub mod modules;
pub mod utils;

pub use abis::*;
pub use chain::*;
pub use config::*;
pub use core::*;
pub use database::*;
pub use modules::*;
use std::future::Future;
use std::pin::Pin;
pub use utils::*;

use ::function_name::named;
use rand::rngs::OsRng;
use tokio::sync::Mutex;

use anyhow::{anyhow, bail, Result};
use dotenv::dotenv;
use ethers::prelude::*;
use lazy_static::lazy_static;
use relayer_utils::{converters::*, cryptos::*, Fr, LOG};
use slog::{error, info, trace};
use std::env;
use std::path::PathBuf;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, OnceLock};
use tokio::time::{sleep, Duration};

pub static CIRCUITS_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();
pub static WEB_SERVER_ADDRESS: OnceLock<String> = OnceLock::new();
pub static RELAYER_RAND: OnceLock<String> = OnceLock::new();
pub static PROVER_ADDRESS: OnceLock<String> = OnceLock::new();
pub static PRIVATE_KEY: OnceLock<String> = OnceLock::new();
pub static CHAIN_ID: OnceLock<u32> = OnceLock::new();
pub static CHAIN_RPC_PROVIDER: OnceLock<String> = OnceLock::new();
pub static CHAIN_RPC_EXPLORER: OnceLock<String> = OnceLock::new();
pub static CORE_CONTRACT_ADDRESS: OnceLock<String> = OnceLock::new();
pub static FEE_PER_GAS: OnceLock<U256> = OnceLock::new();
pub static INPUT_FILES_DIR: OnceLock<String> = OnceLock::new();
pub static EMAIL_TEMPLATES: OnceLock<String> = OnceLock::new();
pub static SUBGRAPH_URL: OnceLock<String> = OnceLock::new();
pub static ONBOARDING_TOKEN_ADDR: OnceLock<H160> = OnceLock::new();
pub static ONBOARDING_TOKEN_DISTRIBUTION_LIMIT: OnceLock<u32> = OnceLock::new();
pub static ONBOARDING_TOKEN_AMOUNT: OnceLock<U256> = OnceLock::new();
pub static ONBOARDING_COUNTER: AtomicU32 = AtomicU32::new(1);
pub static ONBOARDING_REPLY_MSG: OnceLock<String> = OnceLock::new();
pub static RELAYER_EMAIL_ADDRESS: OnceLock<String> = OnceLock::new();
pub static SAFE_API_ENDPOINT: OnceLock<String> = OnceLock::new();
pub static SMTP_SERVER: OnceLock<String> = OnceLock::new();
pub static ERROR_EMAIL_ADDRESSES: OnceLock<Vec<String>> = OnceLock::new();

lazy_static! {
    pub static ref DB: Arc<Database> = {
        dotenv().ok();
        let db = tokio::task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))
                .and_then(|rt| {
                    let database_path = env::var(DATABASE_PATH_KEY)
                        .map_err(|e| format!("DATABASE_PATH environment variable not set: {}", e))?;
                    rt.block_on(Database::open(&database_path))
                        .map_err(|e| format!("Failed to open database: {}", e))
                })
        })
        .unwrap_or_else(|e| {
            eprintln!("Critical error during database initialization: {}", e);
            std::process::exit(1);
        });
        Arc::new(db)
    };
    pub static ref CLIENT: Arc<ChainClient> = {
        dotenv().ok();
        let client = tokio::task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))
                .and_then(|rt| {
                    rt.block_on(ChainClient::setup())
                        .map_err(|e| format!("Failed to setup chain client: {}", e))
                })
        })
        .unwrap_or_else(|e| {
            eprintln!("Critical error during chain client initialization: {}", e);
            std::process::exit(1);
        });
        Arc::new(client)
    };
    pub static ref SHARED_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

pub async fn setup() -> Result<()> {
    dotenv().ok();
    
    let private_key = env::var(PRIVATE_KEY_KEY)
        .map_err(|e| anyhow!("PRIVATE_KEY environment variable not set: {}", e))?;
    PRIVATE_KEY.set(private_key)
        .map_err(|_| anyhow!("Failed to set PRIVATE_KEY"))?;
    
    let chain_id: u64 = env::var(CHAIN_ID_KEY)
        .map_err(|e| anyhow!("CHAIN_ID environment variable not set: {}", e))?
        .parse()
        .map_err(|e| anyhow!("Failed to parse CHAIN_ID: {}", e))?;
    CHAIN_ID.set(chain_id)
        .map_err(|_| anyhow!("Failed to set CHAIN_ID"))?;
    
    let chain_rpc_provider = env::var(CHAIN_RPC_PROVIDER_KEY)
        .map_err(|e| anyhow!("CHAIN_RPC_PROVIDER environment variable not set: {}", e))?;
    CHAIN_RPC_PROVIDER.set(chain_rpc_provider)
        .map_err(|_| anyhow!("Failed to set CHAIN_RPC_PROVIDER"))?;
    
    let chain_rpc_explorer = env::var(CHAIN_RPC_EXPLORER_KEY)
        .map_err(|e| anyhow!("CHAIN_RPC_EXPLORER environment variable not set: {}", e))?;
    CHAIN_RPC_EXPLORER.set(chain_rpc_explorer)
        .map_err(|_| anyhow!("Failed to set CHAIN_RPC_EXPLORER"))?;
    
    let core_contract_address = env::var(CORE_CONTRACT_ADDRESS_KEY)
        .map_err(|e| anyhow!("CORE_CONTRACT_ADDRESS environment variable not set: {}", e))?;
    CORE_CONTRACT_ADDRESS.set(core_contract_address)
        .map_err(|_| anyhow!("Failed to set CORE_CONTRACT_ADDRESS"))?;

    let private_key_for_rand = PRIVATE_KEY.get()
        .ok_or_else(|| anyhow!("PRIVATE_KEY not initialized"))?;
    let relayer_rand = derive_relayer_rand(private_key_for_rand)?;

    let client = ChainClient::setup().await?;
    
    let relayer_email = env::var(RELAYER_EMAIL_ADDR_KEY)
        .map_err(|e| anyhow!("RELAYER_EMAIL_ADDR environment variable not set: {}", e))?;
    let relayer_hostname = env::var(RELAYER_HOSTNAME_KEY)
        .map_err(|e| anyhow!("RELAYER_HOSTNAME environment variable not set: {}", e))?;
    
    let tx_hash = client
        .register_relayer(relayer_email, relayer_hostname)
        .await?;
    println!("Register relayer in {}", tx_hash);
    Ok(())
}

#[named]
pub async fn run(config: RelayerConfig) -> Result<()> {
    info!(LOG, "Starting relayer"; "func" => function_name!());

    CIRCUITS_DIR_PATH.set(config.circuits_dir_path)
        .map_err(|_| anyhow!("Failed to set CIRCUITS_DIR_PATH"))?;
    WEB_SERVER_ADDRESS.set(config.web_server_address)
        .map_err(|_| anyhow!("Failed to set WEB_SERVER_ADDRESS"))?;
    PROVER_ADDRESS.set(config.prover_address)
        .map_err(|_| anyhow!("Failed to set PROVER_ADDRESS"))?;
    PRIVATE_KEY.set(config.private_key)
        .map_err(|_| anyhow!("Failed to set PRIVATE_KEY"))?;
    CHAIN_ID.set(config.chain_id)
        .map_err(|_| anyhow!("Failed to set CHAIN_ID"))?;
    CHAIN_RPC_PROVIDER.set(config.chain_rpc_provider)
        .map_err(|_| anyhow!("Failed to set CHAIN_RPC_PROVIDER"))?;
    CHAIN_RPC_EXPLORER.set(config.chain_rpc_explorer)
        .map_err(|_| anyhow!("Failed to set CHAIN_RPC_EXPLORER"))?;
    CORE_CONTRACT_ADDRESS.set(config.core_contract_address)
        .map_err(|_| anyhow!("Failed to set CORE_CONTRACT_ADDRESS"))?;
    FEE_PER_GAS.set(config.fee_per_gas)
        .map_err(|_| anyhow!("Failed to set FEE_PER_GAS"))?;
    INPUT_FILES_DIR.set(config.input_files_dir)
        .map_err(|_| anyhow!("Failed to set INPUT_FILES_DIR"))?;
    EMAIL_TEMPLATES.set(config.email_templates)
        .map_err(|_| anyhow!("Failed to set EMAIL_TEMPLATES"))?;
    SUBGRAPH_URL.set(config.subgraph_url)
        .map_err(|_| anyhow!("Failed to set SUBGRAPH_URL"))?;
    ONBOARDING_TOKEN_ADDR.set(config.onboarding_token_addr)
        .map_err(|_| anyhow!("Failed to set ONBOARDING_TOKEN_ADDR"))?;
    ONBOARDING_TOKEN_DISTRIBUTION_LIMIT.set(config.onboarding_token_distribution_limit)
        .map_err(|_| anyhow!("Failed to set ONBOARDING_TOKEN_DISTRIBUTION_LIMIT"))?;
    ONBOARDING_TOKEN_AMOUNT.set(config.onboarding_token_amount)
        .map_err(|_| anyhow!("Failed to set ONBOARDING_TOKEN_AMOUNT"))?;
    ONBOARDING_REPLY_MSG.set(config.onboarding_reply_msg)
        .map_err(|_| anyhow!("Failed to set ONBOARDING_REPLY_MSG"))?;
    RELAYER_EMAIL_ADDRESS.set(config.relayer_email_addr)
        .map_err(|_| anyhow!("Failed to set RELAYER_EMAIL_ADDRESS"))?;
    SAFE_API_ENDPOINT.set(config.safe_api_endpoint)
        .map_err(|_| anyhow!("Failed to set SAFE_API_ENDPOINT"))?;
    SMTP_SERVER.set(config.smtp_server)
        .map_err(|_| anyhow!("Failed to set SMTP_SERVER"))?;
    ERROR_EMAIL_ADDRESSES.set(config.error_email_addresses)
        .map_err(|_| anyhow!("Failed to set ERROR_EMAIL_ADDRESSES"))?;

    let private_key_for_rand = PRIVATE_KEY.get()
        .ok_or_else(|| anyhow!("PRIVATE_KEY not initialized"))?;
    let relayer_rand = derive_relayer_rand(private_key_for_rand)?;
    RELAYER_RAND.set(field2hex(&relayer_rand.0))
        .map_err(|_| anyhow!("Failed to set RELAYER_RAND"))?;

    let safe_task = tokio::task::spawn(async move {
        loop {
            match safe_fn().await {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at safe: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let api_server_task = tokio::task::spawn(async move {
        loop {
            match run_server().await {
                Ok(_) => {
                    info!(LOG, "run_server exited normally"; "func" => function_name!());
                    break; // Exit loop if run_server exits normally
                }
                Err(err) => {
                    error!(LOG, "Error api server: {}", err; "func" => function_name!());
                    // Optionally, add a delay before restarting
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });

    let event_listener_task = tokio::task::spawn(async move {
        // Get latest block
        let mut from_block_fund = CLIENT.get_latest_block_number().await;
        let mut from_block_state = CLIENT.get_latest_block_number().await;
        let fund_f = |event: email_wallet_events::UnclaimedFundRegisteredFilter, meta: LogMeta| {
            Box::pin(async move {
                if event.email_addr.is_empty() {
                    return Ok(());
                }
                let random = field2hex(&bytes32_to_fr(&u256_to_bytes32(
                    &event.commitment_randomness,
                ))?);
                let commit = field2hex(&bytes32_to_fr(&event.email_addr_commit)?);
                let claim = Claim {
                    tx_hash: meta.transaction_hash.to_string(),
                    id: event.id,
                    email_address: event.email_addr,
                    random,
                    commit,
                    expiry_time: i64::try_from(event.expiry_time.as_u64()).unwrap(),
                    is_fund: true,
                    is_announced: true,
                    is_seen: false,
                };
                match claim_unclaims(claim.clone()).await {
                    Ok(value) => {
                        if let Err(e) = handle_email_event(value).await {
                            error!(LOG, "Error handling email event: {}", e; "func" => function_name!());
                        }
                    }
                    Err(e) => error!(LOG, "Error claiming: {}", e; "func" => function_name!()),
                }
                Ok(())
            }) as Pin<Box<dyn Future<Output = Result<()>> + Send>> // Add + Send here
        };
        let state_f = |event: email_wallet_events::UnclaimedStateRegisteredFilter,
                       meta: LogMeta| {
            Box::pin(async move {
                if event.email_addr.is_empty() {
                    return Ok(());
                }
                let random = field2hex(&bytes32_to_fr(&u256_to_bytes32(
                    &event.commitment_randomness,
                ))?);
                let commit = field2hex(&bytes32_to_fr(&event.email_addr_commit)?);
                let claim = Claim {
                    tx_hash: meta.transaction_hash.to_string(),
                    id: event.id,
                    email_address: event.email_addr,
                    random,
                    commit,
                    expiry_time: i64::try_from(event.expiry_time.as_u64()).unwrap(),
                    is_fund: false,
                    is_announced: true,
                    is_seen: false,
                };
                match claim_unclaims(claim.clone()).await {
                    Ok(value) => {
                        if let Err(e) = handle_email_event(value).await {
                            error!(LOG, "Error handling email event: {}", e; "func" => function_name!());
                        }
                    }
                    Err(e) => error!(LOG, "Error claiming: {}", e; "func" => function_name!()),
                }
                Ok(())
            }) as Pin<Box<dyn Future<Output = Result<()>> + Send>> // Add + Send here
        };
        loop {
            match event_listener_fn(from_block_fund, fund_f, from_block_state, state_f).await {
                Ok((last_block_f, last_block_s)) => {
                    from_block_fund = last_block_f;
                    from_block_state = last_block_s;
                }
                Err(e) => {
                    error!(LOG, "Error at event_listener: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let voider_task = tokio::task::spawn(async move {
        loop {
            match catch_claims_in_db_fn().await {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at catch claims: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let _ = tokio::join!(api_server_task, event_listener_task, voider_task, safe_task);

    Ok(())
}

async fn event_listener_fn(
    from_block_fund: U64,
    fund_f: impl FnMut(
        email_wallet_events::UnclaimedFundRegisteredFilter,
        LogMeta,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>, // Add + Send here
    from_block_state: U64,
    state_f: impl FnMut(
        email_wallet_events::UnclaimedStateRegisteredFilter,
        LogMeta,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>, // And here, if necessary
) -> Result<(U64, U64)> {
    let last_block_f = CLIENT
        .stream_unclaim_fund_registration(from_block_fund, fund_f)
        .await?;
    let last_block_s = CLIENT
        .stream_unclaim_state_registration(from_block_state, state_f)
        .await?;
    sleep(Duration::from_secs(120)).await;
    Ok((last_block_f + 1, last_block_s + 1))
}

#[named]
async fn catch_claims_in_db_fn() -> Result<()> {
    let now = now();
    let claims = DB.get_claims_unexpired(now).await?;
    for claim in claims {
        info!(LOG, "Claiming claim for : {}", claim.email_address; "func" => function_name!());
        match claim_unclaims(claim.clone()).await {
            Ok(value) => {
                if let Err(e) = handle_email_event(value).await {
                    error!(LOG, "Error handling email event: {}", e; "func" => function_name!());
                }
            }
            Err(e) => trace!(LOG, "Error claiming: {}", e; "func" => function_name!()),
        }
    }
    let claims = DB.get_claims_expired(now).await?;
    for claim in claims {
        let email_addr = claim.email_address.clone();
        info!(LOG, "Voiding claim for : {}", email_addr.clone(); "func" => function_name!());
        let event = match void_unclaims(claim).await {
            Ok(event) => event,
            Err(err) => {
                error!(LOG, "Error voider task: {}", err; "func" => function_name!());
                EmailWalletEvent::Error {
                    email_addr,
                    error_subject: "Voiding claim".to_string(),
                    error: err.to_string(),
                }
            }
        };
        handle_email_event(event).await?;
    }
    sleep(Duration::from_secs(120)).await;
    Ok(())
}
