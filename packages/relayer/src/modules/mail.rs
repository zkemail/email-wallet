use crate::*;
use handlebars::Handlebars;
use relayer_utils::AccountCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::Ordering;
use tokio::fs::read_to_string;

#[derive(Debug, Clone)]
pub enum EmailWalletEvent {
    AccountCreated {
        email_addr: String,
        account_code: AccountCode,
        // is_faucet: bool,
        tx_hash: String,
    },
    EmailHandled {
        sender_email_addr: String,
        account_code: AccountCode,
        recipient_email_addr: Option<String>,
        original_subject: String,
        message_id: String,
        email_op: EmailOp,
        tx_hash: String,
    },
    Invitation {
        email_addr: String,
        account_code: AccountCode,
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
        recipient_account_code: AccountCode,
        tx_hash: String,
    },
    Voided {
        claim: Claim,
        tx_hash: String,
    },
    Error {
        email_addr: String,
        error_subject: String,
        error: String,
    },
    Ack {
        email_addr: String,
        subject: String,
        original_message_id: Option<String>,
    },
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub reference: Option<String>,
    pub reply_to: Option<String>,
    pub body_plain: String,
    pub body_html: String,
    pub body_attachments: Option<Vec<EmailAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub inline_id: String,
    pub content_type: String,
    pub contents: Vec<u8>,
}

pub async fn handle_email_event(event: EmailWalletEvent) -> Result<()> {
    match event {
        EmailWalletEvent::AccountCreated {
            email_addr,
            account_code,
            tx_hash,
        } => {
            let subject = format!("Your Email Wallet Account is created.",);
            let account_salt =
                AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
            CLIENT
                .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
                .await?;
            // Distribute onboarding tokens
            let current_count = ONBOARDING_COUNTER.fetch_add(1, Ordering::SeqCst);
            if current_count < *ONBOARDING_TOKEN_DISTRIBUTION_LIMIT.get().unwrap() {
                if CLIENT
                    .transfer_onboarding_tokens(wallet_addr)
                    .await
                    .is_err()
                {
                    ONBOARDING_COUNTER.fetch_sub(1, Ordering::SeqCst);
                }
                true
            } else {
                ONBOARDING_COUNTER.fetch_sub(1, Ordering::SeqCst);
                false
            };
            let body_plain = format!(
                            "Awesome, {}!\nYour Email Wallet account is now
                           created. PLEASE DO NOT DELETE THIS EMAIL to keep your account
                           secure.\nYou
                           can send 10 TEST tokens directly to emailwallet.relayer@gmail.com by sending us
                           ({}) an email with the subject \"Send 10 TEST to
                           emailwallet.relayer@gmail.com\".\nSimilarly,
                           you can send any currency we support directly to an email address by
                           sending an email with the amount, currency name, and recipient's
                           email address replaced respectively in the subject line.\n{}\nYour wallet address: {}/address/{}.\nCheck the transaction on etherscan: {}/tx/{}",
                           email_addr, RELAYER_EMAIL_ADDRESS.get().unwrap(), ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr, CHAIN_RPC_EXPLORER.get().unwrap(), tx_hash
                        );
            let account_code_str = field_to_hex(&account_code.0);
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "relayerEmailAddr": RELAYER_EMAIL_ADDRESS.get().unwrap(), "faucetMessage": ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountCode": account_code_str});
            let body_html = render_html("account_created.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::EmailHandled {
            sender_email_addr,
            account_code,
            recipient_email_addr: _,
            original_subject,
            message_id,
            email_op: _,
            tx_hash,
        } => {
            let subject = format!("Re: {}", original_subject);
            let account_salt = AccountSalt::new(
                &PaddedEmailAddr::from_email_addr(&sender_email_addr),
                account_code,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nYour transaction request {} is completed in
                            this transaction {}/tx/{}. Thank you for using Email Wallet!\nYour wallet address: {}/address/{}.\nCheck the transaction on etherscan: {}/tx/{}",
                            sender_email_addr, original_subject, CHAIN_RPC_EXPLORER.get().unwrap(), &tx_hash,CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr, CHAIN_RPC_EXPLORER.get().unwrap(), &tx_hash
                        );
            let render_data = serde_json::json!({"userEmailAddr": sender_email_addr, "originalSubject": original_subject, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
            let body_html = render_html("email_handled.html", render_data).await?;
            let email = EmailMessage {
                to: sender_email_addr,
                subject,
                body_plain,
                body_html,
                reference: Some(message_id.clone()),
                reply_to: Some(message_id),
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::Invitation {
            email_addr,
            account_code,
            // claim,
            is_first: _,
            tx_hash,
        } => {
            // let assets_msgs = vec!["ERC20: 100 TEST".to_string()];
            let assets_msgs = vec![];
            let mut assets = search_user_assets(&email_addr).await?;
            for asset in assets.iter_mut() {
                if let Asset::ERC20 {
                    token_addr,
                    token_name,
                    amount,
                    amount_str,
                } = asset
                {
                    if token_name == "TEST" {
                        *amount += U256::from(100);
                        let token_decimal =
                            CLIENT.query_decimals_of_erc20_address(*token_addr).await?;
                        *amount_str =
                            uint_to_decimal_string(amount.as_u128(), token_decimal as usize);
                    }
                }
            }

            let invitation_code_hex = &field_to_hex(&account_code.0)[2..];
            let subject = format!(
                "Your Email Wallet Account is ready to be deployed. Code {}",
                invitation_code_hex
            );

            let (assets_list_plain, assets_list_html) =
                generate_asset_list_body(&assets, assets_msgs).await?;

            let account_salt =
                AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nYour Email Wallet account is ready to be deployed. Your wallet address: {}/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.",
                            email_addr, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "walletAddr": wallet_addr, "assetsList": assets_list_html, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
            println!("render_data: {:?}", render_data);
            let body_html = render_html("invitation.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr.to_string(),
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::Claimed {
            unclaimed_fund: _,
            unclaimed_state: _,
            email_addr,
            is_fund,
            is_announced: _,
            recipient_account_code,
            tx_hash,
        } => {
            let subject = format!(
                "Email Wallet Notification. {}",
                if is_fund {
                    "You received cryptocurrency"
                } else {
                    "You got some data of Email Wallet extensions"
                }
            );
            let account_salt = AccountSalt::new(
                &PaddedEmailAddr::from_email_addr(&email_addr),
                recipient_account_code,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nCheck the transaction for you on etherscan: {}/tx/{}.\nNote that your wallet address is {}\n",
                            email_addr, CHAIN_RPC_EXPLORER.get().unwrap(), &tx_hash, wallet_addr
                        );
            let account_code_str = field_to_hex(&recipient_account_code.0);
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountCode": account_code_str});
            let body_html = render_html("claimed.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::Voided { claim, tx_hash } => {
            let subject = format!(
                "Email Wallet Notification. {}",
                if claim.is_fund {
                    "Your money transfer on Ethereum is voided"
                } else {
                    "Your data of Email Wallet extensions is voided"
                }
            );
            let account_code = DB
                .get_account_code(&claim.email_address)
                .await?
                .ok_or(anyhow!("Account not found"))?;
            let account_code = AccountCode(hex_to_field(&account_code)?);
            let account_salt = AccountSalt::new(
                &PaddedEmailAddr::from_email_addr(&claim.email_address),
                account_code,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nCheck the transaction for you on etherscan: {}/tx/{}.\nNote that your wallet address is {}\n",
                            claim.email_address, CHAIN_RPC_EXPLORER.get().unwrap(), &tx_hash, wallet_addr
                        );
            let render_data = serde_json::json!({"userEmailAddr": claim.email_address, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
            let body_html = render_html("voided.html", render_data).await?;
            let email = EmailMessage {
                to: claim.email_address,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::Error {
            email_addr,
            error_subject,
            error,
        } => {
            let error = parse_error(error)?;
            if let Some(error) = error {
                let subject = format!("Email Wallet Notification. Error occurred.");
                let body_plain = format!("Hi {}!\nError occurred: {}", email_addr, error);
                let render_data = serde_json::json!({"userEmailAddr": email_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
                let body_html = render_html("error.html", render_data).await?;
                let email = EmailMessage {
                    to: email_addr.clone(),
                    subject,
                    body_plain,
                    body_html,
                    reference: None,
                    reply_to: None,
                    body_attachments: None,
                };
                send_email(email).await?;

                // Send error email to team email addresses
                let error_email_addresses = ERROR_EMAIL_ADDRESSES.get().unwrap();
                for error_email_addr in error_email_addresses {
                    let subject = format!("Email Wallet Notification. Error occurred.");
                    let body_plain = format!("Error occurred");
                    let render_data = serde_json::json!({"userEmailAddr": error_email_addr, "error": error, "subject": error_subject, "emailAddr": email_addr});
                    let body_html = render_html("error_alert.html", render_data).await?;
                    let email = EmailMessage {
                        to: error_email_addr.clone(),
                        subject: subject.clone(),
                        body_plain: body_plain.clone(),
                        body_html: body_html.clone(),
                        reference: None,
                        reply_to: None,
                        body_attachments: None,
                    };
                    send_email(email).await?;
                }
            }
        }
        EmailWalletEvent::Ack {
            email_addr,
            subject,
            original_message_id,
        } => {
            let body_plain = format!(
                "Hi {}!\nYour email with the subject {} is received.",
                email_addr, subject
            );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "request": subject});
            let body_html = render_html("acknowledgement.html", render_data).await?;
            let subject = format!("Re: {}", subject);
            let email = EmailMessage {
                to: email_addr,
                subject,
                body_plain,
                body_html,
                reference: original_message_id.clone(),
                reply_to: original_message_id,
                body_attachments: None,
            };
            send_email(email).await?;
        }
        EmailWalletEvent::NoOp => {}
    }

    Ok(())
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
        _ => Ok(Some(error)),
    }
}

pub async fn send_email(email: EmailMessage) -> Result<()> {
    let smtp_server = SMTP_SERVER.get().unwrap();

    // Send POST request to email server
    let client = reqwest::Client::new();
    let response = client
        .post(smtp_server)
        .json(&email)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send email: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to send email: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    Ok(())
}
