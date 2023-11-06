use crate::*;

use handlebars::Handlebars;
use std::path::PathBuf;
use tokio::fs::read_to_string;

pub(crate) async fn email_send_message() -> Result<String> {
    let email_send_filename = PathBuf::new()
        .join(EMAIL_TEMPLATES.get().unwrap())
        .join("email_send.html");
    let email_send = read_to_string(&email_send_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({});

    Ok(reg.render_template(&email_send, &data)?)
}

pub(crate) async fn email_invite_message(transaction_hash: &str) -> Result<String> {
    let email_invite_filename = PathBuf::new()
        .join(EMAIL_TEMPLATES.get().unwrap())
        .join("email_invite.html");
    let email_invite = read_to_string(&email_invite_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({"transactionHash": transaction_hash});

    Ok(reg.render_template(&email_invite, &data)?)
}

pub(crate) async fn email_create_message(
    account_key: &str,
    transaction_hash: &str,
) -> Result<String> {
    let email_create_filename = PathBuf::new()
        .join(EMAIL_TEMPLATES.get().unwrap())
        .join("email_create.html");
    let email_create = read_to_string(&email_create_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({ "preheader": "New Email Wallet Account", "accountKey": account_key, "transactionHash": transaction_hash});

    Ok(reg.render_template(&email_create, &data)?)
}
