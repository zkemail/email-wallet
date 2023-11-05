use crate::*;

use handlebars::Handlebars;
use tokio::fs::read_to_string;

pub(crate) async fn email_send_message() -> Result<String> {
    let email_send_filename = format!("{}{}", EMAIL_TEMPLATES.get().unwrap(), "email_send.html");
    let email_send = read_to_string(&email_send_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({});

    Ok(reg.render_template(&email_send, &data)?)
}

pub(crate) async fn email_invite_message() -> Result<String> {
    let email_send_filename = format!("{}{}", EMAIL_TEMPLATES.get().unwrap(), "email_invite.html");
    let email_send = read_to_string(&email_send_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({});

    Ok(reg.render_template(&email_send, &data)?)
}

pub(crate) async fn email_create_message() -> Result<String> {
    let email_send_filename = format!("{}{}", EMAIL_TEMPLATES.get().unwrap(), "email_create.html");
    let email_send = read_to_string(&email_send_filename).await?;

    let reg = Handlebars::new();

    let data = serde_json::json!({});

    Ok(reg.render_template(&email_send, &data)?)
}
