use crate::*;

use std::{env, path::PathBuf};

use dotenv::dotenv;

#[derive(Clone)]
pub struct RelayerConfig {
    pub smtp_server: String,
    pub error_email_addresses: Vec<String>,
    pub relayer_email_addr: String,
    pub db_path: String,
    pub web_server_address: String,
    pub circuits_dir_path: PathBuf,
    pub prover_address: String,
    pub chain_rpc_provider: String,
    pub chain_rpc_explorer: String,
    pub chain_id: u32,
    pub private_key: String,
    pub core_contract_address: String,
    pub fee_per_gas: U256,
    pub input_files_dir: String,
    pub email_templates: String,
    pub subgraph_url: String,
    pub onboarding_token_addr: H160,
    pub onboarding_token_amount: U256,
    pub onboarding_token_distribution_limit: u32,
    pub onboarding_reply_msg: String,
    pub safe_api_endpoint: String,
}

impl RelayerConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let fee_per_gas = env::var(FEE_PER_GAS_KEY)
            .unwrap_or_else(|_| panic!("Failed to read environment variable: {}", FEE_PER_GAS_KEY));
        let fee_per_gas = U256::from_dec_str(&fee_per_gas)
            .unwrap_or_else(|_| panic!("Failed to parse fee_per_gas: {}", fee_per_gas));

        let input_files_dir = env::var(INPUT_FILES_DIR_KEY).unwrap_or_else(|_| {
            panic!(
                "Failed to read environment variable: {}",
                INPUT_FILES_DIR_KEY
            )
        });

        let subgraph_url = env::var(SUBGRAPH_URL_KEY).unwrap_or_else(|_| {
            panic!("Failed to read environment variable: {}", SUBGRAPH_URL_KEY)
        });

        let onboarding_token_addr: H160 = env::var(ONBOARDING_TOKEN_ADDR_KEY)
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    ONBOARDING_TOKEN_ADDR_KEY
                )
            })
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse onboarding_token_addr"));

        let onboarding_token_distribution_limit: u32 =
            env::var(ONBOARDING_TOKEN_DISTRIBUTION_LIMIT_KEY)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to read environment variable: {}",
                        ONBOARDING_TOKEN_DISTRIBUTION_LIMIT_KEY
                    )
                })
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse onboarding_token_distribution_limit"));

        let onboarding_token_amount = env::var(ONBOARDING_TOKEN_AMOUNT_KEY).unwrap_or_else(|_| {
            panic!(
                "Failed to read environment variable: {}",
                ONBOARDING_TOKEN_AMOUNT_KEY
            )
        });
        let onboarding_token_amount =
            U256::from_dec_str(&onboarding_token_amount).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse onboarding_token_amount: {}",
                    onboarding_token_amount
                )
            });

        let onboarding_reply_msg = env::var(ONBOARDING_REPLY_KEY).unwrap_or_else(|_| {
            panic!(
                "Failed to read environment variable: {}",
                ONBOARDING_REPLY_KEY
            )
        });

        Self {
            smtp_server: env::var(SMTP_SERVER_KEY).unwrap_or_else(|_| {
                panic!("Failed to read environment variable: {}", SMTP_SERVER_KEY)
            }),
            error_email_addresses: env::var(ERROR_EMAIL_ADDRESSES_KEY)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to read environment variable: {}",
                        ERROR_EMAIL_ADDRESSES_KEY
                    )
                })
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            relayer_email_addr: env::var(RELAYER_EMAIL_ADDR_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    RELAYER_EMAIL_ADDR_KEY
                )
            }),
            db_path: env::var(DATABASE_PATH_KEY).unwrap_or_else(|_| {
                panic!("Failed to read environment variable: {}", DATABASE_PATH_KEY)
            }),
            web_server_address: env::var(WEB_SERVER_ADDRESS_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    WEB_SERVER_ADDRESS_KEY
                )
            }),
            circuits_dir_path: env::var(CIRCUITS_DIR_PATH_KEY)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to read environment variable: {}",
                        CIRCUITS_DIR_PATH_KEY
                    )
                })
                .into(),
            prover_address: env::var(PROVER_ADDRESS_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    PROVER_ADDRESS_KEY
                )
            }),
            chain_rpc_provider: env::var(CHAIN_RPC_PROVIDER_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    CHAIN_RPC_PROVIDER_KEY
                )
            }),
            chain_rpc_explorer: env::var(CHAIN_RPC_EXPLORER_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    CHAIN_RPC_EXPLORER_KEY
                )
            }),
            chain_id: env::var(CHAIN_ID_KEY)
                .unwrap_or_else(|_| panic!("Failed to read environment variable: {}", CHAIN_ID_KEY))
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse chain_id")),
            private_key: env::var(PRIVATE_KEY_KEY).unwrap_or_else(|_| {
                panic!("Failed to read environment variable: {}", PRIVATE_KEY_KEY)
            }),
            core_contract_address: env::var(CORE_CONTRACT_ADDRESS_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    CORE_CONTRACT_ADDRESS_KEY
                )
            }),
            fee_per_gas,
            input_files_dir,
            email_templates: env::var(EMAIL_TEMPLATES_PATH_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    EMAIL_TEMPLATES_PATH_KEY
                )
            }),
            subgraph_url,
            onboarding_token_addr,
            onboarding_token_distribution_limit,
            onboarding_token_amount,
            onboarding_reply_msg,
            safe_api_endpoint: env::var(SAFE_API_ENDPOINT_KEY).unwrap_or_else(|_| {
                panic!(
                    "Failed to read environment variable: {}",
                    SAFE_API_ENDPOINT_KEY
                )
            }),
        }
    }
}

impl Default for RelayerConfig {
    fn default() -> Self {
        Self::new()
    }
}
