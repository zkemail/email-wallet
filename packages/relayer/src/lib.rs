#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub(crate) mod abis;
// pub(crate) mod account_creator;
pub(crate) mod chain;
pub(crate) mod claimer;
pub(crate) mod config;
pub(crate) mod core;
pub(crate) mod database;
pub(crate) mod dkim_oracle;
pub(crate) mod emails_pool;
pub(crate) mod imap_client;
pub(crate) mod logger;
pub(crate) mod psi;
pub(crate) mod shared;
pub(crate) mod smtp_client;
pub(crate) mod strings;
pub(crate) mod subgraph;
pub(crate) mod subject_templates;
pub(crate) mod utils;
pub(crate) mod voider;
pub(crate) mod web_server;

pub(crate) use crate::core::*;
use ::function_name::named;
pub(crate) use abis::*;
pub use axum::routing::MethodRouter;
// pub(crate) use account_creator::*;
pub(crate) use chain::*;
pub(crate) use claimer::*;
pub use config::*;
pub(crate) use database::*;
pub(crate) use dkim_oracle::*;
pub(crate) use emails_pool::*;
use futures::TryFutureExt;
pub(crate) use imap_client::*;
pub(crate) use logger::*;
pub(crate) use psi::*;
use rand::rngs::OsRng;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;
pub(crate) use subgraph::*;
pub(crate) use subject_templates::*;
pub(crate) use utils::*;
pub(crate) use voider::*;
pub(crate) use web_server::*;

use anyhow::{anyhow, bail, Result};
use dotenv::dotenv;
use email_wallet_utils::{converters::*, cryptos::*, parse_email::ParsedEmail, Fr};
use ethers::prelude::*;
use lazy_static::lazy_static;
use slog::{debug, error, info, trace};
use std::env;
use std::path::PathBuf;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};

static CIRCUITS_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();
static WEB_SERVER_ADDRESS: OnceLock<String> = OnceLock::new();
static RELAYER_RAND: OnceLock<String> = OnceLock::new();
static PROVER_ADDRESS: OnceLock<String> = OnceLock::new();
static PRIVATE_KEY: OnceLock<String> = OnceLock::new();
static CHAIN_ID: OnceLock<u32> = OnceLock::new();
static CHAIN_RPC_PROVIDER: OnceLock<String> = OnceLock::new();
static CORE_CONTRACT_ADDRESS: OnceLock<String> = OnceLock::new();
static FEE_PER_GAS: OnceLock<U256> = OnceLock::new();
static INPUT_FILES_DIR: OnceLock<String> = OnceLock::new();
static EMAIL_TEMPLATES: OnceLock<String> = OnceLock::new();
static SUBGRAPH_URL: OnceLock<String> = OnceLock::new();
static ONBOARDING_TOKEN_ADDR: OnceLock<H160> = OnceLock::new();
static ONBOARDING_TOKEN_DISTRIBUTION_LIMIT: OnceLock<u32> = OnceLock::new();
static ONBOARDING_TOKEN_AMOUNT: OnceLock<U256> = OnceLock::new();
static ONBOARDING_COUNTER: AtomicU32 = AtomicU32::new(1);
static ONBOARDING_REPLY_MSG: OnceLock<String> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum EmailWalletEvent {
    AccountCreated {
        user_email_addr: String,
        account_key: AccountKey,
        // is_faucet: bool,
        tx_hash: String,
    },
    EmailHandled {
        sender_email_addr: String,
        account_key: AccountKey,
        recipient_email_addr: Option<String>,
        original_subject: String,
        message_id: String,
        email_op: EmailOp,
        tx_hash: String,
    },
    PsiRegistered {
        email_addr: String,
        account_key: AccountKey,
        tx_hash: String,
    },
    Claimed {
        claim: Claim,
        recipient_account_key: AccountKey,
        tx_hash: String,
    },
    Voided {
        claim: Claim,
        tx_hash: String,
    },
    Error {
        error: String,
    },
}

lazy_static! {
    static ref DB: Arc<Database> = {
        dotenv().ok();
        let db = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Database::open(&env::var(DATABASE_PATH_KEY).unwrap()))
            .unwrap();
        Arc::new(db)
    };
    static ref CLIENT: Arc<ChainClient> = {
        dotenv().ok();
        let client = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(ChainClient::setup())
            .unwrap();
        Arc::new(client)
    };
}

#[derive(Debug, Clone)]
pub struct EmailForwardSender(UnboundedSender<EmailMessage>);

impl EmailForwardSender {
    pub fn new() -> (Self, UnboundedReceiver<EmailMessage>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (Self(tx), rx)
    }

    pub fn send(&self, email: EmailMessage) -> Result<()> {
        self.0.send(email)?;
        Ok(())
    }
}

pub struct EventConsumer {
    tx: UnboundedSender<EmailWalletEvent>,
    rx: UnboundedReceiver<EmailWalletEvent>,
    fn_to_call: Box<dyn Fn(EmailWalletEvent) -> Result<()> + Send>,
}

impl EventConsumer {
    pub fn new(fn_to_call: Box<dyn Fn(EmailWalletEvent) -> Result<()> + Send>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self { tx, rx, fn_to_call }
    }
}

pub async fn setup() -> Result<()> {
    dotenv().ok();
    PRIVATE_KEY.set(env::var(PRIVATE_KEY_KEY).unwrap()).unwrap();
    CHAIN_ID
        .set(env::var(CHAIN_ID_KEY).unwrap().parse().unwrap())
        .unwrap();
    CHAIN_RPC_PROVIDER
        .set(env::var(CHAIN_RPC_PROVIDER_KEY).unwrap())
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
    mut email_forward_rx: UnboundedReceiver<EmailMessage>,
    routes: Vec<(String, MethodRouter)>,
) -> Result<()> {
    info!(LOG, "Starting relayer"; "func" => function_name!());

    CIRCUITS_DIR_PATH.set(config.circuits_dir_path).unwrap();
    WEB_SERVER_ADDRESS.set(config.web_server_address).unwrap();
    PROVER_ADDRESS.set(config.prover_address).unwrap();
    PRIVATE_KEY.set(config.private_key).unwrap();
    CHAIN_ID.set(config.chain_id).unwrap();
    CHAIN_RPC_PROVIDER.set(config.chain_rpc_provider).unwrap();
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

    let relayer_rand = derive_relayer_rand(PRIVATE_KEY.get().unwrap())?;
    RELAYER_RAND.set(field2hex(&relayer_rand.0)).unwrap();

    let (tx_handler, mut rx_handler) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (tx_sender, mut rx_sender) = tokio::sync::mpsc::unbounded_channel::<EmailMessage>();
    // let (tx_creator, mut rx_creator) =
    //     tokio::sync::mpsc::unbounded_channel::<(String, Option<AccountKey>)>();
    let (tx_claimer, mut rx_claimer) = tokio::sync::mpsc::unbounded_channel::<Claim>();

    // let db = Arc::new(Database::open(&config.db_path).await?);
    // let client = Arc::new(ChainClient::setup().await?);
    let db = DB.clone();
    let client = CLIENT.clone();

    let event_consumer_tx = event_consumer.tx.clone();
    let event_consumer_task = tokio::task::spawn(async move {
        loop {
            match event_consumer_fn(&mut event_consumer).await {
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

    // let tx_sender_for_email_task = tx_sender.clone();
    let tx_event_consumer_for_email_task = event_consumer_tx.clone();
    let tx_claimer_for_email_task = tx_claimer.clone();
    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let email_handler_task = tokio::task::spawn(async move {
        loop {
            match email_handler_fn(
                &mut rx_handler,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                // tx_sender_for_email_task.clone().into(),
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

    // let tx_sender_for_claimer_task = tx_sender.clone();
    let tx_event_consumer_for_claimer_task = event_consumer_tx.clone();
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
            routes,
            Arc::clone(&db),
            Arc::clone(&client),
            tx_claimer_for_server_task,
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
        let mut from_block_fund = U64::from(0);
        let mut from_block_state = U64::from(0);
        let fund_f = |event: email_wallet_events::UnclaimedFundRegisteredFilter, meta: LogMeta| {
            if event.email_addr.is_empty() {
                return Ok(());
            }
            let random = field2hex(&bytes32_to_fr(&u256_to_bytes32(
                &event.commitment_randomness,
            ))?);
            let commit = field2hex(&bytes32_to_fr(&event.email_addr_commit)?);
            let claim = Claim {
                id: event.id,
                email_address: event.email_addr,
                random,
                commit,
                expiry_time: i64::try_from(event.expiry_time.as_u64()).unwrap(),
                is_fund: true,
                is_announced: true,
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
                id: event.id,
                email_address: event.email_addr,
                random,
                commit,
                expiry_time: i64::try_from(event.expiry_time.as_u64()).unwrap(),
                is_fund: false,
                is_announced: true,
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
    let tx_event_consumer_for_catcher_task = event_consumer_tx.clone();
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
                    error!(LOG, "Error at voider: {}", e; "func" => function_name!())
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
    info!(LOG, "Handling email hash {}", email_hash; "func" => function_name!());
    let emails_pool = FileEmailsPool::new();
    let email = emails_pool.get_email_by_hash(&email_hash).await?;
    info!(LOG, "Handled email {}", email; "func" => function_name!());
    let emails_pool = FileEmailsPool::new();
    emails_pool.delete_email(&email_hash).await?;
    tokio::task::spawn(
        async move {
            let event = handle_email(
                email.clone(),
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                emails_pool,
                tx_claimer,
            )
            .map_err(|err: anyhow::Error| {
                let error = err.to_string();
                EmailWalletEvent::Error { error }
            })
            .await
            .unwrap();
            tx_event_consumer.send(event)?;
            Ok::<(), anyhow::Error>(())
        }, // let event = handle_email(
           //     email.clone(),
           //     Arc::clone(&db_clone),
           //     Arc::clone(&client_clone),
           //     emails_pool,
           //     // Arc::clone(&tx_sender_for_email_task),
           //     tx_claimer_for_email_task.clone(),
           //     // tx_creator_for_email_task.clone(),
           // )
           // .map_err(|err: anyhow::Error| {
           //     let error = err.to_string();
           //     // tokio::spawn(async move {
           //     //     match error_email_if_needed(email, Arc::clone(&db_clone), Arc::clone(&client_clone), tx_sender_for_email_task.clone(), err.clone()).await {
           //     //         Ok(err_cleaned) => {
           //     //             error!(LOG, "Error handling email: {}", err_cleaned; "func" => function_name!());
           //     //         }
           //     //         Err(err_to_send) => {
           //     //             error!(LOG, "Error sending error email: {}", err_to_send; "func" => function_name!());
           //     //         }
           //     //     }
           //     // });
           //     EmailWalletEvent::Error { error }
           // }).await:
    );

    anyhow::Ok(())
}

// #[named]
// async fn account_creation_fn(
//     rx_creator: &mut UnboundedReceiver<(String, Option<AccountKey>)>,
//     db_clone: Arc<Database>,
//     client_clone: Arc<ChainClient>,
//     tx_sender_for_creator_task: &UnboundedSender<EmailMessage>,
// ) -> Result<()> {
//     let (email_address, account_key) = rx_creator
//         .recv()
//         .await
//         .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
//     info!(LOG, "Creating account for email: {}", email_address; "func" => function_name!());
//     tokio::task::spawn(
//         create_account(
//             email_address,
//             account_key,
//             Arc::clone(&db_clone),
//             Arc::clone(&client_clone),
//             tx_sender_for_creator_task.clone(),
//         )
//         .map_err(|err| error!(LOG, "Error creating account: {}", err; "func" => function_name!())),
//     );
//     Ok(())
// }

#[named]
async fn event_consumer_fn(consumer: &mut EventConsumer) -> Result<()> {
    let event = consumer.rx.recv().await.ok_or(anyhow!("No event"))?;
    info!(LOG, "Event: {:?}", event; "func" => function_name!());
    (consumer.fn_to_call)(event)?;
    Ok(())
}

#[named]
async fn claimer_fn(
    rx_claimer: &mut UnboundedReceiver<Claim>,
    db_clone: Arc<Database>,
    client_clone: Arc<ChainClient>,
    tx_event_consumer: UnboundedSender<EmailWalletEvent>,
    // tx_creator_for_claimer_task: &UnboundedSender<(String, Option<AccountKey>)>,
    // tx_sender: &UnboundedSender<EmailMessage>,
) -> Result<()> {
    let claim = rx_claimer
        .recv()
        .await
        .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
    info!(LOG, "Claiming unclaim for {:?}", claim.email_address; "func" => function_name!());
    tokio::task::spawn(async move {
        let event = claim_unclaims(
            claim,
            Arc::clone(&db_clone),
            Arc::clone(&client_clone),
            // tx_creator_for_claimer_task.clone(),
            // tx_sender_for_claimer_task.clone(),
        )
        .map_err(|err| error!(LOG, "Error claiming unclaim: {}", err; "func" => function_name!()))
        .await
        .unwrap();
        tx_event_consumer.send(event)?;
        Ok::<_, anyhow::Error>(())
    });
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
    // tx_sender: &UnboundedSender<EmailMessage>,
) -> Result<()> {
    let now = now();
    let claims = db_clone.get_claims_unexpired(now).await?;
    for claim in claims {
        info!(LOG, "Claiming claim for : {}", claim.email_address; "func" => function_name!());
        tx_claimer.send(claim)?;
    }
    let claims = db_clone.get_claims_expired(now).await?;
    for claim in claims {
        info!(LOG, "Voiding claim for : {}", claim.email_address; "func" => function_name!());
        let db_clone = Arc::clone(&db_clone);
        let client_clone = Arc::clone(&client_clone);
        let tx_event_consumer = tx_event_consumer.clone();
        tokio::task::spawn(async move {
            let event = void_unclaims(
                claim,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                // tx_sender_for_catcher_task.clone(),
            )
            .map_err(|err| error!(LOG, "Error voider task: {}", err; "func" => function_name!()))
            .await
            .unwrap();
            tx_event_consumer.send(event)?;
            Ok::<_, anyhow::Error>(())
        });
    }
    sleep(Duration::from_secs(120)).await;
    Ok(())
}
