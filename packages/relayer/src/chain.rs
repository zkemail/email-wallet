// use ethers_core::utils::CompiledContract;
// use ethers_providers::{Http, Middleware, Provider};
// use ethers_signers::{LocalWallet, Signer};

use dotenv::dotenv;
use ethers::abi::Abi;
// use ethers::contract::ContractError;
use ethers::prelude::*;
use anyhow::{Error};
use ethers::core::types::{Address, U256, H160, H256};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use crate::strings::{reply_with_etherscan};
use crate::config::{INCOMING_EML_PATH, ETHERSCAN_KEY, LOGIN_ID_KEY, LOGIN_PASSWORD_KEY, SMTP_DOMAIN_NAME_KEY};
use crate::smtp_client::EmailSenderClient;
// use hex_literal::hex;
use k256::ecdsa::SigningKey;
use serde_json::Value;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::{self, FromStr};
// use std::error::Error;
// use rand::thread_rng;
// use std::borrow::Borrow;
// use rustc_hex::{FromHex, ToHex};
// use std::sync::Arc;

pub type SignerType = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;
pub type ClientType = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
#[derive(Debug, Clone)]
pub struct CircomCalldata {
    pi_a: [U256; 2],
    pi_b: [[U256; 2]; 2],
    pi_c: [U256; 2],
    signals: [U256; 27],
}

// Define a new function that takes optional arguments and provides default values
pub fn get_calldata(dir: Option<&str>, nonce: Option<&str>) -> Result<CircomCalldata, Error> {
    // Provide default values if arguments are not specified
    let dir = dir.unwrap_or("");
    let nonce = nonce.unwrap_or("");

    // Call the main function with the specified or default values
    parse_files_into_calldata(dir, nonce)
}

// #[tokio::main]
// async
fn parse_files_into_calldata(
    dir: &str,
    nonce: &str,
) -> Result<CircomCalldata, Error> {
    let proof_dir = dir.to_owned() + "rapidsnark_proof_" + nonce + ".json";
    let proof_json: Value = serde_json::from_str(&fs::read_to_string(proof_dir).unwrap()).unwrap();
    let public_json: Value = serde_json::from_str(
        &fs::read_to_string(dir.to_owned() + "rapidsnark_public_" + nonce + ".json").unwrap(),
    )
    .unwrap();

    let pi_a: [U256; 2] = [
        U256::from_dec_str(proof_json["pi_a"][0].as_str().unwrap()).unwrap(),
        U256::from_dec_str(proof_json["pi_a"][1].as_str().unwrap()).unwrap(),
    ];

    let pi_b_raw: [[U256; 2]; 2] = [
        [
            U256::from_dec_str(proof_json["pi_b"][0][0].as_str().unwrap()).unwrap(),
            U256::from_dec_str(proof_json["pi_b"][0][1].as_str().unwrap()).unwrap(),
        ],
        [
            U256::from_dec_str(proof_json["pi_b"][1][0].as_str().unwrap()).unwrap(),
            U256::from_dec_str(proof_json["pi_b"][1][1].as_str().unwrap()).unwrap(),
        ],
    ];

    // Swap the G2 points to be the correct order with the new snarkjs
    let pi_b_swapped: Vec<[U256; 2]> = pi_b_raw.iter().map(|inner| [inner[1], inner[0]]).collect();

    // Convert the Vec to an array
    let pi_b: [[U256; 2]; 2] = [pi_b_swapped[0], pi_b_swapped[1]];

    let pi_c: [U256; 2] = [
        U256::from_dec_str(proof_json["pi_c"][0].as_str().unwrap()).unwrap(),
        U256::from_dec_str(proof_json["pi_c"][1].as_str().unwrap()).unwrap(),
    ];

    let signals_vec = public_json
        .as_array()
        .unwrap()
        .iter()
        .map(|x| U256::from_dec_str(x.as_str().unwrap()).unwrap())
        .collect::<Vec<_>>();

    println!("signals_vec: {:?}", signals_vec);

    let signals: [U256; 27] = signals_vec
        .as_slice()
        .try_into()
        .unwrap();

    let calldata = CircomCalldata {
        pi_a,
        pi_b,
        pi_c,
        signals,
    };
    Ok(calldata)
}

pub enum AbiType {
    Wallet,
    ERC20,
    TokenRegistry,
}

pub async fn get_provider(force_localhost: bool) -> Result<Provider<Http>, Error> {
    // let alchemy_api_key = std::env::var("ALCHEMY_GOERLI_KEY").unwrap();
    // println!("alchemy_api_key: {}", alchemy_api_key);
    let rpcurl = if force_localhost {
        "http://localhost:8548".to_string()
    } else {
        std::env::var("RPC_URL").expect("The RPC_URL environment variable must be set")
    };
    // Get the private key from the environment variable
    println!("Got provider for rpcurl: {}", rpcurl);

    let provider = Provider::<Http>::try_from(rpcurl)?;
    Ok(provider)
}

pub async fn get_gas_price(force_localhost: bool) -> Result<U256, Error> {
    let provider = (get_provider(force_localhost).await).unwrap();
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

pub fn get_abi(abi_type: AbiType) -> Result<Abi, Error> {
    // Read the contents of the ABI file as bytes
    let abi_bytes: &[u8] = match abi_type {
        AbiType::Wallet => include_bytes!("../abi/wallet.abi"),
        AbiType::TokenRegistry => include_bytes!("../abi/tokenRegistry.abi"),
        AbiType::ERC20 => include_bytes!("../abi/erc20.abi"),
    };
    // Convert the bytes to a string
    let abi_str = str::from_utf8(abi_bytes)?;
    // Parse the string as JSON to obtain the ABI
    let abi_json: Value = serde_json::from_str(abi_str)?;
    // Convert the JSON value to the Abi type
    let abi: Abi = serde_json::from_value(abi_json)?;
    Ok(abi)
}

pub async fn get_signer(force_localhost: bool) -> Result<SignerType, Error> {
    let chain_id: u64 = std::env::var("CHAIN_ID")
        .expect("The CHAIN_ID environment variable must be set")
        .parse()?;
    let provider = (get_provider(force_localhost).await).unwrap();
    let private_key_hex =
        std::env::var("PRIVATE_KEY").expect("The PRIVATE_KEY environment variable must be set");
    let wallet: LocalWallet = LocalWallet::from_str(&private_key_hex)?;
    let address = wallet.address();
    let signer = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Ok(signer)
}

#[derive(Debug, serde::Deserialize)]
struct EtherscanResponse {
    status: String,
    message: String,
    result: Vec<Transaction>,
}

#[derive(Debug, serde::Deserialize)]
struct Transaction {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    nonce: String,
    blockHash: String,
    transactionIndex: String,
    from: String,
    to: String,
    value: String,
    gas: String,
    gasPrice: String,
    isError: String,
    txreceipt_status: String,
    input: String,
    contractAddress: String,
    cumulativeGasUsed: String,
    gasUsed: String,
    confirmations: String,
}

pub async fn get_pending_tx_count(force_localhost: bool, wallet_address: H160) -> Result<usize, Error> {
    Ok(0)
    // Query the current nonce of the account
    // let provider = (get_provider(force_localhost).await).unwrap();
    // let current_nonce = provider.get_transaction_count(wallet_address, None).await.unwrap();

    // // Get pending txes
    // let etherscan_api_key = std::env::var(ETHERSCAN_KEY).expect("The ETHERSCAN_API_KEY environment variable must be set");
    // let chain_id: u64 = std::env::var("CHAIN_ID").expect("The CHAIN_ID environment variable must be set").parse().unwrap();
    // let etherscan_url = match chain_id {
    //     5 => "https://goerli.etherscan.io",
    //     1 => "https://etherscan.io",
    //     42161 => "https://arbiscan.io",
    //     10 => "https://optimistic.etherscan.io",
    //     _ => panic!("Unsupported chain id"),
    // };
    // let url = format!("{}/api?module=account&action=txlist&address={}&startblock=0&endblock=999999999&sort=asc&apikey={}", etherscan_url, wallet_address, etherscan_api_key);
    // println!("Etherscan Url: {}", url);
    // let response_text = reqwest::get(&url).await?.text().await?;
    // println!("API response: {}", response_text);
    // let response: EtherscanResponse = serde_json::from_str(&response_text)?;

    // // let response = reqwest::get(&url).await.expect("Failed to send pending tx request to Etherscan").json::<EtherscanResponse>().await.expect("Failed to parse Etherscan return value");

    // let pending_transactions: Vec<Transaction> = response.result.into_iter().filter(|tx| tx.txreceipt_status == "Pending").collect();
    // let pending_count = pending_transactions.len();

    // println!("Pending transactions for {}: {}", wallet_address, pending_count);
    // Ok(pending_count)
}

// local: bool - whether or not to send to a local RPC
// dir: data directory where the intermediate rapidsnark inputs/proofs will be stored
pub async fn send_to_chain(
    force_localhost: bool,
    dir: &str,
    nonce: &str,
) -> Result<(), Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let contract_address: Address = std::env::var("CONTRACT_ADDRESS").unwrap().parse()?;
    let signer_raw = get_signer(force_localhost).await.unwrap();
    let sender_address = signer_raw.address().clone();
    let signer = signer_raw.nonce_manager(sender_address);

    let gas_price = get_gas_price(force_localhost).await.unwrap();

    // Read proof and public parameters from JSON files
    let calldata = get_calldata(Some(dir), Some(nonce)).unwrap();

    // Initialize NonceManagerMiddleware
    // let nonce_manager = NonceManagerMiddleware::new(signer, sender_address);
        // let contract: ContractInstance<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, Abi> =
        // ContractInstance::new(contract_address, get_abi(AbiType::Wallet).unwrap(), signer);
    let contract = ContractInstance::<_, ClientType>::new(contract_address, get_abi(AbiType::Wallet).unwrap(), &signer);
    // let contract = ContractInstance::new(contract_address, get_abi(AbiType::Wallet).unwrap(), signer);

    signer.initialize_nonce(None).await?;

    let pending_txes = get_pending_tx_count(force_localhost, sender_address).await.expect("Pending tx count failed");
    for _ in 0..pending_txes {
        let mut _nonce = signer.next();
    }

    println!("Sending transaction with gas price {:?}...", gas_price);

    // Call the transfer function
    let call = contract
        .method::<_, ()>(
            "transfer",
            (
                calldata.pi_a,
                calldata.pi_b,
                calldata.pi_c,
                calldata.signals,
            ),
        )?
        .gas_price(gas_price * 2);

    println!("Calling call: {:?}", call);

    // Send the transaction with the updated nonce
    let pending_tx = match call.send().await {
        Ok(tx) => tx,
        Err(e) => {
            println!("Error: {:?}", e);
            reply_with_message(nonce, "Error sending transaction. Most likely your email domain is not supported (must be @gmail.com, @hotmail.com, @ethereum.org, or @skiff.com).", false);
            println!("Error bytes: {:?}", e.as_revert());
            return Err(e.into());
        }
    };
    println!("Transaction hash: {:?}", pending_tx);
    let etherscan_reply = reply_with_etherscan(pending_tx.tx_hash());
    reply_with_message(nonce, &etherscan_reply, true);
    Ok(())
}

fn reply_with_message(nonce: &str, reply: &str, send_to_recipient: bool) {
    dotenv().ok();
    let mut sender: EmailSenderClient = EmailSenderClient::new(
        env::var(LOGIN_ID_KEY).unwrap().as_str(),
        env::var(LOGIN_PASSWORD_KEY).unwrap().as_str(),
        Some(env::var(SMTP_DOMAIN_NAME_KEY).unwrap().as_str()),
    );
    // Read raw email from received_eml/wallet_{nonce}.eml
    let eml_var = env::var(INCOMING_EML_PATH).unwrap();

    let raw_email = fs::read_to_string(format!("{}/wallet_{}.eml", eml_var, nonce)).unwrap();
    let confirmation = sender.reply_all(&raw_email, &reply, send_to_recipient);
}

pub async fn query_address(
    force_localhost: bool,
    user_salt: &str,
) -> Result<H160, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let abi = get_abi(AbiType::Wallet)?;
    let signer = get_signer(force_localhost).await?;
    let logic_contract_address: Address = std::env::var("CONTRACT_ADDRESS").unwrap().parse()?;
    let logic_contract = ContractInstance::new(logic_contract_address, abi, signer);
    let decimal_salt_u256 = U256::from_dec_str(&user_salt)?;
    let address_method = logic_contract.method::<_, Address>("getOrCreateWallet", decimal_salt_u256)?;
    let address = address_method.call().await?;
    Ok(address)
}

// Given an address and token, get the balance of that token for that address from the chain
// This can be done on a local light node or fork to ensure future tx data is not leaked
pub async fn query_balance(
    force_localhost: bool,
    user_address: &str,
    token_name: &str,
) -> Result<f64, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let abi = get_abi(AbiType::Wallet)?;
    let signer = get_signer(force_localhost).await?;
    let logic_contract_address: Address = std::env::var("CONTRACT_ADDRESS").unwrap().parse()?;
    let logic_contract = ContractInstance::new(logic_contract_address, abi, signer);
    let erc20_address_method = logic_contract.method::<_, Address>("getTokenAddress", "DAI".to_owned())?;
    let erc20_address = erc20_address_method.call().await?;
    let erc_contract = ContractInstance::new(erc20_address, get_abi(AbiType::ERC20).unwrap(), get_signer(force_localhost).await.unwrap());
    // Call the balanceOf function on the ERC20 contract to get the raw balance in wei
    let raw_balance: U256 = erc_contract
        .method::<_, U256>("balanceOf", Address::from_str(user_address).unwrap())
        .unwrap()
        .call()
        .await?;

    // Call the decimals function on the ERC20 contract to get the decimal count
    let decimals: U256 = erc_contract
        .method::<_, U256>("decimals", ())
        .unwrap()
        .call()
        .await?;

    // Calculate the balance in tokens by dividing the raw balance by 10 to the power of the decimal count
    let balance: f64 = raw_balance.low_u64() as f64 / 10f64.powi(decimals.as_u32() as i32);
    Ok(balance)
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_query_balance() {
        dotenv::dotenv().ok();
        let balance = query_balance(false, "0x11fE4B6AE13d2a6055C8D9cF65c55bac32B5d844", "DAI").await;

        match balance {
            Ok(bal) => {
                assert!(bal > 0.0, "Balance must be more than 0");
            },
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "Error getting balance");
            }
        }
    }

    #[tokio::test]
    async fn test_get_pending_tx_count() {
        dotenv::dotenv().ok();
        let wallet_address = "0x11fE4B6AE13d2a6055C8D9cF65c55bac32B5d844".parse().unwrap();
        let pending_tx_count = get_pending_tx_count(false, wallet_address).await;

        match pending_tx_count {
            Ok(count) => {
                assert!(count >= 0, "Pending transaction count must be non-negative");
            },
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "Error getting pending transaction count");
            }
        }
    }
}


