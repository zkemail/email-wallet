use anyhow::{anyhow, Result};

use email_wallet_utils::{
    converters::hex2field,
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use ethers::types::{Address};

use rand::Rng;
use relayer::CLIENT;
use relayer::*;
use serde::{Deserialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct NFTTransferRequest {
    email_addr: String,
    nft_id: u64,
    nft_addr: String,
    recipient_addr: String,
    is_recipient_email: bool,
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
            "Your wallet is not yet created. Please create your Email Wallet first.".to_string();
        let render_data =
            serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
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
        "Hi {}! To send your NFT {} with id {} to {}, please reply to this email!\nYour wallet address: https://sepolia.etherscan.io/address/{}.",
        request.email_addr, nft_name, request.nft_id, request.recipient_addr, wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "nftName": nft_name, "nftId": request.nft_id, "recipientAddr": request.recipient_addr, "walletAddr": wallet_addr, });
    let body_html = render_html("nft_transfer.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        // body: format!(
        //     "NFT Send {} of {} to {}",
        //     request.nft_id, nft_name, request.recipient_addr
        // ),
        to: request.email_addr,
        reference: None,
        reply_to: None,
    };
    Ok((request_id, email))
}
