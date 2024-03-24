use crate::{
    error, hex2field, parse_error, render_html, AccountKey, EmailForwardSender, EmailMessage,
    EmailWalletEvent, Future, Result, CHAIN_RPC_EXPLORER, CLIENT, DB, LOG, ONBOARDING_COUNTER,
    ONBOARDING_REPLY_MSG, ONBOARDING_TOKEN_DISTRIBUTION_LIMIT, RELAYER_EMAIL_ADDRESS,
};
use anyhow::anyhow;
use email_wallet_utils::cryptos::{PaddedEmailAddr, WalletSalt};
use std::{pin::Pin, sync::atomic::Ordering};

pub fn event_consumer(
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
            let subject = format!("Your Email Wallet Account is created.",);
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
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "relayerEmailAddr": RELAYER_EMAIL_ADDRESS.get().unwrap(), "faucetMessage": ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
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
            let subject = format!("Your Email Wallet transaction is completed.",);
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&sender_email_addr),
                account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
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
            let subject = format!("Your Email Wallet account is ready to be deployed.",);
            let wallet_salt =
                WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nYour Email Wallet account is ready to be deployed. Your wallet address: {}/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.",
                            email_addr, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "walletAddr": wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
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
            unclaimed_fund: _,
            unclaimed_state: _,
            email_addr,
            is_fund,
            is_announced: _,
            recipient_account_key,
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
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&email_addr),
                recipient_account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let body_plain = format!(
                            "Hi {}!\nCheck the transaction for you on etherscan: {}/tx/{}.\nNote that your wallet address is {}\n",
                            email_addr, CHAIN_RPC_EXPLORER.get().unwrap(), &tx_hash, wallet_addr
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
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
            sender.send(email)?;
        }
        EmailWalletEvent::Error { email_addr, error } => {
            let error = parse_error(error)?;
            if let Some(error) = error {
                let subject = format!("Email Wallet Notification. Error occurred.");
                let body_plain = format!("Hi {}!\nError occurred: {}", email_addr, error);
                let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
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
        EmailWalletEvent::Ack {
            email_addr,
            subject,
        } => {
            let body_plain = format!(
                "Hi {}!\nYour email with the subject {} is received.",
                email_addr, subject
            );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "request": subject});
            let body_html = render_html("acknowledgement.html", render_data).await?;
            let subject = format!("Email Wallet Notification. Acknowledgement.");
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

    Ok(())
}
