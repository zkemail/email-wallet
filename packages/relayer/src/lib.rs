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

pub(crate) use chain::*;
pub use config::*;
pub(crate) use core::*;
pub(crate) use database::*;
pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use email_wallet_utils::parse_email::ParsedEmail;

pub async fn run(config: RelayerConfig) -> Result<()> {
    // let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let db = Arc::new(Database::open(&config.db_path)?);
    // for email in db.get_unhandled_emails()? {
    //     tx.send(email)?;
    // }

    let email_receiver_task = tokio::task::spawn(async move {
        println!("Receiver: {:?}", std::thread::current().id());
        let mut email_receiver = ImapClient::new(config.imap_config)?;
        loop {
            println!("I'm retrieving new emails");
            let fetches = email_receiver.retrieve_new_emails()?;
            for fetch in fetches {
                for email in fetch.iter() {
                    if let Some(body) = email.body() {
                        let parsed_email = serde_json::to_string(
                            &ParsedEmail::new_from_raw_email(std::str::from_utf8(body)?).await?,
                        )?;
                        // tx.send(parsed_email)?;
                    }
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    let email_handler_task = tokio::task::spawn(async move {
        println!("Handler: {:?}", std::thread::current().id());
        loop {
            // let email = rx
            //     .recv()
            //     .await
            //     .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
            // tokio::task::spawn(handle_email(email, Arc::clone(&db)));

            println!("Hello world from second task!");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(email_receiver_task, email_handler_task);

    Ok(())
}

async fn handle_email(email: String, db: Arc<Database>) -> Result<()> {
    let email = serde_json::from_str(&email)?;
    is_valid(email);
    Ok(())
}
