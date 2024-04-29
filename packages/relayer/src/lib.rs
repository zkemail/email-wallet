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
pub use utils::*;

use ::function_name::named;
use futures::TryFutureExt;
use rand::rngs::OsRng;
use tokio::sync::Mutex;

use anyhow::{anyhow, bail, Result};
use dotenv::dotenv;
use email_wallet_utils::{converters::*, cryptos::*, parse_email::ParsedEmail, Fr};
use ethers::prelude::*;
use lazy_static::lazy_static;
use slog::{error, info, trace};
use std::env;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
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

lazy_static! {
    pub static ref DB: Arc<Database> = {
        dotenv().ok();
        let db = tokio::task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(Database::open(&env::var(DATABASE_PATH_KEY).unwrap()))
        })
        .unwrap();
        Arc::new(db)
    };
    pub static ref CLIENT: Arc<ChainClient> = {
        dotenv().ok();
        let client = tokio::task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(ChainClient::setup())
        })
        .unwrap();
        Arc::new(client)
    };
    pub static ref SHARED_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

pub type EventConsumer =
    fn(EmailWalletEvent, EmailForwardSender) -> Pin<Box<dyn Future<Output = ()> + Send>>;

pub async fn setup() -> Result<()> {
    dotenv().ok();
    PRIVATE_KEY.set(env::var(PRIVATE_KEY_KEY).unwrap()).unwrap();
    CHAIN_ID
        .set(env::var(CHAIN_ID_KEY).unwrap().parse().unwrap())
        .unwrap();
    CHAIN_RPC_PROVIDER
        .set(env::var(CHAIN_RPC_PROVIDER_KEY).unwrap())
        .unwrap();
    CHAIN_RPC_EXPLORER
        .set(env::var(CHAIN_RPC_EXPLORER_KEY).unwrap())
        .unwrap();
    CORE_CONTRACT_ADDRESS
        .set(env::var(CORE_CONTRACT_ADDRESS_KEY).unwrap())
        .unwrap();

    let rng = OsRng;
    let relayer_rand = derive_relayer_rand(PRIVATE_KEY.get().unwrap())?;

    let client = ChainClient::setup().await?;
    let tx_hash = client
        .register_relayer(
            env::var(RELAYER_EMAIL_ADDR_KEY).unwrap(),
            env::var(RELAYER_HOSTNAME_KEY).unwrap(),
        )
        .await?;
    println!("Register relayer in {}", tx_hash);
    Ok(())
}

#[named]
pub async fn run(
    config: RelayerConfig,
    mut event_consumer: EventConsumer,
    email_forward_sender: EmailForwardSender,
    mut email_forward_rx: UnboundedReceiver<EmailMessage>,
) -> Result<()> {
    info!(LOG, "Starting relayer"; "func" => function_name!());

    CIRCUITS_DIR_PATH.set(config.circuits_dir_path).unwrap();
    WEB_SERVER_ADDRESS.set(config.web_server_address).unwrap();
    PROVER_ADDRESS.set(config.prover_address).unwrap();
    PRIVATE_KEY.set(config.private_key).unwrap();
    CHAIN_ID.set(config.chain_id).unwrap();
    CHAIN_RPC_PROVIDER.set(config.chain_rpc_provider).unwrap();
    CHAIN_RPC_EXPLORER.set(config.chain_rpc_explorer).unwrap();
    CORE_CONTRACT_ADDRESS
        .set(config.core_contract_address)
        .unwrap();
    FEE_PER_GAS.set(config.fee_per_gas).unwrap();
    INPUT_FILES_DIR.set(config.input_files_dir).unwrap();
    EMAIL_TEMPLATES.set(config.email_templates).unwrap();
    SUBGRAPH_URL.set(config.subgraph_url).unwrap();
    ONBOARDING_TOKEN_ADDR
        .set(config.onboarding_token_addr)
        .unwrap();
    ONBOARDING_TOKEN_DISTRIBUTION_LIMIT
        .set(config.onboarding_token_distribution_limit)
        .unwrap();
    ONBOARDING_TOKEN_AMOUNT
        .set(config.onboarding_token_amount)
        .unwrap();
    ONBOARDING_REPLY_MSG
        .set(config.onboarding_reply_msg)
        .unwrap();
    RELAYER_EMAIL_ADDRESS
        .set(config.smtp_config.id.clone())
        .unwrap();

    let relayer_rand = derive_relayer_rand(PRIVATE_KEY.get().unwrap())?;
    RELAYER_RAND.set(field2hex(&relayer_rand.0)).unwrap();

    let (tx_handler, mut rx_handler) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (tx_sender, mut rx_sender) = tokio::sync::mpsc::unbounded_channel::<EmailMessage>();
    let (tx_claimer, mut rx_claimer) = tokio::sync::mpsc::unbounded_channel::<Claim>();
    let (tx_event_consumer, mut rx_event_consumer) = tokio::sync::mpsc::unbounded_channel();
    let db = DB.clone();
    let client = CLIENT.clone();

    let email_sender = email_forward_sender.clone();
    let event_consumer_task = tokio::task::spawn(async move {
        loop {
            match event_consumer_fn(
                &mut event_consumer,
                &mut rx_event_consumer,
                email_sender.clone(),
            )
            .await
            {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at event_consumer: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let tx_handler_for_fetcher_task = tx_handler.clone();
    let emails_pool_fetcher_task = tokio::task::spawn(async move {
        loop {
            match emails_pool_fetcher_fn(&tx_handler_for_fetcher_task).await {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at emails_pool_fetcher: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });
    let db_clone_receiver = Arc::clone(&db);
    let mut email_receiver = ImapClient::new(config.imap_config).await?;
    let tx_handler_for_receiver_task = tx_handler.clone();
    let email_receiver_task = tokio::task::spawn(async move {
        loop {
            match email_receiver_fn(&mut email_receiver, &tx_handler_for_receiver_task).await {
                Ok(new_email_receiver) => {}
                Err(e) => {
                    error!(LOG, "Error at email_receiver: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let tx_event_consumer_for_email_task = tx_event_consumer.clone();
    let tx_claimer_for_email_task = tx_claimer.clone();
    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let email_handler_task = tokio::task::spawn(async move {
        loop {
            match email_handler_fn(
                &mut rx_handler,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_event_consumer_for_email_task.clone(),
                tx_claimer_for_email_task.clone(),
            )
            .await
            {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at email_handler: {}", e; "func" => function_name!())
                }
            }
        }

        anyhow::Ok(())
    });

    let tx_event_consumer_for_claimer_task = tx_event_consumer.clone();
    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let claimer_task = tokio::task::spawn(async move {
        loop {
            match claimer_fn(
                &mut rx_claimer,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_event_consumer_for_claimer_task.clone(),
            )
            .await
            {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at claimer: {}", e; "func" => function_name!())
                }
            }
        }

        anyhow::Ok(())
    });

    let tx_claimer_for_server_task = tx_claimer.clone();
    let api_server_task = tokio::task::spawn(
        run_server(
            WEB_SERVER_ADDRESS.get().unwrap(),
            Arc::clone(&db),
            Arc::clone(&client),
            tx_claimer_for_server_task,
            email_forward_sender.clone(),
        )
        .map_err(|err| error!(LOG, "Error api server: {}", err; "func" => function_name!())),
    );

    let email_sender = SmtpClient::new(config.smtp_config)?;
    let email_sender_task = tokio::task::spawn(async move {
        loop {
            match email_sender_fn(&mut rx_sender, &email_sender).await {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at email_sender: {}", e; "func" => function_name!())
                }
            }
        }

        anyhow::Ok(())
    });

    let tx_sender_for_forward_task = tx_sender.clone();
    let email_forward_task = tokio::task::spawn(async move {
        loop {
            match email_forward_rx.recv().await {
                Some(email) => {
                    tx_sender_for_forward_task.send(email)?;
                }
                None => {
                    error!(LOG, "Error at email_forward: no email"; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let tx_claimer_for_listener_task = tx_claimer.clone();
    let client_clone = Arc::clone(&client);
    let event_listener_task = tokio::task::spawn(async move {
        // Get latest block
        let mut from_block_fund = client_clone.get_latest_block_number().await;
        let mut from_block_state = client_clone.get_latest_block_number().await;
        let fund_f = |event: email_wallet_events::UnclaimedFundRegisteredFilter, meta: LogMeta| {
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
            tx_claimer_for_listener_task.send(claim)?;
            Ok(())
        };
        let state_f = |event: email_wallet_events::UnclaimedStateRegisteredFilter,
                       meta: LogMeta| {
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
            tx_claimer_for_listener_task.send(claim)?;
            Ok(())
        };
        loop {
            match event_listener_fn(
                Arc::clone(&client_clone),
                &tx_claimer_for_listener_task,
                from_block_fund,
                fund_f,
                from_block_state,
                state_f,
            )
            .await
            {
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

    let tx_claimer_for_catcher_task = tx_claimer.clone();
    let tx_event_consumer_for_catcher_task = tx_event_consumer.clone();
    // let tx_sender_for_catcher_task = tx_sender.clone();
    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let voider_task = tokio::task::spawn(async move {
        loop {
            match catch_claims_in_db_fn(
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_claimer_for_catcher_task.clone(),
                tx_event_consumer_for_catcher_task.clone(),
            )
            .await
            {
                Ok(()) => {}
                Err(e) => {
                    error!(LOG, "Error at catch claims: {}", e; "func" => function_name!())
                }
            }
        }
        anyhow::Ok(())
    });

    let _ = tokio::join!(
        event_consumer_task,
        email_receiver_task,
        email_handler_task,
        claimer_task,
        api_server_task,
        email_sender_task,
        event_listener_task,
        voider_task
    );

    Ok(())
}

#[named]
async fn emails_pool_fetcher_fn(
    tx_handler_for_fetcher_task: &UnboundedSender<String>,
) -> Result<()> {
    let emails_pool = FileEmailsPool::new();
    let unhandled_emails = emails_pool.get_unhandled_emails().await?;
    for (email_hash, _) in unhandled_emails {
        info!(LOG, "unhandled email {}", email_hash; "func" => function_name!());
        tx_handler_for_fetcher_task.send(email_hash)?;
    }
    sleep(Duration::from_secs(30)).await;
    anyhow::Ok(())
}

#[named]
async fn email_receiver_fn(
    email_receiver: &mut ImapClient,
    tx_handler_for_receiver_task: &UnboundedSender<String>,
) -> Result<()> {
    let fetches = email_receiver.retrieve_new_emails().await?;
    info!(LOG, "Fetched {} emails", fetches.len(); "func" => function_name!());
    for fetch in fetches {
        for email in fetch.iter() {
            if let Some(body) = email.body() {
                let body = String::from_utf8(body.to_vec())?;
                info!(LOG, "Received email {}", body; "func" => function_name!());
                let email_hash = calculate_default_hash(&body);
                let emails_pool = FileEmailsPool::new();
                if !emails_pool.contains_email(&email_hash).await? {
                    emails_pool.insert_email(&email_hash, &body).await?;
                    tx_handler_for_receiver_task.send(email_hash)?;
                }
            }
        }
    }
    sleep(Duration::from_secs(5)).await;
    Ok(())
}

#[named]
async fn email_handler_fn(
    rx_handler: &mut UnboundedReceiver<String>,
    db_clone: Arc<Database>,
    client_clone: Arc<ChainClient>,
    tx_event_consumer: UnboundedSender<EmailWalletEvent>,
    tx_claimer: UnboundedSender<Claim>,
    // tx_creator_for_email_task: &UnboundedSender<(String, Option<AccountKey>)>,
) -> Result<()> {
    let email_hash = rx_handler
        .recv()
        .await
        .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
    let emails_pool = FileEmailsPool::new();
    let email = emails_pool.get_email_by_hash(&email_hash).await?;
    let emails_pool = FileEmailsPool::new();
    emails_pool.delete_email(&email_hash).await?;
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let email_addr = parsed_email.get_from_addr().unwrap();
    tx_event_consumer.send(EmailWalletEvent::Ack {
        email_addr: email_addr.clone(),
        subject: parsed_email.get_subject_all().unwrap(),
    })?;
    let event = match handle_email(
        email.clone(),
        Arc::clone(&db_clone),
        Arc::clone(&client_clone),
        emails_pool,
        tx_claimer,
    )
    .await
    {
        Ok(event) => event,
        Err(err) => {
            let error = err.to_string();
            error!(LOG, "Error handling email: {}", error; "func" => function_name!());
            EmailWalletEvent::Error { email_addr, error }
        }
    };
    tx_event_consumer.send(event)?;
    anyhow::Ok(())
}

#[named]
async fn event_consumer_fn(
    consumer: &mut EventConsumer,
    rx: &mut UnboundedReceiver<EmailWalletEvent>,
    email_sender: EmailForwardSender,
) -> Result<()> {
    let event = rx.recv().await.ok_or(anyhow!("No event"))?;
    info!(LOG, "Event: {:?}", event; "func" => function_name!());
    (consumer)(event, email_sender).await;
    Ok(())
}

#[named]
async fn claimer_fn(
    rx_claimer: &mut UnboundedReceiver<Claim>,
    db_clone: Arc<Database>,
    client_clone: Arc<ChainClient>,
    tx_event_consumer: UnboundedSender<EmailWalletEvent>,
) -> Result<()> {
    let claim = rx_claimer
        .recv()
        .await
        .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
    let email_addr = claim.email_address.clone();
    info!(LOG, "Claiming unclaim {:?}", claim; "func" => function_name!());
    let event = match claim_unclaims(claim, Arc::clone(&db_clone), Arc::clone(&client_clone)).await
    {
        Ok(event) => event,
        Err(err) => {
            error!(LOG, "Error claiming unclaim: {}", err.to_string(); "func" => function_name!());
            EmailWalletEvent::Error {
                email_addr,
                error: err.to_string(),
            }
        }
    };
    tx_event_consumer.send(event)?;
    Ok(())
}

#[named]
async fn email_sender_fn(
    rx_sender: &mut UnboundedReceiver<EmailMessage>,
    email_sender: &SmtpClient,
) -> Result<()> {
    let email = rx_sender
        .recv()
        .await
        .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
    info!(LOG, "Sending email: {:?}", email; "func" => function_name!());
    // info!(LOG, "Email arg: {:?}", email.email_args; "func" => function_name!());
    email_sender.send_new_email(email).await?;
    Ok(())
}

async fn event_listener_fn<
    F1: FnMut(email_wallet_events::UnclaimedFundRegisteredFilter, LogMeta) -> Result<()>,
    F2: FnMut(email_wallet_events::UnclaimedStateRegisteredFilter, LogMeta) -> Result<()>,
>(
    client_clone: Arc<ChainClient>,
    tx_claimer_for_listener_task: &UnboundedSender<Claim>,
    from_block_fund: U64,
    fund_f: F1,
    from_block_state: U64,
    state_f: F2,
) -> Result<(U64, U64)> {
    let last_block_f = client_clone
        .stream_unclaim_fund_registration(from_block_fund, fund_f)
        .await?;
    let last_block_s = client_clone
        .stream_unclaim_state_registration(from_block_state, state_f)
        .await?;
    sleep(Duration::from_secs(120)).await;
    Ok((last_block_f + 1, last_block_s + 1))
}

#[named]
async fn catch_claims_in_db_fn(
    db_clone: Arc<Database>,
    client_clone: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
    tx_event_consumer: UnboundedSender<EmailWalletEvent>,
) -> Result<()> {
    let now = now();
    let claims = db_clone.get_claims_unexpired(now).await?;
    info!(LOG, "unexpired claims: {:?}", claims; "func" => function_name!());
    for claim in claims {
        info!(LOG, "Claiming claim for : {}", claim.email_address; "func" => function_name!());
        tx_claimer.send(claim)?;
    }
    let claims = db_clone.get_claims_expired(now).await?;
    info!(LOG, "expired claims: {:?}", claims; "func" => function_name!());
    for claim in claims {
        let email_addr = claim.email_address.clone();
        info!(LOG, "Voiding claim for : {}", email_addr.clone(); "func" => function_name!());
        let db_clone = Arc::clone(&db_clone);
        let client_clone = Arc::clone(&client_clone);
        let tx_event_consumer = tx_event_consumer.clone();
        let event = match void_unclaims(
            claim,
            Arc::clone(&db_clone),
            Arc::clone(&client_clone),
            // tx_sender_for_catcher_task.clone(),
        )
        .await
        {
            Ok(event) => event,
            Err(err) => {
                error!(LOG, "Error voider task: {}", err; "func" => function_name!());
                EmailWalletEvent::Error {
                    email_addr,
                    error: err.to_string(),
                }
            }
        };
        tx_event_consumer.send(event)?;
    }
    sleep(Duration::from_secs(120)).await;
    Ok(())
}
