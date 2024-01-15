#![allow(unused_imports)]

use crate::*;

use anyhow::anyhow;
use handlebars::Handlebars;
use lettre::{
    message::{
        header::{Cc, From, Header, HeaderName, InReplyTo, ReplyTo, To},
        Mailbox, Mailboxes, MessageBuilder, MultiPart,
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
pub(crate) enum EmailArgs {
    AccountCreation {
        user_email_addr: String,
    },
    AccountInit {
        user_email_addr: String,
        relayer_email_addr: String,
        faucet_message: Option<String>,
        reply_to: String,
    },
    AccountTransport {
        user_email_addr: String,
        relayer_email_addr: String,
        reply_to: String,
    },
    TxComplete {
        user_email_addr: String,
        original_subject: String,
        reply_to: String,
    },
    TxReceived {
        user_email_addr: String,
    },
    Claim {
        user_email_addr: String,
        is_fund: bool,
        claim_msg: String,
    },
    Void {
        user_email_addr: String,
        is_fund: bool,
        void_msg: String,
    },
    Error {
        user_email_addr: String,
        original_subject: Option<String>,
        error_msg: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct EmailMessage {
    pub(crate) to: String,
    pub(crate) email_args: EmailArgs,
    pub(crate) account_key: Option<String>,
    pub(crate) wallet_addr: Option<String>,
    pub(crate) tx_hash: Option<String>,
}

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

    pub(crate) async fn send_new_email(&self, email: EmailMessage) -> Result<()> {
        match email.email_args {
            EmailArgs::AccountCreation { user_email_addr } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. Your Email Wallet Account is created. CODE:{}",
                    &account_key
                );
                let body_plain = format!(
                    "Welcome to Email Wallet, {}!\nYour email
                wallet account is
                created.\nPlease reply to this email to start using Email
                    Wallet. You don't have to add any message in the reply 😄.\nYou already have 100 TEST tokens. Once you reply to this
                    email, you can send them to your friends by specifying the recpient's
                    email address.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "transactionHash": email.tx_hash});
                let body_html = self
                    .render_html("account_creation.html", render_data)
                    .await?;
                self.send_inner(
                    email.to,
                    subject,
                    Some("CODE:".to_string() + &account_key),
                    None,
                    body_plain,
                    body_html,
                )
                .await
            }
            EmailArgs::AccountInit {
                user_email_addr,
                relayer_email_addr,
                faucet_message,
                reply_to,
            } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. Your Email Wallet Account is initialized.",
                );
                let body_plain = format!(
                    "Awesome, {}!\nYour Email Wallet account is now
                   initialized. PLEASE DO NOT DELETE THIS EMAIL to keep your account
                   secure.\nYou
                   can send 10 TEST tokens directly to emailwallet.relayer@gmail.com by sending us
                   ({}) an email with the subject \"Send 10 TEST to
                   emailwallet.relayer@gmail.com\".\nSimilarly,
                   you can send any currency we support directly to an email address by
                   sending an email with the amount, currency name, and recipient's
                   email address replaced respectively in the subject line.\n{}\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, relayer_email_addr, faucet_message.clone().unwrap_or(String::new()), email.wallet_addr.clone().expect("wallet_addr must be set"), email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "relayerEmailAddr": relayer_email_addr, "faucetMessage": faucet_message.unwrap_or(String::new()), "walletAddr": email.wallet_addr.expect("wallet_addr must be set"),  "transactionHash": email.tx_hash});
                let body_html = self.render_html("account_init.html", render_data).await?;
                self.send_inner(
                    email.to,
                    subject,
                    Some("CODE:".to_string() + &account_key),
                    Some(reply_to),
                    body_plain,
                    body_html,
                )
                .await
            }
            EmailArgs::AccountTransport {
                user_email_addr,
                relayer_email_addr,
                reply_to,
            } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. Your Email Wallet Account is transported.",
                );
                let body_plain = format!(
                    "Hi {}!\nYour account is securely transported to us. Now you can make any Email
                    Wallet transactions by sending an email to {}.\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, relayer_email_addr, email.wallet_addr.clone().expect("wallet_addr must be set"), email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "relayerEmailAddr": relayer_email_addr, "walletAddr":email.wallet_addr.clone().expect("wallet_addr must be set"), "transactionHash": email.tx_hash});
                let body_html = self
                    .render_html("account_transport.html", render_data)
                    .await?;
                self.send_inner(
                    email.to,
                    subject,
                    Some("CODE:".to_string() + &account_key),
                    Some(reply_to),
                    body_plain,
                    body_html,
                )
                .await
            }
            EmailArgs::TxComplete {
                user_email_addr,
                original_subject,
                reply_to,
            } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. Your Email Wallet transaction is completed.",
                );
                let body_plain = format!(
                    "Hi {}!\nYour transaction request {} is completed in
                    this transaction https://sepolia.etherscan.io/tx/{}. Thank you for using Email Wallet!\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, original_subject, email.tx_hash.clone().expect("tx_hash must be set"), email.wallet_addr.clone().expect("wallet_addr must be set"), email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "originalSubject": original_subject, "walletAddr":email.wallet_addr.clone().expect("wallet_addr must be set"), "transactionHash": email.tx_hash});
                let body_html = self
                    .render_html("transaction_complete.html", render_data)
                    .await?;
                self.send_inner(
                    email.to,
                    subject,
                    Some("CODE:".to_string() + &account_key),
                    Some(reply_to),
                    body_plain,
                    body_html,
                )
                .await
            }
            EmailArgs::TxReceived { user_email_addr } => {
                let subject = format!(
                    "Email Wallet Notification. There is an Email Wallet transaction for you.",
                );
                let body_plain = format!(
                    "Hi {}!\nAn Email Wallet transaction is executed for you in
                    this transaction https://sepolia.etherscan.io/tx/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, email.tx_hash.clone().expect("tx_hash must be set"), email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "transactionHash": email.tx_hash});
                let body_html = self
                    .render_html("transaction_receiver.html", render_data)
                    .await?;
                self.send_inner(email.to, subject, None, None, body_plain, body_html)
                    .await
            }
            EmailArgs::Claim {
                user_email_addr,
                is_fund,
                claim_msg,
            } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. {}",
                    if is_fund {
                        "You received money on Ethereum"
                    } else {
                        "You got some data of Email Wallet extensions"
                    }
                );
                let body_plain = format!(
                    "Hi {}!\n{}\\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr,
                    claim_msg,
                    email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "claimMsg": claim_msg, "transactionHash": email.tx_hash});
                let body_html = self.render_html("claim.html", render_data).await?;
                self.send_inner(
                    email.to,
                    subject,
                    Some("CODE:".to_string() + &account_key),
                    None,
                    body_plain,
                    body_html,
                )
                .await
            }
            EmailArgs::Void {
                user_email_addr,
                is_fund,
                void_msg,
            } => {
                let account_key = email
                    .account_key
                    .expect("account_key must be set for the account creation email.");
                let subject = format!(
                    "Email Wallet Notification. {}",
                    if is_fund {
                        "Your token transfer is voided due to expiration."
                    } else {
                        "Your data of Email Wallet extension is voided due to expiration."
                    }
                );
                let body_plain = format!(
                    "Hi {}!\n{}\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                    user_email_addr, void_msg,email.wallet_addr.clone().expect("wallet_addr must be set"), email.tx_hash.clone().expect("tx_hash must be set")
                );
                let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "voidMsg": void_msg, "walletAddr":email.wallet_addr.clone().expect("wallet_addr must be set"), "transactionHash": email.tx_hash});
                let body_html = self.render_html("void.html", render_data).await?;
                self.send_inner(email.to, subject, None, None, body_plain, body_html)
                    .await
            }
            EmailArgs::Error {
                user_email_addr,
                original_subject,
                error_msg,
            } => {
                let account_key = email.account_key;
                let subject = format!("Email Wallet Notification. Something went wrong.",);
                match original_subject {
                    Some(original_subject) => {
                        let body_plain = format!(
                            "Hi {}!\nYour request \"{}\" failed due to the following error: {}.",
                            user_email_addr, original_subject, error_msg
                        );
                        let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "originalSubject": original_subject, "errorMsg": error_msg});
                        let body_html = self.render_html("error.html", render_data).await?;
                        self.send_inner(
                            email.to,
                            subject,
                            account_key
                                .map(|value| "CODE:".to_string() + &value)
                                .or(None),
                            None,
                            body_plain,
                            body_html,
                        )
                        .await
                    }
                    None => {
                        let original_subject =
                            "to initialize or transport your account".to_string();
                        let body_plain = format!(
                            "Hi {}!\nYour request \"{}\" failed due to the following error: {}.",
                            user_email_addr, original_subject, error_msg
                        );
                        let render_data = serde_json::json!({"userEmailAddr": user_email_addr, "originalSubject": original_subject, "errorMsg": error_msg});
                        let body_html = self.render_html("error.html", render_data).await?;
                        self.send_inner(
                            email.to,
                            subject,
                            account_key
                                .map(|value| "CODE:".to_string() + &value)
                                .or(None),
                            None,
                            body_plain,
                            body_html,
                        )
                        .await
                    }
                }
            }
        }
    }

    async fn send_inner(
        &self,
        to: String,
        subject: String,
        reference: Option<String>,
        reply_to: Option<String>,
        body_plain: String,
        body_html: String,
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
        let email =
            email_builder.multipart(MultiPart::alternative_plain_html(body_plain, body_html))?;

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

    async fn render_html(&self, template_name: &str, render_data: Value) -> Result<String> {
        let email_template_filename = PathBuf::new()
            .join(EMAIL_TEMPLATES.get().unwrap())
            .join(template_name);
        let email_template = read_to_string(&email_template_filename).await?;

        let reg = Handlebars::new();

        // let data =
        //     serde_json::json!({"messageText": message_text, "transactionHash": transaction_hash});

        Ok(reg.render_template(&email_template, &render_data)?)
    }
}

pub(crate) async fn error_email_if_needed(
    email: String,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_sender: Arc<UnboundedSender<EmailMessage>>,
    mut error: String,
) -> Result<String> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let from_address = parsed_email.get_from_addr()?;
    let subject = parsed_email.get_subject_all()?;

    let account_key = db
        .get_account_key(&from_address)
        .await?
        .ok_or(anyhow!("Account key not found"))?;
    let account_key = AccountKey::from(hex2field(&account_key)?);
    let wallet_salt = account_key.to_wallet_salt()?;
    let wallet_addr = chain_client
        .get_wallet_addr_from_salt(&wallet_salt.0)
        .await?;
    debug!(LOG, "wallet_addr: {}", wallet_addr);

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

    let error_email = match error.as_str() {
        "Account is already created" => {
            let email = EmailMessage {
                to: from_address.clone(),
                email_args: EmailArgs::Error {
                    user_email_addr: from_address,
                    original_subject: Some(subject),
                    error_msg: "Account is already created".to_string(),
                },
                account_key: Some(field2hex(&account_key.0)),
                wallet_addr: Some(ethers::utils::to_checksum(&wallet_addr, None)),
                tx_hash: None,
            };
            Ok(email)
        }
        "insufficient balance" => {
            let email = EmailMessage {
                to: from_address.clone(),
                email_args: EmailArgs::Error {
                    user_email_addr: from_address,
                    original_subject: Some(subject),
                    error_msg: "You don't have sufficient balance".to_string(),
                },
                account_key: Some(field2hex(&account_key.0)),
                wallet_addr: Some(ethers::utils::to_checksum(&wallet_addr, None)),
                tx_hash: None,
            };
            Ok(email)
        }
        _ => Err(()),
    };
    match error_email {
        Ok(email) => {
            tx_sender.send(email).unwrap();
            Ok(error)
        }
        Err(_) => Ok(error),
    }
}
