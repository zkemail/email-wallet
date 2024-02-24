use anyhow::{anyhow, Result};

use email_wallet_utils::{
    converters::hex2field,
    cryptos::{AccountKey, PaddedEmailAddr, WalletSalt},
};
use ethers::types::{Address, U256};

use rand::Rng;
use relayer::CLIENT;
use relayer::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::download_img_from_uri;

#[derive(Serialize, Deserialize)]
pub struct NFTTransferRequest {
    pub email_addr: String,
    pub nft_id: u64,
    pub nft_addr: String,
    pub recipient_addr: String,
    pub is_recipient_email: bool,
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
        "Hi {}! Please reply to this email to send {} your NFT: ID {} of {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: https://sepolia.etherscan.io/address/{}.",
        request.email_addr,  request.recipient_addr, request.nft_id, nft_name,wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "nftName": nft_name, "nftID": request.nft_id, "recipientAddr": request.recipient_addr, "walletAddr": wallet_addr, "img":"cid:0" });
    let body_html = render_html("nft_transfer.html", render_data).await?;
    let nft_uri = CLIENT
        .query_erc721_token_uri_of_token(nft_addr, U256::from(request.nft_id))
        .await?;
    let img = download_img_from_uri(&nft_uri).await?;
    let attachment = EmailAttachment {
        inline_id: "0".to_string(),
        content_type: "image/png".to_string(),
        contents: img,
    };
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: Some(vec![attachment]),
    };
    Ok((request_id, email))
}
