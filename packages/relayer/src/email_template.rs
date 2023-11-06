use crate::*;

use handlebars::Handlebars;
use tokio::fs::read_to_string;

pub(crate) async fn email_template_message(
    message_text: &str,
    transaction_hash: &str,
) -> Result<String> {
    let email_template_filename = format!("{}{}", EMAIL_TEMPLATES.get().unwrap(), "email.html");
    let email_template = read_to_string(&email_template_filename).await?;

    let reg = Handlebars::new();

    let data =
        serde_json::json!({"messageText": message_text, "transactionHash": transaction_hash});

    Ok(reg.render_template(&email_template, &data)?)
}
