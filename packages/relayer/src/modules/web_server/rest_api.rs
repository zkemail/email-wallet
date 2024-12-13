use anyhow::{anyhow, Result};
use lettre::error;

use crate::{
    error, handle_email, handle_email_event, render_html, trace, wallet::EphemeralTx, EmailMessage,
    EmailWalletEvent, RELAYER_EMAIL_ADDRESS,
};
use crate::{CHAIN_RPC_EXPLORER, CLIENT, DB, WEB_SERVER_ADDRESS};
use ethers::{
    etherscan::account,
    types::{Address, Bytes, Signature, U256},
    utils::{hash_message, keccak256, to_checksum},
};
use hex::encode;
use rand::Rng;
use relayer_utils::{
    converters::{field2hex, hex2field},
    cryptos::{AccountCode, AccountSalt, PaddedEmailAddr},
    ParsedEmail, LOG,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::{from_str, Number};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct NFTTransferRequest {
    pub email_addr: String,
    pub nft_id: u64,
    pub nft_addr: String,
    pub recipient_addr: String,
    pub is_recipient_email: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendRequest {
    pub email_addr: String,
    pub amount: Number,
    pub token_id: String,
    pub recipient_addr: String,
    pub is_recipient_email: bool,
}

#[derive(Serialize, Deserialize)]
pub struct IsAccountCreatedRequest {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWalletAddress {
    pub email_addr: String,
    pub account_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecoverAccountCode {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailAddrCommitRequest {
    pub email_address: String,
    pub random: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnclaimRequest {
    pub email_address: String,
    pub random: String,
    pub expiry_time: i64,
    pub is_fund: bool,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationRequest {
    pub email_address: String,
    pub account_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationResponse {
    pub account_code: String,
    pub wallet_addr: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatResponse {
    pub onboarding_tokens_distributed: u32,
    pub onboarding_tokens_left: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SafeRequest {
    pub wallet_addr: String,
    pub safe_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupOrInRequest {
    pub email_addr: String,
    pub ephe_addr: Option<String>,
    pub username: Option<String>,
    pub expiry_time: Option<Number>,
    pub token_allowances: Option<Vec<(Number, String)>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SigninRequest {
//     pub email_addr: String,
//     pub username: String,
//     pub nonce: String,
//     pub expiry_time: Option<Number>,
//     pub token_allowances: Option<Vec<(Number, String)>>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct RegisterEpheAddrRequest {
//     pub wallet_addr: String,
//     pub ephe_addr: String,
//     pub signature: String,
// }

#[derive(Serialize, Deserialize)]
pub struct EpheAddrStatusRequest {
    pub request_id: Number,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct EpheAddrStatusResponse {
    pub is_activated: bool,
    pub wallet_addr: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteEphemeralTxRequest {
    pub wallet_addr: String,
    pub tx_nonce: String,
    pub ephe_addr: String,
    pub ephe_addr_nonce: String,
    pub target: String,
    pub eth_value: String,
    pub data: String,
    pub token_amount: String,
    pub signature: String,
}

pub async fn nft_transfer_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<NFTTransferRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let nft_addr = Address::from_str(&request.nft_addr)?;
    let nft_name = CLIENT.query_nft_name_of_address(nft_addr).await?;
    let subject = format!(
        "NFT Send {} of {} to {}",
        request.nft_id, nft_name, request.recipient_addr
    );
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to send {} your NFT: ID {} of {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr,  request.recipient_addr, request.nft_id, nft_name, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let nft_uri = CLIENT
        .query_erc721_token_uri_of_token(nft_addr, U256::from(request.nft_id))
        .await?;
    let json_uri: Value = serde_json::from_str(
        &String::from_utf8(
            base64::decode(nft_uri.split(",").nth(1).expect("Invalid base64 string"))
                .expect("Failed to decode base64 string"),
        )
        .expect("Invalid UTF-8 sequence"),
    )
    .expect("Failed to parse JSON");
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "nftName": nft_name, "nftID": request.nft_id, "recipientAddr": request.recipient_addr, "walletAddr": wallet_addr, "img":"cid:0", "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "img": json_uri["image"].as_str().unwrap_or_default()});
    let body_html = render_html("nft_transfer.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: Some(vec![]),
    };
    Ok((request_id, email))
}

pub async fn gen_account_code_api_fn() -> Result<String> {
    let account_code = AccountCode::new(rand::thread_rng());
    Ok(field2hex(&account_code.0))
}

pub async fn create_account_api_fn(payload: String) -> Result<(String, EmailMessage)> {
    let request = serde_json::from_str::<CreateAccountRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_code_str = DB.get_account_code(&email_addr).await?;
    if account_code_str.is_none() {
        let account_code = AccountCode::new(rand::thread_rng());
        let invitation_code_hex = &field2hex(&account_code.0)[2..];
        let subject = format!(
            "Email Wallet Account Creation. Code {}",
            invitation_code_hex
        );
        let render_data = serde_json::json!({"userEmailAddr": email_addr.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("account_creation.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr.clone(),
            body_plain: "To create your email wallet, reply to this email.".to_string(),
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        Ok((field2hex(&account_code.0), email))
    } else {
        let subject = "Sign in to your Email Wallet".to_string();
        let error_msg =
            "Your wallet is already created. Please use the login page instead.".to_string();
        // TODO: Get user's account address
        let account_code = AccountCode(hex2field(&account_code_str.clone().unwrap())?);
        let account_salt =
            AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
        let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
        let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountCode": account_code_str.unwrap(), "walletAddr": wallet_addr});
        let body_html = render_html("account_already_exist.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        Ok(("0x".to_string(), email))
    }
}

pub async fn is_account_created_api_fn(payload: String) -> Result<bool> {
    let request = serde_json::from_str::<IsAccountCreatedRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn send_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<SendRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let subject = format!(
        "Send {} {} to {}",
        request.amount, request.token_id, request.recipient_addr
    );
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to send {} {} to {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr, request.amount, request.token_id, request.recipient_addr, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("send_request.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn get_wallet_address_api_fn(payload: String) -> Result<String> {
    let request = serde_json::from_str::<GetWalletAddress>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        return Err(anyhow!(
            "Account key not found for email address: {}",
            request.email_addr
        ));
    }
    let account_code = AccountCode(hex2field(&request.account_code)?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    Ok("0x".to_string() + &encode(&wallet_addr.0))
}

pub async fn recover_account_code_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<RecoverAccountCode>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_code_str = DB.get_account_code(&email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_code_hex = &field2hex(&account_code.0)[2..];
    let account_salt =
        AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let subject = "Email Wallet Account Login".to_string();
    let render_data = serde_json::json!({"userEmailAddr": email_addr, "accountCode": account_code_hex, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("account_recovery.html", render_data).await?;
    let email = EmailMessage {
        subject,
        to: email_addr.clone(),
        body_plain: format!("Hi {}! Your account key is {}, keep it in a safe space.\nYour wallet address: {}/address/{}.", email_addr, account_code_hex, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr),
        body_html,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn add_safe_owner_api_fn(payload: String) -> Result<()> {
    let request = serde_json::from_str::<SafeRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;

    let is_wallet_addr_email_wallet = DB.is_wallet_addr_exist(&request.wallet_addr).await?;

    if is_wallet_addr_email_wallet {
        DB.add_user_with_safe(&request.wallet_addr, &request.safe_addr)
            .await?;
    }

    Ok(())
}

pub async fn delete_safe_owner_api_fn(payload: String) -> Result<()> {
    let request = serde_json::from_str::<SafeRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;

    let is_wallet_addr_email_wallet = DB.is_wallet_addr_exist(&request.wallet_addr).await?;

    if is_wallet_addr_email_wallet {
        DB.remove_safe_from_user(&request.wallet_addr, &request.safe_addr)
            .await?;
    }

    Ok(())
}

pub async fn receive_email_api_fn(email: String) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await.unwrap();
    let from_addr = parsed_email.get_from_addr().unwrap();
    tokio::spawn(async move {
        match handle_email_event(EmailWalletEvent::Ack {
            email_addr: from_addr.clone(),
            subject: parsed_email.get_subject_all().unwrap_or_default(),
            original_message_id: parsed_email.get_message_id().ok(),
        })
        .await
        {
            Ok(_) => {
                trace!(LOG, "Ack email event sent");
            }
            Err(e) => {
                error!(LOG, "Error handling email event: {:?}", e);
            }
        }
        match handle_email(email.clone()).await {
            Ok((event, is_replay)) => {
                match handle_email_event(event.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!(LOG, "Error handling email event: {:?}", e);
                    }
                };
                if is_replay {
                    let event2 = match handle_email(email.clone()).await {
                        Ok((event2, _)) => event2,
                        Err(e) => {
                            let error_event = EmailWalletEvent::Error {
                                email_addr: from_addr,
                                error_subject: parsed_email.get_subject_all().unwrap_or_default(),
                                error: e.to_string(),
                            };
                            error_event
                        }
                    };
                    match handle_email_event(event2).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!(LOG, "Error handling email event: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!(LOG, "Error handling email: {:?}", e);
                match handle_email_event(EmailWalletEvent::Error {
                    email_addr: from_addr,
                    error_subject: parsed_email.get_subject_all().unwrap_or_default(),
                    error: e.to_string(),
                })
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!(LOG, "Error handling email event: {:?}", e);
                    }
                }
            }
        }
    });
    Ok(())
}

pub async fn signup_or_in_api_fn(payload: String) -> Result<(u32, EmailMessage)> {
    let mut request_id: u32 = rand::thread_rng().gen();
    while DB
        .get_ephe_addr_info(&request_id.to_string())
        .await?
        .is_some()
    {
        request_id = rand::thread_rng().gen();
    }
    let request = serde_json::from_str::<SignupOrInRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    let (account_code, code_in_email) = if let Some(code_str) = account_code_str {
        (AccountCode(hex2field(&code_str)?), None)
    } else {
        let account_code = AccountCode::new(rand::thread_rng());
        (
            account_code,
            Some(field2hex(&account_code.0)[2..].to_string()),
        )
    };
    trace!(LOG, "Account code: {:?}", account_code);
    trace!(LOG, "Code in email: {:?}", code_in_email);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    trace!(LOG, "Account salt: {:?}", account_salt);
    let registered_username = CLIENT.get_username_from_wallet(&account_salt).await?;
    trace!(LOG, "Registered Username: {:?}", registered_username);
    let is_signup = registered_username.len() == 0;

    if is_signup && request.username.is_none() {
        let subject = "Email Wallet Error: No username in the sign-up request".to_string();
        let error_msg = "Please specify a username when you sign-up".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    if !is_signup && request.ephe_addr.is_none() {
        let subject = "Email Wallet Error: No ephemeral address in the sign-in request".to_string();
        let error_msg = format!(
            "Please specify an ephemeral address when you sign-in {}",
            &registered_username
        );
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }

    let mut nonce = None;
    // register ephe addr
    if let Some(ephe_addr_str) = request.ephe_addr.as_ref() {
        let ephe_addr = Address::from_str(ephe_addr_str)?;
        let (tx_hash, got_nonce) = CLIENT
            .register_ephe_addr_for_wallet(wallet_addr, ephe_addr)
            .await?;
        trace!(
            LOG,
            "Register ephe addr tx hash: {}, nonce: {}, request: {:?}",
            tx_hash,
            got_nonce,
            request
        );
        nonce = Some(got_nonce);
        println!("request_id int: {}", request_id);
        println!("request_id string: {}", request_id.to_string());
        DB.insert_ephe_addr_info(
            &request_id.to_string(),
            &encode(&wallet_addr.0),
            &ephe_addr_str,
            &got_nonce.to_string(),
        )
        .await?;
    }

    let prefix = if is_signup { "Sign-up" } else { "Sign-in" };
    let used_username = if is_signup {
        request.username.as_ref().unwrap()
    } else {
        &registered_username
    };
    let mut subject = _construct_sign_up_in_subject(
        prefix,
        used_username,
        nonce.map(|s| s.to_string()),
        request.expiry_time,
        request.token_allowances,
    );
    if let Some(code) = code_in_email {
        subject = format!("{} Code {}", subject, code);
    }

    let body_plain = format!(
        "Hi {}! Please reply to this email to {} {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr
        if is_signup { "sign-up" } else { "sign-in" },
        used_username, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("send_request.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

// pub async fn signin_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
//     let request_id = rand::thread_rng().gen();
//     let request = serde_json::from_str::<SigninRequest>(&payload)
//         .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
//     let subject = _construct_sign_up_in_subject(
//         "Sign-in",
//         request.username.as_str(),
//         Some(request.nonce.as_str()),
//         request.expiry_time,
//         request.token_allowances,
//     );
//     let account_code_str = DB.get_account_code(&request.email_addr).await?;
//     if account_code_str.is_none() {
//         let subject = "Email Wallet Error: Account Not Found".to_string();
//         let error_msg =
//             "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
//         let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
//         let body_html = render_html("error.html", render_data).await?;
//         let email = EmailMessage {
//             subject,
//             to: request.email_addr,
//             body_plain: error_msg,
//             body_html,
//             reference: None,
//             reply_to: None,
//             body_attachments: None,
//         };
//         return Ok((request_id, email));
//     }
//     let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
//     let account_salt = AccountSalt::new(
//         &PaddedEmailAddr::from_email_addr(&request.email_addr),
//         account_code,
//     )?;
//     let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
//     let body_plain = format!(
//         "Hi {}! Please reply to this email to sign-in {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
//         request.email_addr, request.username, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
//     );
//     let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
//     let body_html = render_html("send_request.html", render_data).await?;
//     let email = EmailMessage {
//         subject: subject.clone(),
//         body_html,
//         body_plain,
//         to: request.email_addr,
//         reference: None,
//         reply_to: None,
//         body_attachments: None,
//     };
//     Ok((request_id, email))
// }

// pub async fn register_ephe_addr(payload: String) -> Result<U256> {
//     let request = serde_json::from_str::<RegisterEpheAddrRequest>(&payload)
//         .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
//     let wallet_addr = Address::from_str(&request.wallet_addr)?;
//     let ephe_addr = Address::from_str(&request.ephe_addr)?;
//     let signature = Bytes::from_str(&request.signature)?;
//     let (tx_hash, nonce) = CLIENT
//         .register_ephe_addr_for_wallet(wallet_addr, ephe_addr, signature)
//         .await?;
//     trace!(
//         LOG,
//         "Register ephe addr tx hash: {}, nonce: {}, request: {:?}",
//         tx_hash,
//         nonce,
//         request
//     );
//     Ok(nonce)
// }

pub async fn ephe_addr_status_api_fn(payload: String) -> Result<EpheAddrStatusResponse> {
    let request = serde_json::from_str::<EpheAddrStatusRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let ephe_addr_info = DB
        .get_ephe_addr_info(&request.request_id.to_string())
        .await?;
    if ephe_addr_info.is_none() {
        error!(LOG, "Ephe addr info not found");
        return Ok(EpheAddrStatusResponse {
            is_activated: false,
            wallet_addr: None,
            nonce: None,
        });
    }
    trace!(LOG, "Ephe addr info: {:?}", ephe_addr_info);
    let (wallet_addr_str, ephe_addr_str, nonce) = ephe_addr_info.unwrap();
    trace!(LOG, "Wallet addr str: {}", &wallet_addr_str);
    let wallet_addr = Address::from_str(&wallet_addr_str)?;
    trace!(LOG, "Wallet addr: {}", wallet_addr);
    let ephe_addr = Address::from_str(&ephe_addr_str)?;
    trace!(LOG, "Ephe addr: {}", ephe_addr);
    // verify if request.signature
    let signed_msg = format!(
        "{}:/api/epheAddrStatus/{}",
        RELAYER_EMAIL_ADDRESS.get().unwrap(),
        request.request_id
    );
    trace!(LOG, "Signed msg: {}", signed_msg);
    let signed_msg_hash = hash_message(&signed_msg);
    let signature = Signature::from_str(&request.signature)?;
    trace!(LOG, "signature: {:?}", signature);
    let recovered_addr = signature.recover(signed_msg_hash)?;
    trace!(LOG, "Recovered address: {:?}", recovered_addr);
    if recovered_addr != ephe_addr {
        error!(LOG, "Recovered address does not match ephe addr");
        return Ok(EpheAddrStatusResponse {
            is_activated: false,
            wallet_addr: None,
            nonce: None,
        });
    }
    if CLIENT
        .validate_ephe_addr(wallet_addr, ephe_addr, U256::from_str_radix(&nonce, 10)?)
        .await
        .is_err()
    {
        trace!(LOG, "Ephe addr not activated");
        return Ok(EpheAddrStatusResponse {
            is_activated: false,
            wallet_addr: None,
            nonce: None,
        });
    }

    let wallet_addr_checksumed = to_checksum(&wallet_addr, None);
    Ok(EpheAddrStatusResponse {
        is_activated: true,
        wallet_addr: Some(wallet_addr_checksumed),
        nonce: Some(nonce),
    })
}

pub async fn execute_ephemeral_tx(payload: String) -> Result<String> {
    let request = serde_json::from_str::<ExecuteEphemeralTxRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let tx = EphemeralTx {
        wallet_addr: Address::from_str(&request.wallet_addr)?,
        tx_nonce: U256::from_str_radix(&request.tx_nonce, 10)?,
        ephe_addr: Address::from_str(&request.ephe_addr)?,
        ephe_addr_nonce: U256::from_str_radix(&request.ephe_addr_nonce, 10)?,
        target: Address::from_str(&request.target)?,
        eth_value: U256::from_str_radix(&request.eth_value, 10)?,
        data: Bytes::from_str(&request.data)?,
        token_amount: U256::from_str_radix(&request.token_amount, 10)?,
        signature: Bytes::from_str(&request.signature)?,
    };
    let tx_hash = CLIENT.execute_ephemeral_tx(tx).await?;
    trace!(
        LOG,
        "Execute ephemeral tx hash: {}, request: {:?}",
        tx_hash,
        request
    );
    Ok(tx_hash)
}

fn _construct_sign_up_in_subject(
    prefix: &str,
    username: &str,
    nonce: Option<String>,
    expiry_time: Option<Number>,
    token_allowances: Option<Vec<(Number, String)>>,
) -> String {
    let mut subject_words = vec![prefix.to_string()];
    subject_words.push(username.to_string());
    if let Some(nonce) = nonce {
        subject_words.push("on device".to_string());
        subject_words.push(nonce.to_string());
    }
    if let Some(expiry_time) = expiry_time {
        subject_words.push("until timestamp".to_string());
        subject_words.push(expiry_time.to_string());
    }
    if let Some(token_allowances) = token_allowances {
        subject_words.push("for".to_string());
        for (amount, token_name) in token_allowances {
            subject_words.push(format!("{} {}", amount.to_string(), token_name));
        }
    }
    subject_words.join(" ")
}
