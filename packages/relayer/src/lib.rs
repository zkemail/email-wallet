#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub(crate) mod chain;
pub mod config;
pub(crate) mod core;
pub(crate) mod database;
pub(crate) mod imap_client;
pub(crate) mod smtp_client;
pub(crate) mod strings;
pub(crate) mod web_server;

pub(crate) use chain::*;
pub use config::*;
pub(crate) use core::*;
pub(crate) use database::*;
pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;
pub(crate) use web_server::*;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, bail, Result};
use email_wallet_utils::converters::*;
use email_wallet_utils::cryptos::*;
use email_wallet_utils::parse_email::ParsedEmail;

static CIRCUITS_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();
static WEB_SERVER_ADDRESS: OnceLock<String> = OnceLock::new();
static RELAYER_RAND: OnceLock<String> = OnceLock::new();

pub async fn run(config: RelayerConfig) -> Result<()> {
    CIRCUITS_DIR_PATH.set(config.circuits_dir_path).unwrap();
    WEB_SERVER_ADDRESS.set(config.web_server_address).unwrap();
    RELAYER_RAND.set(config.relayer_randomness).unwrap();

    let (tx_handler, mut rx_handler) = tokio::sync::mpsc::unbounded_channel();
    let (tx_sender, mut rx_sender) = tokio::sync::mpsc::unbounded_channel::<(String, String)>();

    let db = Arc::new(Database::open(&config.db_path).await?);
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

    let email_handler_task = tokio::task::spawn(async move {
        loop {
            let email = rx_handler
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;

            tokio::task::spawn(handle_email(email, Arc::clone(&db), tx_sender.clone()));
        }

        Ok::<(), anyhow::Error>(())
    });

    let email_sender_task = tokio::task::spawn(async move {
        let email_sender = SmtpClient::new(config.smtp_config)?;
        loop {
            let (email, recipient) = rx_sender
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
            email_sender
                .send_new_email("Transaction Status", &email, &recipient)
                .await?;
        }

        Ok::<(), anyhow::Error>(())
    });

    let api_server_task = tokio::task::spawn(async move {
        run_server(WEB_SERVER_ADDRESS.get().unwrap()).await?;

        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(
        email_receiver_task,
        email_handler_task,
        email_sender_task,
        api_server_task
    );

    Ok(())
}
