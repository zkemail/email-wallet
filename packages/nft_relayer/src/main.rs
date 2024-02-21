use anyhow::{anyhow, Result};
use email_wallet_utils::{
    converters::{field2hex, hex2field},
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use ethers::abi::{self, ParamType, Token};
use ethers::types::{Address, Bytes, I256, U256};
use handlebars::Handlebars;
use relayer::*;
use serde_json::Value;
use slog::{error, info};
use std::sync::{Arc, OnceLock};
use std::{env, path::PathBuf, sync::atomic::Ordering};
use tokio::fs::read_to_string;
mod rest_api;
use rest_api::*;
mod utils;
use utils::*;

pub static DEMO_NFT_ADDR: OnceLock<Address> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        dotenv::dotenv().ok();
        DEMO_NFT_ADDR
            .set(
                env::var("DEMO_NFT_ADDR")
                    .map(|s| s.parse().unwrap())
                    .unwrap(),
            )
            .unwrap();
        let (sender, rx) = EmailForwardSender::new();
        let sender_for_event = sender.clone();
        let event_consumer_fn = move |event: EmailWalletEvent| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(event_consumer_fn(event, sender_for_event.clone()))
        };
        let event_consumer = EventConsumer::new(Box::new(event_consumer_fn));
        let sender_for_api = sender.clone();
        let nft_transfer_fn =
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "NFT transfer payload: {}", payload);
                match nft_transfer_api_fn(payload).await {
                    Ok((request_id, email)) => {
                        sender_for_api.send(email).unwrap();
                        request_id.to_string()
                    }
                    Err(err) => {
                        error!(relayer::LOG, "Failed to accept nft transfer: {}", err);
                        err.to_string()
                    }
                }
            });

        let routes = vec![("/api/nftTransfer".to_string(), nft_transfer_fn)];

        run(RelayerConfig::new(), event_consumer, rx, routes).await?;
    }
    Ok(())
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
            // CLIENT
            //     .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
            //     .await?;
            // // Distribute onboarding tokens
            // let current_count = ONBOARDING_COUNTER.fetch_add(1, Ordering::SeqCst);
            // if current_count < *ONBOARDING_TOKEN_DISTRIBUTION_LIMIT.get().unwrap() {
            //     if CLIENT
            //         .transfer_onboarding_tokens(wallet_addr)
            //         .await
            //         .is_err()
            //     {
            //         ONBOARDING_COUNTER.fetch_sub(1, Ordering::SeqCst);
            //         // info!(LOG, "onboarding tokens transfer failed"; "func" => function_name!());
            //     }
            //     true
            // } else {
            //     ONBOARDING_COUNTER.fetch_sub(1, Ordering::SeqCst);
            //     // info!(LOG, "onboarding token limit reached"; "func" => function_name!());
            //     false
            // };
            let body_plain = format!(
                            "Awesome, {}!\nYour Email Wallet account is now
                           created. PLEASE DO NOT DELETE THIS EMAIL to keep your account
                           secure.\nYou
                           can send 10 TEST tokens directly to emailwallet.relayer@gmail.com by sending us
                           ({}) an email with the subject \"Send 10 TEST to
                           emailwallet.relayer@gmail.com\".\nSimilarly,
                           you can send any currency we support directly to an email address by
                           sending an email with the amount, currency name, and recipient's
                           email address replaced respectively in the subject line.\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                           email_addr, RELAYER_EMAIL_ADDRESS.get().unwrap(),  wallet_addr, tx_hash
                        );
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "relayerEmailAddr": RELAYER_EMAIL_ADDRESS.get().unwrap(), "walletAddr":wallet_addr, "transactionHash": tx_hash});
            let body_html = render_html("account_created.html", render_data).await?;
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
            let body_html = render_html("email_handled.html", render_data).await?;
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
        EmailWalletEvent::AccountNotCreated {
            email_addr,
            account_key,
            is_first,
            tx_hash: _,
        } => {
            let email = generate_invitation_email(&email_addr, account_key, is_first).await?;
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
                };
                sender.send(email)?;
            }
        }
    }

    Ok(())
}

pub(crate) async fn generate_invitation_email(
    email_addr: &str,
    account_key: AccountKey,
    is_first: bool,
) -> Result<EmailMessage> {
    let wallet_salt = WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
    let mut is_for_nft_demo = false;
    let assets_msgs = vec!["ERC20: 100 TEST".to_string()];
    // let mut images = vec![];
    let assets = search_user_assets(email_addr).await?;
    for asset in assets.iter() {
        if let Asset::ERC721 {
            token_addr,
            token_name: _,
            token_id: _,
            token_uri: _,
        } = asset
        {
            is_for_nft_demo = token_addr == DEMO_NFT_ADDR.get().unwrap();
        }
    }
    let invitation_code_hex = &field2hex(&account_key.0)[2..];
    let subject = if is_for_nft_demo {
        format!(
            "Email Wallet Notification. You can claim ETH Denver NFT. Code {}",
            invitation_code_hex
        )
    } else {
        format!(
            "Email Wallet Notification. Your Email Wallet Account is ready to be deployed. Code {}",
            invitation_code_hex
        )
    };
    let (assets_list_plain, assets_list_html) =
        generate_asset_list_body(&assets, assets_msgs).await?;
    let body_plain = if is_for_nft_demo {
        format!(
            "Hi {}!\nYou can claim ETH Denver NFT. Your wallet address: https://sepolia.etherscan.io/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.\nAfter creating the wallet, you can recive the following assets!\n{}",
            email_addr, wallet_addr, assets_list_plain,
        )
    } else {
        format!(
            "Hi {}!\nYour Email Wallet account is ready to be deployed. Your wallet address: https://sepolia.etherscan.io/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.\nAfter creating the wallet, you can recive the following assets!\n{}",
            email_addr, wallet_addr, assets_list_plain
        )
    };
    let body_html = if is_for_nft_demo {
        render_html(
            "invitation_nft.html",
            serde_json::json!({
                "userEmailAddr": email_addr,
                "walletAddr": wallet_addr,
                "assetsList": assets_list_html,
            }),
        )
        .await?
    } else {
        render_html(
            "invitation.html",
            serde_json::json!({
                "userEmailAddr": email_addr,
                "walletAddr": wallet_addr,
                "assetsList": assets_list_html,
            }),
        )
        .await?
    };
    if is_first {
        CLIENT
            .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
            .await?;
    }
    Ok(EmailMessage {
        to: email_addr.to_string(),
        subject,
        body_plain,
        body_html,
        reference: None,
        reply_to: None,
    })
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
        "Account is already created" => Ok(Some(error)),
        "insufficient balance" => Ok(Some("You don't have sufficient balance".to_string())),
        _ => Ok(None),
    }
}
