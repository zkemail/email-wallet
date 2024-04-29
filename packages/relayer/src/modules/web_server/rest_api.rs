use anyhow::{anyhow, Result};

use crate::smtp_client::*;
use email_wallet_utils::{
    converters::{field2hex, hex2field},
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use ethers::types::{Address, U256};

use crate::{CHAIN_RPC_EXPLORER, CLIENT, DB};
use hex::encode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_json::Value;
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
    pub account_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecoverAccountKey {
    pub email_addr: String,
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
    let account_key_str = DB.get_account_key(&request.email_addr).await?;
    if account_key_str.is_none() {
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
    let account_key = AccountKey(hex2field(&account_key_str.unwrap())?);
    let wallet_salt = WalletSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_key,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
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

pub async fn create_account_api_fn(payload: String) -> Result<(String, EmailMessage)> {
    let request = serde_json::from_str::<CreateAccountRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_key_str = DB.get_account_key(&email_addr).await?;
    if account_key_str.is_none() {
        let account_key = AccountKey::new(rand::thread_rng());
        let invitation_code_hex = &field2hex(&account_key.0)[2..];
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
        Ok((field2hex(&account_key.0), email))
    } else {
        let subject = "Email Wallet Error: Account Already Exists".to_string();
        let error_msg =
            "Your wallet is already created. Please use the login page instead.".to_string();
        // TODO: Get user's account address
        let account_key = AccountKey(hex2field(&account_key_str.clone().unwrap())?);
        let wallet_salt =
            WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
        let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
        let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountKey": account_key_str.unwrap(), "walletAddr": wallet_addr});
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
    let account_key_str = DB.get_account_key(&request.email_addr).await?;
    if account_key_str.is_none() {
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
    let account_key_str = DB.get_account_key(&request.email_addr).await?;
    if account_key_str.is_none() {
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
    let account_key = AccountKey(hex2field(&account_key_str.unwrap())?);
    let wallet_salt = WalletSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_key,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
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
    let account_key_str = DB.get_account_key(&request.email_addr).await?;
    if account_key_str.is_none() {
        return Err(anyhow!(
            "Account key not found for email address: {}",
            request.email_addr
        ));
    }
    let account_key = AccountKey(hex2field(&request.account_key)?);
    let wallet_salt = WalletSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_key,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
    Ok("0x".to_string() + &encode(&wallet_addr.0))
}

pub async fn recover_account_key_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<RecoverAccountKey>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_key_str = DB.get_account_key(&email_addr).await?;
    if account_key_str.is_none() {
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
    let account_key = AccountKey(hex2field(&account_key_str.unwrap())?);
    let account_key_hex = &field2hex(&account_key.0)[2..];
    let wallet_salt = WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
    let subject = "Email Wallet Account Login".to_string();
    let render_data = serde_json::json!({"userEmailAddr": email_addr, "accountKey": account_key_hex, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("account_recovery.html", render_data).await?;
    let email = EmailMessage {
        subject,
        to: email_addr.clone(),
        body_plain: format!("Hi {}! Your account key is {}, keep it in a safe space.\nYour wallet address: {}/address/{}.", email_addr, account_key_hex, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr),
        body_html,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}
