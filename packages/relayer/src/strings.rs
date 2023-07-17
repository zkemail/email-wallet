use ethers::core::types::{Address, U256, H160, H256};
use crate::chain::{query_balance};
pub const CHAIN: &str = "Ethereum Goerli";

pub fn first_reply(amount: &str, currency: &str, recipient: &str) -> String {
    format!(
        "Valid send initiated. Sending {} {} from you to {} on {}. \
        We will follow up with Etherscan link when finished! \n \nYou are sending with ZK email technology. \
        The relayer will automatically prove, on-chain, that you sent an email authorizing this transaction.  \
        We will deploy a wallet for senders or recipients if you don't have one, where the private keys are \
        managed by your email provider. You can send up to 10 'TEST' tokens for free if you want to try it out!",
        amount, currency, recipient, CHAIN
    )
}

pub fn invalid_reply(reason: &str) -> String {
    format!(
        "Send {}! Please try again with this subject: \"Send _ dai to __@__.___\". \
    You can send DAI, ETH, or TEST tokens right now.",
        reason
    )
}

pub fn reply_with_etherscan(tx_hash: H256) -> String {
    let etherscan_url = format!("https://goerli.etherscan.io/tx/0x{:x}", tx_hash);
    let reply = format!(
        "Transaction sent! View Etherscan confirmation: {}. Spot the transfer of the ERC20 for the amount you specified!",
        etherscan_url
    );
    println!("Replying with confirmation...{}", reply);
    reply
}

pub async fn pending_reply(address: &str, amount: &str, currency: &str, recipient: &str) -> String {
    let mut enough_balance = false;
    let balance_detected_message = match query_balance(
        false,
        address.clone(),
        currency,
    )
    .await {
        Ok(balance) => {
            enough_balance = balance >= amount.parse().unwrap();
            let remaining = balance - amount.parse::<f64>().unwrap();
            
            let enough_balance_str = if enough_balance {
                format!("Your wallet {} has {} {}. The transaction will send {} {} to {} and your remaining balance will be {} {}", address, balance, currency, amount, currency, recipient, remaining, currency)
            } else {
                format!("Created new wallet for you at {} -- in order to send this transaction, you must add at least {} {} to send. \
                The send has been queued and will execute once enough balance is detected, then automatically send {} {} to {}.",
                address, amount, currency, amount, currency, recipient)
            };
            enough_balance_str
        },
        Err(_) => {
            format!("Failed to detect balance in account.")
        }
    };
    println!("Balance detected message: {}", balance_detected_message);
       

    format!(
        "{} \
        We will follow up with {} Etherscan link when finished! \n \nYou are sending with ZK email technology. \
        The relayer will automatically prove, on-chain, that you sent an email authorizing this transaction. \
        We will deploy a wallet for you if you don't have one. While we're in beta, we've given each email \
        address 10 'TEST' tokens to test out free transfers.",
        balance_detected_message, CHAIN
    )
}