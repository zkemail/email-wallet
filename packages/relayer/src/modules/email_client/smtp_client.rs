#![allow(unused_imports)]

use crate::*;

use anyhow::anyhow;
use handlebars::Handlebars;
use lettre::{
    message::{
        header::{Cc, From, Header, HeaderName, InReplyTo, ReplyTo, To},
        Attachment, Mailbox, Mailboxes, MessageBuilder, MultiPart, SinglePart,
    },
    transport::smtp::{
        authentication::Credentials, client::SmtpConnection, commands::*, extension::ClientId,
        SMTP_PORT,
    },
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde_json::Value;
use tokio::fs::read_to_string;

#[derive(Debug, Clone)]
pub enum EmailWalletEvent {
    AccountCreated {
        email_addr: String,
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
    AccountNotCreated {
        email_addr: String,
        account_key: AccountKey,
        // claim: Claim,
        is_first: bool,
        tx_hash: String,
    },
    Claimed {
        // claim: Claim,
        unclaimed_fund: Option<UnclaimedFund>,
        unclaimed_state: Option<UnclaimedState>,
        email_addr: String,
        is_fund: bool,
        is_announced: bool,
        recipient_account_key: AccountKey,
        tx_hash: String,
    },
    Voided {
        claim: Claim,
        tx_hash: String,
    },
    Error {
        email_addr: String,
        error: String,
    },
    Ack {
        email_addr: String,
        subject: String,
    },
}

#[derive(Debug, Clone)]
pub struct EmailForwardSender(UnboundedSender<EmailMessage>);

impl EmailForwardSender {
    pub fn new() -> (Self, UnboundedReceiver<EmailMessage>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (Self(tx), rx)
    }

    pub fn send(&self, email: EmailMessage) -> Result<()> {
        match self.0.send(email) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub reference: Option<String>,
    pub reply_to: Option<String>,
    pub body_plain: String,
    pub body_html: String,
    pub body_attachments: Option<Vec<EmailAttachment>>,
}

#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub inline_id: String,
    pub content_type: String,
    pub contents: Vec<u8>,
}

#[derive(Clone)]
pub struct SmtpConfig {
    pub domain_name: String,
    pub id: String,
    pub password: String,
}

pub struct SmtpClient {
    config: SmtpConfig,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub fn new(config: SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(config.id.clone(), config.password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.domain_name)?
            .credentials(creds)
            .build();

        Ok(Self { config, transport })
    }

    pub async fn send_new_email(&self, email: EmailMessage) -> Result<()> {
        self.send_inner(
            email.to,
            email.subject,
            email.reference,
            email.reply_to,
            email.body_plain,
            email.body_html,
            email.body_attachments,
        )
        .await
    }

    async fn send_inner(
        &self,
        to: String,
        subject: String,
        reference: Option<String>,
        reply_to: Option<String>,
        body_plain: String,
        body_html: String,
        body_attachments: Option<Vec<EmailAttachment>>,
    ) -> Result<()> {
        let from_mbox = Mailbox::new(None, self.config.id.parse::<Address>()?);
        let to_mbox = Mailbox::new(None, to.parse::<Address>()?);

        let mut email_builder = Message::builder()
            .from(from_mbox)
            .subject(subject)
            .to(to_mbox);
        if let Some(reference) = reference {
            email_builder = email_builder.references(reference);
        }
        if let Some(reply_to) = reply_to {
            email_builder = email_builder.in_reply_to(reply_to);
        }
        let mut multipart = MultiPart::related().singlepart(SinglePart::html(body_html));
        if let Some(body_attachments) = body_attachments {
            for attachment in body_attachments {
                multipart = multipart.singlepart(
                    Attachment::new_inline(attachment.inline_id)
                        .body(attachment.contents, attachment.content_type.parse()?),
                );
            }
        }
        let email = email_builder.multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(body_plain))
                .multipart(multipart),
        )?;

        self.transport.send(email).await?;

        Ok(())
    }

    fn reconnect(mut self) -> Result<()> {
        const MAX_RETRIES: u32 = 5;
        let mut retry_count = 0;

        while retry_count < MAX_RETRIES {
            match SmtpClient::new(self.config.clone()) {
                Ok(new_client) => {
                    self.transport = new_client.transport;
                    return Ok(());
                }
                Err(_) => {
                    retry_count += 1;
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }

        Err(anyhow!("{SMTP_RECONNECT_ERROR}"))
    }
}

pub async fn render_html(template_name: &str, render_data: Value) -> Result<String> {
    let email_template_filename = PathBuf::new()
        .join(EMAIL_TEMPLATES.get().unwrap())
        .join(template_name);
    let email_template = read_to_string(&email_template_filename).await?;

    let reg = Handlebars::new();

    Ok(reg.render_template(&email_template, &render_data)?)
}

pub fn parse_error(error: String) -> Result<Option<String>> {
    let mut error = error;
    if error.contains("Contract call reverted with data: ") {
        let revert_data = error
            .replace("Contract call reverted with data: ", "")
            .split_at(10)
            .1
            .to_string();
        let revert_bytes = hex::decode(revert_data)
            .unwrap()
            .into_iter()
            .filter(|&b| b >= 0x20 && b <= 0x7E)
            .collect();
        error = String::from_utf8(revert_bytes).unwrap().trim().to_string();
    }

    match error.as_str() {
        "Account is already created" => Ok(Some(error)),
        "insufficient balance" => Ok(Some("You don't have sufficient balance".to_string())),
        _ => Ok(None),
    }
}
