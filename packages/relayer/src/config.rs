use crate::*;

use std::{env, path::PathBuf};

use dotenv::dotenv;

#[derive(Clone)]
pub struct RelayerConfig {
    pub(crate) imap_config: ImapConfig,
    pub(crate) smtp_config: SmtpConfig,
    pub(crate) db_path: String,
    // pub(crate) relayer_randomness: String,
    pub(crate) web_server_address: String,
    pub(crate) circuits_dir_path: PathBuf,
    pub(crate) prover_address: String,
    pub(crate) chain_rpc_provider: String,
    pub(crate) chain_id: u32,
    pub(crate) private_key: String,
    pub(crate) core_contract_address: String,
    pub(crate) fee_per_gas: U256,
    pub(crate) input_files_dir: String,
}

impl RelayerConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let imap_auth = env::var(IMAP_AUTH_TYPE_KEY).unwrap();
        let imap_auth = match &*imap_auth {
            "password" => ImapAuth::Password {
                user_id: env::var(LOGIN_ID_KEY).unwrap(),
                password: env::var(LOGIN_PASSWORD_KEY).unwrap(),
            },
            "oauth" => ImapAuth::Oauth {
                user_id: env::var(LOGIN_ID_KEY).unwrap(),
                client_id: env::var(IMAP_CLIENT_ID_KEY).unwrap(),
                client_secret: env::var(IMAP_CLIENT_SECRET_KEY).unwrap(),
                auth_url: env::var(IMAP_AUTH_URL_KEY).unwrap(),
                token_url: env::var(IMAP_TOKEN_URL_KEY).unwrap(),
                redirect_url: IMAP_REDIRECT_URL_KEY.to_string(),
            },
            _ => panic!("{WRONG_AUTH_METHOD}"),
        };

        let imap_config = ImapConfig {
            domain_name: env::var(IMAP_DOMAIN_NAME_KEY).unwrap(),
            port: env::var(IMAP_PORT_KEY).unwrap().parse().unwrap(),
            auth: imap_auth,
        };

        let smtp_config = SmtpConfig {
            domain_name: env::var(SMTP_DOMAIN_NAME_KEY).unwrap(),
            id: env::var(LOGIN_ID_KEY).unwrap(),
            password: env::var(LOGIN_PASSWORD_KEY).unwrap(),
        };

        let fee_per_gas = env::var(FEE_PER_GAS_KEY).unwrap();
        let fee_per_gas = U256::from_dec_str(&fee_per_gas).unwrap();

        let input_files_dir = env::var(INPUT_FILES_DIR_KEY).unwrap();

        Self {
            imap_config,
            smtp_config,
            db_path: env::var(DATABASE_PATH_KEY).unwrap(),
            // relayer_randomness: env::var(RELAYER_RANDOMNESS_KEY).unwrap(),
            web_server_address: env::var(WEB_SERVER_ADDRESS_KEY).unwrap(),
            circuits_dir_path: env::var(CIRCUITS_DIR_PATH_KEY).unwrap().into(),
            prover_address: env::var(PROVER_ADDRESS_KEY).unwrap(),
            chain_rpc_provider: env::var(CHAIN_RPC_PROVIDER_KEY).unwrap(),
            chain_id: env::var(CHAIN_ID_KEY).unwrap().parse().unwrap(),
            private_key: env::var(PRIVATE_KEY_KEY).unwrap(),
            core_contract_address: env::var(CORE_CONTRACT_ADDRESS_KEY).unwrap(),
            fee_per_gas,
            input_files_dir,
        }
    }
}

impl Default for RelayerConfig {
    fn default() -> Self {
        Self::new()
    }
}
