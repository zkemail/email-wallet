#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub(crate) mod abis;
pub(crate) mod account_creator;
pub(crate) mod chain;
pub(crate) mod claimer;
pub(crate) mod config;
pub(crate) mod core;
pub(crate) mod database;
pub(crate) mod imap_client;
pub(crate) mod smtp_client;
pub(crate) mod strings;
pub(crate) mod subject_templates;
pub(crate) mod web_server;

pub(crate) use crate::core::*;
pub(crate) use abis::*;
pub(crate) use account_creator::*;
pub(crate) use chain::*;
pub(crate) use claimer::*;
pub use config::*;
pub(crate) use database::*;
pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;
pub(crate) use subject_templates::*;
pub(crate) use web_server::*;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, bail, Result};
use email_wallet_utils::{converters::*, cryptos::*, parse_email::ParsedEmail, Fr};
use ethers::prelude::*;

static CIRCUITS_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();
static WEB_SERVER_ADDRESS: OnceLock<String> = OnceLock::new();
static RELAYER_RAND: OnceLock<String> = OnceLock::new();
static PROVER_ADDRESS: OnceLock<String> = OnceLock::new();
static PRIVATE_KEY: OnceLock<String> = OnceLock::new();
static CHAIN_ID: OnceLock<u32> = OnceLock::new();
static CHAIN_RPC_PROVIDER: OnceLock<String> = OnceLock::new();
static CORE_CONTRACT_ADDRESS: OnceLock<String> = OnceLock::new();
static FEE_PER_GAS: OnceLock<U256> = OnceLock::new();

pub async fn run(config: RelayerConfig) -> Result<()> {
    CIRCUITS_DIR_PATH.set(config.circuits_dir_path).unwrap();
    WEB_SERVER_ADDRESS.set(config.web_server_address).unwrap();
    RELAYER_RAND.set(config.relayer_randomness).unwrap();
    PROVER_ADDRESS.set(config.prover_address).unwrap();
    PRIVATE_KEY.set(config.private_key).unwrap();
    CHAIN_ID.set(config.chain_id).unwrap();
    CHAIN_RPC_PROVIDER.set(config.chain_rpc_provider).unwrap();
    CORE_CONTRACT_ADDRESS
        .set(config.core_contract_address)
        .unwrap();
    FEE_PER_GAS.set(config.fee_per_gas).unwrap();

    let (tx_handler, mut rx_handler) = tokio::sync::mpsc::unbounded_channel();
    let (tx_sender, mut rx_sender) = tokio::sync::mpsc::unbounded_channel::<EmailMessage>();
    let (tx_creator, mut rx_creator) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (tx_claimer, mut rx_claimer) = tokio::sync::mpsc::unbounded_channel::<Claim>();

    let db = Arc::new(Database::open(&config.db_path).await?);
    let client = Arc::new(ChainClient::setup().await?);
    for email in db.get_unhandled_emails().await? {
        tx_handler.send(email)?;
    }

    let db_clone_receiver = Arc::clone(&db);
    let email_receiver_task = tokio::task::spawn(async move {
        let mut email_receiver = ImapClient::new(config.imap_config).await?;
        loop {
            let (new_email_receiver, fetches) = email_receiver.retrieve_new_emails().await?;
            for fetch in fetches {
                for email in fetch.iter() {
                    if let Some(body) = email.body() {
                        let body = String::from_utf8(body.to_vec())?;
                        if !db_clone_receiver.contains_email(&body).await? {
                            db_clone_receiver.insert_email(&body).await?;
                            tx_handler.send(body)?;
                        }
                    }
                }
            }
            email_receiver = new_email_receiver;
        }

        Ok::<(), anyhow::Error>(())
    });

    let tx_sender_for_email_task = tx_sender.clone();
    let tx_claimer_for_email_task = tx_claimer.clone();
    let tx_sender_for_server_task = tx_sender.clone();
    let tx_sender_for_creator_task = tx_sender.clone();
    let tx_creator_for_server_task = tx_creator.clone();
    let tx_sender_for_claimer_task = tx_sender.clone();
    let tx_creator_for_claimer_task = tx_creator.clone();

    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let email_handler_task = tokio::task::spawn(async move {
        loop {
            let email = rx_handler
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;

            tokio::task::spawn(handle_email(
                email,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_sender_for_email_task.clone(),
                tx_claimer_for_email_task.clone(),
            ));
        }

        Ok::<(), anyhow::Error>(())
    });

    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let account_creation_task = tokio::task::spawn(async move {
        loop {
            let email_address = rx_creator
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;

            tokio::task::spawn(create_account(
                email_address,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_sender_for_creator_task.clone(),
            ));
        }

        Ok::<(), anyhow::Error>(())
    });

    let db_clone = Arc::clone(&db);
    let client_clone = Arc::clone(&client);
    let claimer_task = tokio::task::spawn(async move {
        loop {
            let claim = rx_claimer
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;

            tokio::task::spawn(claim_unclaims(
                claim,
                Arc::clone(&db_clone),
                Arc::clone(&client_clone),
                tx_creator_for_claimer_task.clone(),
                tx_sender_for_claimer_task.clone(),
            ));
        }

        Ok::<(), anyhow::Error>(())
    });

    let api_server_task = tokio::task::spawn(run_server(
        WEB_SERVER_ADDRESS.get().unwrap(),
        Arc::clone(&db),
        Arc::clone(&client),
        tx_sender_for_server_task,
        tx_creator_for_server_task,
    ));

    let email_sender_task = tokio::task::spawn(async move {
        let email_sender = SmtpClient::new(config.smtp_config)?;
        loop {
            let email = rx_sender
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;

            email_sender.send_new_email(email).await?;
        }

        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(
        email_receiver_task,
        email_handler_task,
        account_creation_task,
        claimer_task,
        api_server_task,
        email_sender_task,
    );

    Ok(())
}
