#![allow(unused_imports)]

use crate::*;

use anyhow::anyhow;
use lettre::{
    message::{
        header::{Cc, From, Header, HeaderName, InReplyTo, ReplyTo, To},
        Mailbox, Mailboxes, MessageBuilder,
    },
    transport::smtp::{
        authentication::Credentials, client::SmtpConnection, commands::*, extension::ClientId,
        SMTP_PORT,
    },
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Clone)]
pub(crate) struct SmtpConfig {
    pub(crate) domain_name: String,
    pub(crate) id: String,
    pub(crate) password: String,
}

pub(crate) struct SmtpClient {
    config: SmtpConfig,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub(crate) fn new(config: SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(config.id.clone(), config.password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.domain_name)?
            .credentials(creds)
            .build();

        Ok(Self { config, transport })
    }

    pub(crate) async fn send_new_email(
        &self,
        email_subject: &str,
        email_body: &str,
        email_to: &str,
    ) -> Result<()> {
        let from_mbox = Mailbox::new(None, self.config.id.parse::<Address>()?);
        let to_mbox = Mailbox::new(None, email_to.parse::<Address>()?);

        let email = Message::builder()
            .from(from_mbox)
            .subject(email_subject)
            .to(to_mbox)
            .body(email_body.to_string())?;

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

        Err(anyhow!("{IMAP_RECONNECT_ERROR}"))
    }
}
