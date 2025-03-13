// Config strings
pub const SMTP_SERVER_KEY: &str = "SMTP_SERVER";
pub const ERROR_EMAIL_ADDRESSES_KEY: &str = "ERROR_EMAIL_ADDRESSES";
pub const DATABASE_PATH_KEY: &str = "DATABASE_URL";
pub const WEB_SERVER_ADDRESS_KEY: &str = "WEB_SERVER_ADDRESS";
pub const CIRCUITS_DIR_PATH_KEY: &str = "CIRCUITS_DIR_PATH";
pub const PROVER_ADDRESS_KEY: &str = "PROVER_ADDRESS";
pub const CHAIN_RPC_PROVIDER_KEY: &str = "CHAIN_RPC_PROVIDER";
pub const CHAIN_RPC_EXPLORER_KEY: &str = "CHAIN_RPC_EXPLORER";
pub const PRIVATE_KEY_KEY: &str = "PRIVATE_KEY";
pub const CHAIN_ID_KEY: &str = "CHAIN_ID";
pub const CORE_CONTRACT_ADDRESS_KEY: &str = "CORE_CONTRACT_ADDRESS";
pub const FEE_PER_GAS_KEY: &str = "FEE_PER_GAS";
pub const RELAYER_EMAIL_ADDR_KEY: &str = "RELAYER_EMAIL_ADDR";
pub const RELAYER_HOSTNAME_KEY: &str = "RELAYER_HOSTNAME";
pub const INPUT_FILES_DIR_KEY: &str = "INPUT_FILES_DIR_PATH";
pub const EMAIL_TEMPLATES_PATH_KEY: &str = "EMAIL_TEMPLATES_PATH";
pub const SUBGRAPH_URL_KEY: &str = "SUBGRAPH_URL";
pub const ONBOARDING_TOKEN_ADDR_KEY: &str = "ONBOARDING_TOKEN_ADDR";
pub const ONBOARDING_TOKEN_AMOUNT_KEY: &str = "ONBOARDING_TOKEN_AMOUNT";
pub const ONBOARDING_TOKEN_DISTRIBUTION_LIMIT_KEY: &str = "ONBOARDING_TOKEN_DISTRIBUTION_LIMIT";
pub const ONBOARDING_REPLY_KEY: &str = "ONBOARDING_REPLY";
pub const SAFE_API_ENDPOINT_KEY: &str = "SAFE_API_ENDPOINT";

// Error strings
pub const CANNOT_GET_EMAIL_FROM_QUEUE: &str = "Cannot get email from mpsc in handle email task";
pub const NOT_MY_SENDER: &str = "NOT_MY_SENDER";
pub const WRONG_SUBJECT_FORMAT: &str = "Wrong subject format";
pub const INSUFFICIENT_BALANCE: &str = "Insufficient balance";

// Core REGEX'es and Commands
pub const AMOUNT_REGEX: &str = "[0-9]+(\\.[0-9]+)?";
pub const TOKEN_NAME_REGEX: &str = "[A-Z]+";
pub const STRING_RGEX: &str = ".+";
pub const UINT_REGEX: &str = "[0-9]+";
pub const INT_REGEX: &str = "-?[0-9]+";
pub const ETH_ADDR_REGEX: &str = "0x[0-9a-fA-F]{40}";
pub const EMAIL_ADDR_REGEX: &str =
    "[a-zA-Z0-9!#$%&'\\*\\+-/=\\?^_`{\\|}~\\.]+@[a-zA-Z0-9]+\\.[a-zA-Z0-9\\.-]+";
pub const TX_HASH_REGEX: &str = "0x[0-9a-fA-F]{64}";
pub const SEND_COMMAND: &str = "Send";
pub const EXECUTE_COMMAND: &str = "Execute";
pub const INSTALL_COMMAND: &str = "Install";
pub const UNINSTALL_COMMAND: &str = "Uninstall";
pub const EXIT_COMMAND: &str = "Exit";
pub const DKIM_COMMAND: &str = "DKIM";
pub const SAFE_COMMAND: &str = "Safe";

// DKIM ORACLE ARGS
pub const CANISTER_ID_KEY: &str = "CANISTER_ID";
pub const PEM_PATH_KEY: &str = "PEM_PATH";
pub const IC_REPLICA_URL_KEY: &str = "IC_REPLICA_URL";
pub const WALLET_CANISTER_ID_KEY: &str = "WALLET_CANISTER_ID";
