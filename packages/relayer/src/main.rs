use anyhow::{Ok, Result};
use email_wallet_utils::cryptos::{PaddedEmailAddr, WalletSalt};
use handlebars::Handlebars;
use relayer::*;
use serde_json::Value;
use slog::info;
use std::{env, path::PathBuf, sync::atomic::Ordering};
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        let (sender, rx) = EmailForwardSender::new();
        let event_consumer_fn = move |event: EmailWalletEvent| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(event_consumer_fn(event, sender.clone()))
            // match event {
            //     EmailWalletEvent::AccountCreated {
            //         user_email_addr,
            //         account_key,
            //         tx_hash,
            //     } => {
            //         let subject = format!(
            //             "Email Wallet Notification. Your Email Wallet Account is created.",
            //         );
            //         let wallet_salt = WalletSalt::new(
            //             &PaddedEmailAddr::from_email_addr(&user_email_addr),
            //             account_key,
            //         )?;
            //         let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            //         let token_transfered = CLIENT
            //             .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
            //             .await?;
            //         let body_plain = format!(
            //                         "Awesome, {}!\nYour Email Wallet account is now
            //                        created. PLEASE DO NOT DELETE THIS EMAIL to keep your account
            //                        secure.\nYou
            //                        can send 10 TEST tokens directly to emailwallet.relayer@gmail.com by sending us
            //                        ({}) an email with the subject \"Send 10 TEST to
            //                        emailwallet.relayer@gmail.com\".\nSimilarly,
            //                        you can send any currency we support directly to an email address by
            //                        sending an email with the amount, currency name, and recipient's
            //                        email address replaced respectively in the subject line.\n{}\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
            //                         user_email_addr, RELAYER_EMAIL_ADDRESS.get().unwrap(), faucet_message.clone().unwrap_or(String::new()), wallet_addr, tx_hash
            //                     );
            //     }
            //     EmailWalletEvent::PsiRegistered {
            //         email_addr,
            //         account_key,
            //         tx_hash,
            //     } => {}
            //     _ => {}
            // }
        };
        let event_consumer = EventConsumer::new(Box::new(event_consumer_fn));
        let routes = vec![];
        run(RelayerConfig::new(), event_consumer, rx, routes).await?;
    }
    Ok(())
}

async fn event_consumer_fn(event: EmailWalletEvent, sender: EmailForwardSender) -> Result<()> {
    match event {
        EmailWalletEvent::AccountCreated {
            user_email_addr,
            account_key,
            tx_hash,
        } => {
            let subject =
                format!("Email Wallet Notification. Your Email Wallet Account is created.",);
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&user_email_addr),
                account_key,
            )?;
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
                            user_email_addr, RELAYER_EMAIL_ADDRESS.get().unwrap(), ONBOARDING_REPLY_MSG.get().clone().unwrap_or(&String::new()), wallet_addr, tx_hash
                        );
            let render_data =
                serde_json::json!({"userEmailAddr": user_email_addr, "transactionHash": tx_hash});
            let body_html = render_html("account_creation.html", render_data).await?;
            let email = EmailMessage {
                to: user_email_addr,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
            };
            sender.send(email)?;
        }
        EmailWalletEvent::EmailHandled {
            sender_email_addr,
            account_key,
            recipient_email_addr,
            original_subject,
            message_id,
            email_op,
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
            let body_html = render_html("transaction_complete.html", render_data).await?;
            let email = EmailMessage {
                to: sender_email_addr,
                subject,
                body_plain,
                body_html,
                reference: Some(message_id),
                reply_to: Some(RELAYER_EMAIL_ADDRESS.get().unwrap().clone()),
            };
            sender.send(email)?;
        }
        EmailWalletEvent::PsiRegistered {
            email_addr,
            account_key,
            tx_hash,
        } => {
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
            let render_data =
                serde_json::json!({"userEmailAddr": email_addr, "walletAddr": wallet_addr,});
            let body_html = render_html("psi_registered.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
            };
            sender.send(email)?;
        }
        EmailWalletEvent::Claimed {
            claim,
            recipient_account_key,
            tx_hash,
        } => {}
        _ => {}
    }

    Ok(())
}
