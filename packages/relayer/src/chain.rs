// use ethers_core::utils::CompiledContract;
// use ethers_providers::{Http, Middleware, Provider};
// use ethers_signers::{LocalWallet, Signer};

use dotenv::dotenv;
use ethers::abi::Abi;
// use ethers::contract::ContractError;
use ethers::prelude::*;
use anyhow::{Error};
use ethers::core::types::{Address, U256, H160};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
// use hex;
use crate::config::{INCOMING_EML_PATH, LOGIN_ID_KEY, LOGIN_PASSWORD_KEY, SMTP_DOMAIN_NAME_KEY};
use crate::smtp_client::EmailSenderClient;
// use hex_literal::hex;
use k256::ecdsa::SigningKey;
// use rand::thread_rng;
use serde_json::Value;
use std::convert::TryFrom;
use std::env;
// use std::error::Error;
use std::fs;
// use std::borrow::Borrow;
use std::str::{self, FromStr};
// use rustc_hex::{FromHex, ToHex};
// use std::sync::Arc;

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

pub async fn get_signer(force_localhost: bool) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, Error> {
    let chain_id: u64 = std::env::var("CHAIN_ID")
        .expect("The CHAIN_ID environment variable must be set")
        .parse()?;
    let provider = (get_provider(force_localhost).await).unwrap();
    let private_key_hex =
        std::env::var("PRIVATE_KEY").expect("The PRIVATE_KEY environment variable must be set");
    let wallet: LocalWallet = LocalWallet::from_str(&private_key_hex)?;
    let signer = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Ok(signer)
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
    let contract = ContractInstance::new(contract_address, get_abi(AbiType::Wallet).unwrap(), get_signer(force_localhost).await.unwrap());
    let gas_price = get_gas_price(force_localhost).await.unwrap();

    // Read proof and public parameters from JSON files
    let calldata = get_calldata(Some(dir), Some(nonce)).unwrap();

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
        .gas_price(gas_price * 2); // Set an appropriate gas limit

    println!("Calling call: {:?}", call);

    let pending_tx = match call.send().await {
        Ok(tx) => tx,
        Err(e) => {
            println!("Error: {:?}", e);
            reply_with_message(nonce, "Error sending transaction. Most likely your email domain is not supported (must be @gmail.com, @hotmail.com, @ethereum.org, or @skiff.com).");
            println!("Error bytes: {:?}", e.as_revert());
            return Err(e.into());
        }
    };
    println!("Transaction hash: {:?}", pending_tx);
    reply_with_etherscan(nonce, pending_tx.tx_hash());
    Ok(())
}

fn reply_with_etherscan(nonce: &str, tx_hash: H256) {
    let etherscan_url = format!("https://goerli.etherscan.io/tx/0x{:x}", tx_hash);
    let reply = format!(
        "Transaction sent! View Etherscan confirmation: {}. Spot the transfer of the ERC20 for the amount you specified.",
        etherscan_url
    );
    println!("Replying with confirmation...{}", reply);
    reply_with_message(nonce, &reply);
}

fn reply_with_message(nonce: &str, reply: &str) {
    dotenv().ok();
    let mut sender: EmailSenderClient = EmailSenderClient::new(
        env::var(LOGIN_ID_KEY).unwrap().as_str(),
        env::var(LOGIN_PASSWORD_KEY).unwrap().as_str(),
        Some(env::var(SMTP_DOMAIN_NAME_KEY).unwrap().as_str()),
    );
    // Read raw email from received_eml/wallet_{nonce}.eml
    let eml_var = env::var(INCOMING_EML_PATH).unwrap();

    let raw_email = fs::read_to_string(format!("{}/wallet_{}.eml", eml_var, nonce)).unwrap();
    let confirmation = sender.reply_all(&raw_email, &reply);
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
) -> Result<U256, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let abi = get_abi(AbiType::Wallet)?;
    let signer = get_signer(force_localhost).await?;
    let logic_contract_address: Address = std::env::var("CONTRACT_ADDRESS").unwrap().parse()?;
    let logic_contract = ContractInstance::new(logic_contract_address, abi, signer);
    let erc20_address_method = logic_contract.method::<_, Address>("getTokenAddress", "DAI".to_owned())?;
    let erc20_address = erc20_address_method.call().await?;
    let erc_contract = ContractInstance::new(erc20_address, get_abi(AbiType::ERC20).unwrap(), get_signer(force_localhost).await.unwrap());

    // Call the balanceOf function on the ERC20 contract
    let balance: U256 = erc_contract
        .method::<_, U256>("balanceOf", Address::from_str(user_address).unwrap())
        .unwrap()
        .call()
        .await?;

    Ok(balance)
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_query_balance() {
        let balance = query_balance(false, "0x11fE4B6AE13d2a6055C8D9cF65c55bac32B5d844", "DAI").await;
        
        match balance {
            Ok(bal) => {
                assert!(bal > U256::zero(), "Balance must be more than 0");
            },
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "Error getting balance");
            }
        }
    }
}


