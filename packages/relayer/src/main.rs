use anyhow::{anyhow, Result};
use email_wallet_utils::{
    converters::hex2field,
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use relayer::*;
use slog::error;
use std::future::Future;
use std::{env, pin::Pin, sync::atomic::Ordering};

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        let (sender, rx) = EmailForwardSender::new();
        // let event_consumer_fn = move |event: EmailWalletEvent| {
        //     tokio::runtime::Runtime::new()
        //         .unwrap()
        //         .block_on(event_consumer_fn(event, sender.clone()))
        // };
        // let event_consumer = EventConsumer::new(Box::new(event_consumer_fn));
        let routes = vec![];
        run(RelayerConfig::new(), event_consumer, sender, rx, routes).await?;
    }
    Ok(())
}

fn event_consumer(
    event: EmailWalletEvent,
    sender: EmailForwardSender,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async {
        match event_consumer_fn(event, sender).await {
            Ok(_) => {}
            Err(err) => {
                error!(LOG, "Failed to accept event: {}", err);
            }
        }
    })
}

async fn event_consumer_fn(event: EmailWalletEvent, sender: EmailForwardSender) -> Result<()> {
    match event {
        EmailWalletEvent::AccountCreated {
            email_addr,
            account_key,
            tx_hash,
        } => {
            let subject =
                format!("Email Wallet Notification. Your Email Wallet Account is created.",);
            let wallet_salt =
                WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
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
                    // info!(LOG, "onboarding tokens transfer failed"; "func" => function_name!());
                }
                true
            } else {
                ONBOARDING_COUNTER.fetch_sub(1, Ordering::SeqCst);
                // info!(LOG, "onboarding token limit reached"; "func" => function_name!());
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
                           email address replaced respectively in the subject line.\n{}\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                           email_addr, RELAYER_EMAIL_ADDRESS.get().unwrap(), ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), wallet_addr, tx_hash
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "relayerEmailAddr": RELAYER_EMAIL_ADDRESS.get().unwrap(), "faucetMessage": ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), "walletAddr":wallet_addr, "transactionHash": tx_hash});
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
            sender.send(email)?;
        }
        EmailWalletEvent::EmailHandled {
            sender_email_addr,
            account_key,
            recipient_email_addr: _,
            original_subject,
            message_id,
            email_op: _,
            tx_hash,
        } => {
            let subject =
                format!("Email Wallet Notification. Your Email Wallet transaction is completed.",);
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&sender_email_addr),
                account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nYour transaction request {} is completed in
                            this transaction https://sepolia.etherscan.io/tx/{}. Thank you for using Email Wallet!\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                            sender_email_addr, original_subject, &tx_hash, wallet_addr, &tx_hash
                        );
            let render_data = serde_json::json!({"userEmailAddr": sender_email_addr, "originalSubject": original_subject, "walletAddr":wallet_addr, "transactionHash": tx_hash});
            let body_html = render_html("email_handled.html", render_data).await?;
            let email = EmailMessage {
                to: sender_email_addr,
                subject,
                body_plain,
                body_html,
                reference: Some(message_id),
                reply_to: Some(RELAYER_EMAIL_ADDRESS.get().unwrap().clone()),
                body_attachments: None,
            };
            sender.send(email)?;
        }
        EmailWalletEvent::AccountNotCreated {
            email_addr,
            account_key,
            // claim,
            is_first: _,
            tx_hash,
        } => {
            // let email_addr = &claim.email_address;
            let subject = format!(
                "Email Wallet Notification. Your Email Wallet account is ready to be deployed.",
            );
            let wallet_salt =
                WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nYour Email Wallet account is ready to be deployed. Your wallet address: https://sepolia.etherscan.io/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.",
                            email_addr, wallet_addr,
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "walletAddr": wallet_addr, "transactionHash": tx_hash});
            let body_html = render_html("account_not_created.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr.to_string(),
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            sender.send(email)?;
        }
        EmailWalletEvent::Claimed {
            claim,
            recipient_account_key,
            tx_hash,
        } => {
            let subject = format!(
                "Email Wallet Notification. {}",
                if claim.is_fund {
                    "You received money on Ethereum"
                } else {
                    "You got some data of Email Wallet extensions"
                }
            );
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&claim.email_address),
                recipient_account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
                            claim.email_address, &tx_hash, wallet_addr
                        );
            let render_data = serde_json::json!({"userEmailAddr": claim.email_address, "walletAddr":wallet_addr, "transactionHash": tx_hash});
            let body_html = render_html("claimed.html", render_data).await?;
            let email = EmailMessage {
                to: claim.email_address,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments: None,
            };
            sender.send(email)?;
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
            let account_key = DB
                .get_account_key(&claim.email_address)
                .await?
                .ok_or(anyhow!("Account not found"))?;
            let account_key = AccountKey(hex2field(&account_key)?);
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&claim.email_address),
                account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
                            claim.email_address, &tx_hash, wallet_addr
                        );
            let render_data = serde_json::json!({"userEmailAddr": claim.email_address, "walletAddr":wallet_addr, "transactionHash": tx_hash});
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
            sender.send(email)?;
        }
        EmailWalletEvent::Error { email_addr, error } => {
            let error = parse_error(error)?;
            if let Some(error) = error {
                let subject = format!("Email Wallet Notification. Error occurred.");
                let body_plain = format!("Hi {}!\nError occurred: {}", email_addr, error);
                let render_data =
                    serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error});
                let body_html = render_html("error.html", render_data).await?;
                let email = EmailMessage {
                    to: email_addr,
                    subject,
                    body_plain,
                    body_html,
                    reference: None,
                    reply_to: None,
                    body_attachments: None,
                };
                sender.send(email)?;
            }
        }
    }

    Ok(())
}

pub(crate) fn parse_error(error: String) -> Result<Option<String>> {
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
        "Account is already created" => {
            // let email = EmailMessage {
            //     to: from_address.clone(),
            //     email_args: EmailArgs::Error {
            //         user_email_addr: from_address,
            //         original_subject: Some(subject),
            //         error_msg: "Account is already created".to_string(),
            //     },
            //     account_key: Some(field2hex(&account_key.0)),
            //     wallet_addr: Some(ethers::utils::to_checksum(&wallet_addr, None)),
            //     tx_hash: None,
            // };
            Ok(Some(error))
        }
        "insufficient balance" => {
            // let email = EmailMessage {
            //     to: from_address.clone(),
            //     email_args: EmailArgs::Error {
            //         user_email_addr: from_address,
            //         original_subject: Some(subject),
            //         error_msg: "You don't have sufficient balance".to_string(),
            //     },
            //     account_key: Some(field2hex(&account_key.0)),
            //     wallet_addr: Some(ethers::utils::to_checksum(&wallet_addr, None)),
            //     tx_hash: None,
            // };
            Ok(Some("You don't have sufficient balance".to_string()))
        }
        _ => Ok(None),
    }
    // match error_email {
    //     Ok(email) => {
    //         tx_sender.send(email).unwrap();
    //         Ok(error)
    //     }
    //     Err(_) => Ok(error),
    // }
}
