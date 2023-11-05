// query a list tokens from json url
use crate::abis::TokenRegistry;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::signers::Signer;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const CONFIRMATIONS: usize = 1;
const UNISWAP_TOKEN_LIST: &str =
    "https://raw.githubusercontent.com/Uniswap/default-token-list/main/src/tokens/mainnet.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    name: String,
    symbol: String,
    address: H160,
}

struct TokenRegistryUtil {
    chain_id: u64,

    rpc_url: String,

    token_registry: H160,

    private_key: String,
}

impl TokenRegistryUtil {
    pub async fn populate_token_registry(&self) {
        println!("CHAIN_ID: {}", &self.chain_id);
        println!("RPC_URL: {}", &self.rpc_url);
        println!("TOKEN_REGISTRY: {}", &self.token_registry);
        println!("Populating token registry...");

        let tokens = &self.get_popular_tokens().await;

        // Update registry in chunks of 50
        let chunks: Vec<&[Token]> = tokens.chunks(50).collect();
        for chunk in chunks {
            self.add_token_to_registry(chunk).await;
        }
    }

    async fn get_popular_tokens(&self) -> Vec<Token> {
        let tokens: Vec<Token> = reqwest::Client::new()
            .get(UNISWAP_TOKEN_LIST)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        return tokens;
    }

    pub async fn add_token_to_registry(&self, tokens: &[Token]) {
        let chain_id_u256 = U256::from(self.chain_id);
        let wallet: LocalWallet = self.private_key.parse().unwrap();
        let provider = Provider::<Http>::try_from(self.rpc_url.clone()).unwrap();

        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(self.chain_id),
        ));
        let token_registry = TokenRegistry::new(self.token_registry, client);

        for token in tokens.iter() {
            let exists = token_registry
                .get_token_address_with_chain_id(chain_id_u256, token.name.clone())
                .await
                .unwrap();

            if exists != H160::zero() {
                println!(
                    "Token {} already exists in registry, address = {}",
                    &token.name, exists
                );
                return;
            }
        }

        println!("Adding {} to registry...", tokens.len());

        let call = token_registry.set_token_addresses(
            self.chain_id.to_string().parse().unwrap(),
            tokens.iter().map(|t| t.name.clone()).collect(),
            tokens.iter().map(|t| t.address.clone()).collect(),
        );

        let tx = call.send().await.unwrap();
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await
            .unwrap()
            .unwrap();
        let tx_hash = receipt.transaction_hash;

        println!("Done. Tx: {:#?}", tx_hash);
    }
}

pub async fn run() {
    dotenv().ok();

    TokenRegistryUtil {
        chain_id: std::env::var("CHAIN_ID").unwrap().parse().unwrap(),
        rpc_url: std::env::var("RPC_URL").unwrap(),
        token_registry: std::env::var("TOKEN_REGISTRY").unwrap().parse().unwrap(),
        private_key: std::env::var("PRIVATE_KEY").unwrap(),
    }
    .populate_token_registry()
    .await;
}
