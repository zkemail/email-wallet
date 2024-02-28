use anyhow::{anyhow, Result};
use chrono::format;
use email_wallet_utils::{
    converters::{field2hex, hex2field},
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use serde_json::Value;

use ethers::types::{Address, U256};

use relayer::*;

use slog::{error, info};
use std::future::Future;
use std::sync::OnceLock;
use std::{env, pin::Pin};

mod rest_api;
use rest_api::*;
mod utils;
use dotenv::dotenv;
use utils::*;

pub static DEMO_NFT_ADDR: OnceLock<Address> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    }
    dotenv::dotenv().ok();
    DEMO_NFT_ADDR
        .set(
            env::var("DEMO_NFT_ADDR")
                .map(|s| s.parse().unwrap())
                .unwrap(),
        )
        .unwrap();
    let (sender, rx) = EmailForwardSender::new();
    let sender_for_nft_transfer_api = sender.clone();
    let nft_transfer_fn = axum::routing::post::<_, _, (), _>(move |payload: String| async move {
        info!(LOG, "NFT transfer payload: {}", payload);
        match nft_transfer_api_fn(payload).await {
            Ok((request_id, email)) => {
                sender_for_nft_transfer_api.send(email).unwrap();
                request_id.to_string()
            }
            Err(err) => {
                error!(relayer::LOG, "Failed to accept nft transfer: {}", err);
                err.to_string()
            }
        }
    });
    let sender_for_create_account_api = sender.clone();
    let create_account_fn = axum::routing::post::<_, _, (), _>(move |payload: String| async move {
        info!(LOG, "Create account payload: {}", payload);
        match create_account_api_fn(payload).await {
            Ok((request_id, email)) => {
                sender_for_create_account_api.send(email).unwrap();
                request_id.to_string()
            }
            Err(err) => {
                error!(relayer::LOG, "Failed to accept create account: {}", err);
                err.to_string()
            }
        }
    });
    let is_account_created_fn =
        axum::routing::post::<_, _, (), _>(move |payload: String| async move {
            info!(LOG, "Is account created payload: {}", payload);
            match is_account_created_api_fn(payload).await {
                Ok(status) => status.to_string(),
                Err(err) => {
                    error!(relayer::LOG, "Failed to accept is account created: {}", err);
                    err.to_string()
                }
            }
        });
    let sender_for_send_fn_api = sender.clone();
    let send_fn = axum::routing::post::<_, _, (), _>(move |payload: String| async move {
        info!(LOG, "Send payload: {}", payload);
        match send_api_fn(payload).await {
            Ok((request_id, email)) => {
                sender_for_send_fn_api.send(email).unwrap();
                request_id.to_string()
            }
            Err(err) => {
                error!(relayer::LOG, "Failed to accept send: {}", err);
                err.to_string()
            }
        }
    });
    let get_wallet_address_fn =
        axum::routing::post::<_, _, (), _>(move |payload: String| async move {
            info!(LOG, "Get wallet address payload: {}", payload);
            match get_wallet_address_api_fn(payload).await {
                Ok(wallet_addr) => wallet_addr,
                Err(err) => {
                    error!(relayer::LOG, "Failed to accept get wallet address: {}", err);
                    err.to_string()
                }
            }
        });
    let sender_for_recover_account_key_fn_api = sender.clone();
    let recover_account_key_fn =
        axum::routing::post::<_, _, (), _>(move |payload: String| async move {
            info!(LOG, "Recover account key payload: {}", payload);
            match recover_account_key_api_fn(payload).await {
                Ok((request_id, email)) => {
                    sender_for_recover_account_key_fn_api.send(email).unwrap();
                    request_id.to_string()
                }
                Err(err) => {
                    error!(
                        relayer::LOG,
                        "Failed to accept recover account key: {}", err
                    );
                    err.to_string()
                }
            }
        });
    let echo_fn = axum::routing::get(move || async move { "Hello, world!" });

    let routes = vec![
        ("/api/nftTransfer".to_string(), nft_transfer_fn),
        ("/api/isAccountCreated".to_string(), is_account_created_fn),
        ("/api/createAccount".to_string(), create_account_fn),
        ("/api/send".to_string(), send_fn),
        ("/api/getWalletAddress".to_string(), get_wallet_address_fn),
        ("/api/recoverAccountKey".to_string(), recover_account_key_fn),
        ("/api/echo".to_string(), echo_fn),
    ];
    run(RelayerConfig::new(), event_consumer, sender, rx, routes).await?;
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
            let invitation_code_hex = &field2hex(&account_key.0)[2..];
            let subject = "Your Email Wallet Account is created.".to_string();
            let wallet_salt =
                WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let test_balance = CLIENT
                .query_user_erc20_balance(&wallet_salt, "TEST")
                .await?;
            if test_balance < U256::from(100) {
                let amount = U256::from(100) - test_balance;
                CLIENT.free_mint_test_erc20(wallet_addr, amount).await?;
            }
            // CLIENT
            //     .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
            //     .await?;
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
            let render_data = serde_json::json!({"userEmailAddr": email_addr, "relayerEmailAddr": RELAYER_EMAIL_ADDRESS.get().unwrap(), "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountKey": invitation_code_hex});
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
                            this transaction https://sepolia.etherscan.io/tx/{}. Thank you for using Email Wallet!\nYour wallet address: https://sepolia.etherscan.io/address/{}.\nCheck the transaction on etherscan: https://sepolia.etherscan.io/tx/{}",
                            sender_email_addr, original_subject, &tx_hash, wallet_addr, &tx_hash
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
            is_first,
            tx_hash: _,
        } => {
            let email = generate_invitation_email(&email_addr, account_key, is_first).await?;
            sender.send(email)?;
        }
        EmailWalletEvent::Claimed {
            unclaimed_fund,
            unclaimed_state,
            email_addr,
            is_fund,
            is_announced,
            recipient_account_key,
            tx_hash,
        } => {
            let account_key_str = &field2hex(&recipient_account_key.0)[2..];
            let wallet_salt = WalletSalt::new(
                &PaddedEmailAddr::from_email_addr(&email_addr),
                recipient_account_key,
            )?;
            let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
            let (subject, body_plain, body_html, body_attachments) = if is_fund {
                let unclaim_fund = unclaimed_fund.unwrap();
                let token_decimal = CLIENT
                    .query_decimals_of_erc20_address(unclaim_fund.token_addr)
                    .await?;
                let amount =
                    uint_to_decimal_string(unclaim_fund.amount.as_u128(), token_decimal as usize);
                let mut name = CLIENT.query_token_name(unclaim_fund.token_addr).await?;
                if name == "WETH" {
                    name = "ETH".to_string();
                }
                let subject = format!("You received {} {}.", amount, name);
                let body_plain = format!(
                            "Hi {}!\nYou received {} {} from {}.\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
                            email_addr, amount, name, unclaim_fund.sender, &tx_hash, wallet_addr
                        );
                let render_data = serde_json::json!({"userEmailAddr": email_addr, "tokenAmount": amount, "tokenName": name, "senderAddr": unclaim_fund.sender, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountKey": account_key_str});
                let body_html = render_html("claimed_fund.html", render_data).await?;
                (subject, body_plain, body_html, None)
            } else {
                let unclaimed_state = unclaimed_state.unwrap();
                info!(LOG, "unclaimed state {:?}", unclaimed_state);
                if unclaimed_state.extension_addr
                    == CLIENT.query_default_extension_for_command("NFT").await?
                {
                    let (nft_addr, nft_id, nft_name, nft_uri) =
                        get_nft_info(&unclaimed_state.state).await?;
                    let json_uri: Value = serde_json::from_str(
                        &String::from_utf8(
                            base64::decode(
                                nft_uri.split(",").nth(1).expect("Invalid base64 string"),
                            )
                            .expect("Failed to decode base64 string"),
                        )
                        .expect("Invalid UTF-8 sequence"),
                    )
                    .expect("Failed to parse JSON");
                    let subject =
                        format!("You received NFT #{} of {}.", nft_id.to_string(), nft_name);
                    let body_plain = format!(
                            "Hi {}!\nYou received the NFT with ID {} of {} from {}.\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
                            email_addr, nft_id.to_string(), nft_name, unclaimed_state.sender, &tx_hash, wallet_addr
                        );
                    let render_data = serde_json::json!({"userEmailAddr": email_addr, "nftId": nft_id.to_string(), "nftName": nft_name, "senderAddr": unclaimed_state.sender, "walletAddr":wallet_addr, "transactionHash": tx_hash, "img": format!("cid:{}", 0), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountKey": account_key_str, "img": json_uri["image"].as_str().unwrap_or_default()});
                    let body_html = render_html("claimed_nft.html", render_data).await?;
                    // let img =
                    //     download_img_from_uri(&json_uri["image"].as_str().unwrap_or_default())
                    //         .await?;
                    // let attachment = EmailAttachment {
                    //     inline_id: "NFT".to_string(),
                    //     content_type: "image/png".to_string(),
                    //     contents: img,
                    // };
                    (subject, body_plain, body_html, Some(vec![]))
                } else {
                    let subject = format!(
                        "You received data of extension {}.",
                        unclaimed_state.extension_addr
                    );
                    let body_plain = format!(
                            "Hi {}!\nYou received data of extension {} from {}.\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
                            email_addr, unclaimed_state.extension_addr, unclaimed_state.sender, &tx_hash, wallet_addr
                        );
                    let render_data = serde_json::json!({"userEmailAddr": email_addr, "extensionAddr": unclaimed_state.extension_addr, "senderAddr": unclaimed_state.sender, "walletAddr":wallet_addr, "transactionHash": tx_hash, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountKey": account_key_str});
                    let body_html = render_html("claimed_extension.html", render_data).await?;
                    (subject, body_plain, body_html, None)
                }
            };
            // let body_plain = format!(
            //                 "Hi {}!\nCheck the transaction for you on etherscan: https://sepolia.etherscan.io/tx/{}.\nNote that your wallet address is {}\n",
            //                 claim.email_address, &tx_hash, wallet_addr
            //             );
            // let render_data = serde_json::json!({"userEmailAddr": claim.email_address, "walletAddr":wallet_addr, "transactionHash": tx_hash});
            // let body_html = render_html("claimed.html", render_data).await?;
            let email = EmailMessage {
                to: email_addr,
                subject,
                body_plain,
                body_html,
                reference: None,
                reply_to: None,
                body_attachments,
            };
            sender.send(email)?;
        }
        EmailWalletEvent::Voided { claim, tx_hash } => {
            let subject = format!(
                "{}",
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
            "You can claim an ETHDenver NFT. Code {}",
            invitation_code_hex
        )
    } else {
        format!(
            "Your Email Wallet Account is ready to be deployed. Code {}",
            invitation_code_hex
        )
    };
    let (assets_list_plain, assets_list_html) =
        generate_asset_list_body(&assets, assets_msgs).await?;
    let body_plain = if is_for_nft_demo {
        format!(
            "Hi {}!\nYou can claim ETHDenver NFT. Your wallet address: https://sepolia.etherscan.io/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.\nAfter creating the wallet, you can receive the following assets!\n{}",
            email_addr, wallet_addr, assets_list_plain,
        )
    } else {
        format!(
            "Hi {}!\nYour Email Wallet account is ready to be deployed. Your wallet address: https://sepolia.etherscan.io/address/{}.\nPlease reply to this email to start using Email Wallet. You don't have to add any message in the reply 😄.\nAfter creating the wallet, you can receive the following assets!\n{}",
            email_addr, wallet_addr, assets_list_plain
        )
    };
    let email_msg = if is_for_nft_demo {
        let html = render_html(
            "invitation_nft.html",
            serde_json::json!({
                "userEmailAddr": email_addr,
                "walletAddr": wallet_addr,
                "assetsList": assets_list_html,
                "invitationCode": invitation_code_hex,
                "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(),
            }),
        )
        .await?;
        EmailMessage {
            to: email_addr.to_string(),
            subject,
            body_plain,
            body_html: html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        }
    } else {
        let html = render_html(
            "invitation.html",
            serde_json::json!({
                "userEmailAddr": email_addr,
                "walletAddr": wallet_addr,
                "assetsList": assets_list_html,
                "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(),
            }),
        )
        .await?;
        EmailMessage {
            to: email_addr.to_string(),
            subject,
            body_plain,
            body_html: html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        }
    };
    // if is_first {
    //     CLIENT
    //         .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
    //         .await?;
    // }
    Ok(email_msg)
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
        "Account is not created" => Ok(Some(
            "Your account is not created. Please reply to an email containing \"Code ~\" first."
                .to_string(),
        )),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Duration, Utc};
    use email_wallet_utils::*;
    use ethers::{abi, contract::abigen, types::Address};
    use ethers::{
        abi::Token,
        core::k256::elliptic_curve::Field,
        types::{Bytes, U256},
    };
    use rand::rngs::OsRng;
    use std::str::FromStr;
    use tokio;

    abigen!(
        ERC721Mintable,
        r"[
        function safeMint(address to, uint256 tokenId, string memory uri)
    ]"
    );

    #[test]
    #[ignore]
    fn test_nft_mint() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            test_nft_mint_inner().await;
        });
    }

    async fn test_nft_mint_inner() {
        dotenv().ok();
        PRIVATE_KEY.set(env::var("PRIVATE_KEY").unwrap()).unwrap();
        CHAIN_ID
            .set(env::var("CHAIN_ID").unwrap().parse().unwrap())
            .unwrap();
        CHAIN_RPC_PROVIDER
            .set(env::var("CHAIN_RPC_PROVIDER").unwrap())
            .unwrap();
        CHAIN_RPC_EXPLORER
            .set(env::var("CHAIN_RPC_EXPLORER").unwrap())
            .unwrap();
        CORE_CONTRACT_ADDRESS
            .set(env::var("CORE_CONTRACT_ADDRESS").unwrap())
            .unwrap();
        let email_addr = "suegamisora@gmail.com";
        // let token_name = "APE";
        let token_addr = Address::from_str("0x1095F49b9d7A980847467C2A71b4231c0A6C208E").unwrap();
        let token_id = U256::from(32);
        let relayer_addr = Address::from_str("0x17E60b84C20CeE3DF59BF2A4E34252053A2B9C38").unwrap();
        let uri = "https://ipfs.io/ipfs/QmQEVVLJUR1WLN15S49rzDJsSP7za9DxeqpUzWuG4aondg".to_string();
        let nft_mintable = ERC721Mintable::new(token_addr, CLIENT.core.client());
        let call = nft_mintable.safe_mint(relayer_addr, token_id, uri);
        let tx = call.send().await.unwrap();
        tx.log().confirmations(1).await.unwrap();
        // let recipient_addr = "alice@gmail.com";
        let relayer_url = "http://localhost:4500";

        let padded_email_addr = PaddedEmailAddr::from_email_addr(email_addr);
        let rand = Fr::random(OsRng);
        println!("rand {}", field2hex(&rand));
        let email_addr_commit = padded_email_addr.to_commitment(&rand).unwrap();
        let extension_addr = CLIENT
            .query_default_extension_for_command("NFT")
            .await
            .unwrap();
        println!("extension_addr {}", extension_addr);
        let tx_hash = CLIENT
            .approve_erc721(token_addr.clone(), extension_addr.clone(), token_id.clone())
            .await
            .unwrap();
        println!("approve tx hash {}", tx_hash);
        let state = abi::encode(&[Token::Address(token_addr), Token::Uint(token_id)]);
        let now = Utc::now();
        let one_day_later = now + Duration::days(1);
        let timestamp = one_day_later.timestamp();
        let tx_hash = CLIENT
            .register_unclaimed_state(
                email_addr_commit,
                extension_addr,
                Bytes::from_iter(state),
                U256::from(timestamp),
                None,
                None,
            )
            .await
            .unwrap();
        println!("register unclaimed state tx hash {}", tx_hash);
        let request = UnclaimRequest {
            email_address: email_addr.to_string(),
            random: field2hex(&rand),
            expiry_time: timestamp,
            is_fund: false,
            tx_hash: tx_hash.to_string(),
        };
        let payload = serde_json::to_string(&request).unwrap();
        reqwest::Client::new()
            .post(format!("{}/api/unclaim", relayer_url))
            .body(payload)
            .send()
            .await
            .unwrap();
        // let request = NFTTransferRequest {
        //     email_addr: email_addr.to_string(),
        //     nft_id: token_id.as_u64(),
        //     nft_addr: token_addr.to_string(),
        //     recipient_addr: recipient_addr.to_string(),
        //     is_recipient_email: true,
        // };
        // let payload = serde_json::to_string(&request).unwrap();
        // reqwest::Client::new()
        //     .post(format!("{}/api/nftTransfer", relayer_url))
        //     .body(payload)
        //     .send()
        //     .await
        //     .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn nft_transfer_test() {
        let email_addr = "suegamisora@gmail.com";
        let relayer_url = "http://localhost:4500";
        let token_id = 31u64;
        let token_addr = "0x1095F49b9d7A980847467C2A71b4231c0A6C208E";
        let recipient_addr = "0x17E60b84C20CeE3DF59BF2A4E34252053A2B9C38";
        let request = NFTTransferRequest {
            email_addr: email_addr.to_string(),
            nft_id: token_id,
            nft_addr: token_addr.to_string(),
            recipient_addr: recipient_addr.to_string(),
            is_recipient_email: false,
        };
        let payload = serde_json::to_string(&request).unwrap();
        reqwest::Client::new()
            .post(format!("{}/api/nftTransfer", relayer_url))
            .body(payload)
            .send()
            .await
            .unwrap();
    }
}
