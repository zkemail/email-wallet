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

use anyhow::{anyhow, bail, Result};
use email_wallet_utils::parse_email::ParsedEmail;

pub async fn run(config: RelayerConfig) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let db = Arc::new(Database::open(&config.db_path)?);
    for email_and_status in db.get_unhandled_emails()? {
        tx.send(email_and_status)?;
    }

    let db_clone_receiver = Arc::clone(&db);
    let email_receiver_task = tokio::task::spawn(async move {
        let mut email_receiver = ImapClient::new(config.imap_config).await?;
        loop {
            let (new_email_receiver, fetches) = email_receiver.retrieve_new_emails().await?;
            for fetch in fetches {
                for email in fetch.iter() {
                    if let Some(body) = email.body() {
                        let email_and_status = serde_json::to_string(&(
                            String::from_utf8(body.to_vec())?,
                            EmailStatus::Unchecked,
                        ))?;
                        db_clone_receiver.insert_raw_email(&email_and_status)?;
                        tx.send(email_and_status)?;
                    }
                }
            }
            email_receiver = new_email_receiver;
        }
        Ok::<(), anyhow::Error>(())
    });

    let email_handler_task = tokio::task::spawn(async move {
        loop {
            let email_and_status = rx
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
            tokio::task::spawn(handle_email(email_and_status, Arc::clone(&db)));
        }
        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(email_receiver_task, email_handler_task);

    Ok(())
}

async fn handle_email(mut email_and_status: String, db: Arc<Database>) -> Result<()> {
    let EmailAndStatus(raw_email, mut status) = serde_json::from_str(&email_and_status)?;
    let parsed_email = ParsedEmail::new_from_raw_email(&raw_email).await?;

    if let EmailStatus::Unchecked = status {
        if is_valid(&parsed_email) {
            status = EmailStatus::Checked;
            email_and_status = serde_json::to_string(&(parsed_email, EmailStatus::Unchecked))?;
        } else {
            bail!("abacaba");
        }
    }

    if let EmailStatus::Checked = status {
        todo!()
    }

    if let EmailStatus::Executed = status {
        todo!()
    }

    db.remove(&email_and_status)?;

    Ok(())
}
