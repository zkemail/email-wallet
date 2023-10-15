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

    let db = Arc::new(Database::open(&config.db_path).await?);
    for email_and_status in db.get_unhandled_emails().await? {
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
                        let body = String::from_utf8(body.to_vec())?;
                        if !db_clone_receiver.contains_finalized_email(&body).await? {
                            let email_and_status =
                                EmailAndStatus::new(body, EmailStatus::Unchecked);
                            db_clone_receiver.insert_email(&email_and_status).await?;
                            tx.send(email_and_status)?;
                        } else {
                            println!("There's such email already");
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

async fn handle_email(email_and_status: EmailAndStatus, db: Arc<Database>) -> Result<()> {
    let EmailAndStatus { email, mut status } = email_and_status;
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;

    if let EmailStatus::Unchecked = status {
        if is_valid(&parsed_email) {
            status = EmailStatus::Checked;
            db.insert_email(&EmailAndStatus::new(email.clone(), status.clone()))
                .await?;
        } else {
            bail!(INVALID_EMAIL);
        }
    }

    if let EmailStatus::Checked = status {
        is_valid(&parsed_email);
        println!("When status checked");
    }

    if let EmailStatus::Executed = status {
        println!("When status executed");
    }

    Ok(())
}
