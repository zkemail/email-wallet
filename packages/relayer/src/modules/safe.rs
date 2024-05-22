use axum::http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct SafeResponse {
    count: u32,
    #[serde(rename = "next")]
    next: Option<String>, // Adjusted to String if the type is known and always a URL or null
    #[serde(rename = "previous")]
    previous: Option<String>, // Adjusted similarly
    results: Vec<SafeTransaction>,
    #[serde(rename = "countUniqueNonce")]
    count_unique_nonce: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct SafeTransaction {
    safe: String,
    to: String,
    value: String,
    #[serde(rename = "data")]
    data: Option<Value>,
    operation: u8,
    #[serde(rename = "gasToken")]
    gas_token: Option<String>,
    #[serde(rename = "safeTxGas")]
    safe_tx_gas: Option<u64>,
    #[serde(rename = "baseGas")]
    base_gas: Option<u64>,
    #[serde(rename = "gasPrice")]
    gas_price: Option<String>,
    #[serde(rename = "refundReceiver")]
    refund_receiver: Option<String>,
    nonce: u64,
    #[serde(rename = "executionDate")]
    execution_date: Option<Value>,
    #[serde(rename = "submissionDate")]
    submission_date: Option<String>,
    modified: String,
    #[serde(rename = "blockNumber")]
    block_number: Option<Value>,
    #[serde(rename = "transactionHash")]
    transaction_hash: Option<Value>,
    #[serde(rename = "safeTxHash")]
    safe_tx_hash: Option<String>,
    proposer: String,
    #[serde(rename = "executor")]
    executor: Option<Value>,
    #[serde(rename = "isExecuted")]
    is_executed: Option<bool>,
    #[serde(rename = "isSuccessful")]
    is_successful: Option<Value>,
    #[serde(rename = "ethGasPrice")]
    eth_gas_price: Option<Value>,
    #[serde(rename = "maxFeePerGas")]
    max_fee_per_gas: Option<Value>,
    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: Option<Value>,
    #[serde(rename = "gasUsed")]
    gas_used: Option<Value>,
    #[serde(rename = "fee")]
    fee: Option<Value>,
    origin: Value,
    #[serde(rename = "dataDecoded")]
    data_decoded: Option<Value>,
    #[serde(rename = "confirmationsRequired")]
    confirmations_required: Option<u32>,
    #[serde(rename = "confirmations")]
    confirmations: Option<Vec<Confirmation>>,
    trusted: bool,
    #[serde(rename = "signatures")]
    signatures: Option<Value>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct Confirmation {
    owner: String,
    #[serde(rename = "submissionDate")]
    submission_date: Option<String>,
    #[serde(rename = "transactionHash")]
    transaction_hash: Option<Value>,
    signature: String,
    #[serde(rename = "signatureType")]
    signature_type: Option<String>,
}

pub async fn safe_fn(email_sender: EmailForwardSender) -> Result<()> {
    // Get all wallet address from safe db
    let wallet_addresses = DB.get_users_with_safe().await.unwrap();
    for wallet_addr in wallet_addresses {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));

        let safes = DB.get_safes_by_user(&wallet_addr).await.unwrap();

        for safe in safes {
            let txn_url = format!(
                "{}/v1/safes/{}/multisig-transactions/",
                SAFE_API_ENDPOINT.get().unwrap(),
                safe
            );
            let txn_response = client.get(txn_url).headers(headers.clone()).send().await?;
            let txn_body = txn_response.text().await?;
            let txns: SafeResponse = serde_json::from_str(&txn_body)?;

            for txn in txns.results {
                let safe_txn_hash = txn.safe_tx_hash.unwrap_or_default();
                let confirmations_required =
                    txn.confirmations_required.unwrap_or_default() as usize;
                if confirmations_required <= txn.confirmations.as_ref().map_or(0, |c| c.len())
                    || txn.confirmations.as_ref().map_or(false, |c| {
                        c.iter()
                            .any(|confirmation| confirmation.owner == wallet_addr)
                    })
                    || DB.has_safe_tx_by_addr(&safe_txn_hash, &wallet_addr).await?
                {
                    continue;
                }
                println!("Approving safe_txn_hash: {}", safe_txn_hash);
                let email_addr = DB.get_email_by_wallet(&wallet_addr).await.unwrap();
                let account_key_str = DB.get_account_key(&email_addr).await?;
                let account_key = AccountKey(hex2field(&account_key_str.unwrap())?);
                let wallet_salt =
                    WalletSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_key)?;
                let subject = format!("Safe Transaction: Approve {} from {}", safe_txn_hash, safe);
                let render_data = serde_json::json!({
                    "userEmailAddr": email_addr,
                    "safeTransactionHash": safe_txn_hash,
                    "chainRpcExplorer": CHAIN_RPC_EXPLORER.get().unwrap(),
                    "walletAddr": wallet_addr,
                });
                let body_html = render_html("safe_txn.html", render_data).await?;
                let email = EmailMessage {
                    to: email_addr.clone(),
                    subject,
                    reference: None,
                    reply_to: None,
                    body_plain: format!(
                        "Hi {}! You have a new safe transaction to approve.",
                        email_addr
                    ),
                    body_html,
                    body_attachments: None,
                };
                email_sender.send(email).unwrap();
                DB.insert_safe_tx(&safe_txn_hash, &wallet_addr).await?;
            }
            // Delay to ensure not exceeding 5 requests per second
            sleep(Duration::from_millis(200)).await; // 1000 ms / 5 = 200 ms
        }
    }
    Ok(())
}
