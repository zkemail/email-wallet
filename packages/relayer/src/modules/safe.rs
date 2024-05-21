use axum::http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
struct SafeResponse {
    count: u32,
    next: Option<Value>, // Using Value to handle any type, change if the type is known
    previous: Option<Value>, // Same as above
    results: Vec<SafeTransaction>,
    count_unique_nonce: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SafeTransaction {
    safe: String,
    to: String,
    value: String,
    data: Option<Value>, // Using Value to handle null or other structures
    operation: u8,
    gas_token: String,
    safe_tx_gas: u64,
    base_gas: u64,
    gas_price: String,
    refund_receiver: String,
    nonce: u64,
    execution_date: Option<Value>, // Using Value for potential null or date string
    submission_date: String,
    modified: String,
    block_number: Option<Value>, // Using Value to handle null or numbers
    transaction_hash: Option<Value>, // Using Value to handle null or string
    safe_tx_hash: String,
    proposer: String,
    executor: Option<Value>, // Using Value for potential null or address string
    is_executed: bool,
    is_successful: Option<Value>, // Using Value to handle null or boolean
    eth_gas_price: Option<Value>, // Using Value for potential null or string
    max_fee_per_gas: Option<Value>, // Using Value for potential null or string
    max_priority_fee_per_gas: Option<Value>, // Using Value for potential null or string
    gas_used: Option<Value>,      // Using Value for potential null or number
    fee: Option<Value>,           // Using Value for potential null or string
    origin: Value,                // Could be an empty object or more complex data
    data_decoded: Option<Value>,  // Using Value to handle null or other structures
    confirmations_required: u32,
    confirmations: Vec<Confirmation>,
    trusted: bool,
    signatures: Option<Value>, // Using Value to handle null or array of signatures
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation {
    owner: String,
    submission_date: String,
    transaction_hash: Option<Value>, // Using Value to handle null or string
    signature: String,
    signature_type: String,
}

pub async fn safe_fn(email_sender: EmailForwardSender) -> Result<()> {
    // Get all wallet address from safe db
    let wallet_addresses = DB.get_users_with_safe().await.unwrap();
    for wallet_addr in wallet_addresses {
        let url = format!(
            "{}/v1/safes/{}/",
            SAFE_API_ENDPOINT.get().unwrap(),
            wallet_addr
        );
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));

        let response = client.get(url).headers(headers.clone()).send().await?;
        let body = response.text().await?;
        let safes: Value = serde_json::from_str(&body)?;

        if let Some(safes_array) = safes.get("safes") {
            if let Some(array) = safes_array.as_array() {
                for safe in array {
                    let txn_url = format!(
                        "{}/v1/safes/{}/multisig-transactions/",
                        SAFE_API_ENDPOINT.get().unwrap(),
                        safe.as_str().unwrap_or_default()
                    );
                    let txn_response = client.get(txn_url).headers(headers.clone()).send().await?;
                    let txn_body = txn_response.text().await?;
                    let txns: SafeResponse = serde_json::from_str(&txn_body)?;

                    for txn in txns.results {
                        let txn_hash = txn.transaction_hash.unwrap_or_default();
                        let confirmations_required = txn.confirmations_required;
                        if confirmations_required as usize >= txn.confirmations.len()
                            || txn
                                .confirmations
                                .iter()
                                .any(|confirmation| confirmation.owner == wallet_addr)
                        {
                            continue;
                        }
                        if DB
                            .has_approved_safe_tx_by_addr(txn_hash.as_str().unwrap(), &wallet_addr)
                            .await?
                        {
                            continue;
                        }
                        let email_addr = DB.get_email_by_wallet(&wallet_addr).await.unwrap();
                        let account_key_str = DB.get_account_key(&email_addr).await?;
                        let account_key = AccountKey(hex2field(&account_key_str.unwrap())?);
                        let wallet_salt = WalletSalt::new(
                            &PaddedEmailAddr::from_email_addr(&email_addr),
                            account_key,
                        )?;
                        let subject = format!(
                            "Safe Transaction: Approve {} from {}",
                            txn_hash,
                            safe.as_str().unwrap()
                        );
                        let render_data = serde_json::json!({
                            "userEmailAddr": email_addr,
                            "safeTransactionHash": txn_hash,
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
                        DB.insert_safe_tx(txn_hash.as_str().unwrap(), &wallet_addr)
                            .await?;
                    }
                    // Delay to ensure not exceeding 5 requests per second
                    sleep(Duration::from_millis(200)).await; // 1000 ms / 5 = 200 ms
                }
            }
        } else {
            DB.delete_user_with_safe(&wallet_addr).await.unwrap();
        }
    }
    Ok(())
}
