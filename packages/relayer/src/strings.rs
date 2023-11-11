// Config strings
pub const ZK_EMAIL_PATH_KEY: &str = "LOCAL_ZK_EMAIL_CIRCOM_PATH";
pub const IMAP_DOMAIN_NAME_KEY: &str = "IMAP_DOMAIN_NAME";
pub const IMAP_PORT_KEY: &str = "IMAP_PORT";
pub const IMAP_AUTH_TYPE_KEY: &str = "AUTH_TYPE";
pub const IMAP_CLIENT_ID_KEY: &str = "IMAP_CLIENT_ID";
pub const IMAP_CLIENT_SECRET_KEY: &str = "IMAP_CLIENT_SECRET";
pub const IMAP_AUTH_URL_KEY: &str = "IMAP_AUTH_URL";
pub const IMAP_TOKEN_URL_KEY: &str = "IMAP_TOKEN_URL";
pub const IMAP_REDIRECT_URL_KEY: &str = "http://127.0.0.1:8000/";
pub const SMTP_DOMAIN_NAME_KEY: &str = "SMTP_DOMAIN_NAME";
pub const SMTP_PORT_KEY: &str = "SMTP_PORT";
pub const LOGIN_ID_KEY: &str = "LOGIN_ID";
pub const LOGIN_PASSWORD_KEY: &str = "LOGIN_PASSWORD";
pub const DATABASE_PATH_KEY: &str = "DATABASE_URL";
// pub const RELAYER_RANDOMNESS_KEY: &str = "RELAYER_RANDOMNESS";
pub const WEB_SERVER_ADDRESS_KEY: &str = "WEB_SERVER_ADDRESS";
pub const CIRCUITS_DIR_PATH_KEY: &str = "CIRCUITS_DIR_PATH";
pub const PROVER_ADDRESS_KEY: &str = "PROVER_ADDRESS";
pub const CHAIN_RPC_PROVIDER_KEY: &str = "CHAIN_RPC_PROVIDER";
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

// Error strings
pub const WRONG_AUTH_METHOD: &str = "Not supported auth type";
pub const IMAP_RECONNECT_ERROR: &str = "Failed to reconnect";
pub const SMTP_RECONNECT_ERROR: &str = "Failed to reconnect";
pub const CANNOT_GET_EMAIL_FROM_QUEUE: &str = "Cannot get email from mpsc in handle email task";
pub const NOT_MY_SENDER: &str = "NOT_MY_SENDER";
pub const WRONG_SUBJECT_FORMAT: &str = "Wrong subject format";
pub const INSUFFICIENT_BALANCE: &str = "Insufficient balance";

// Core REGEX'es and Commands
pub(crate) const AMOUNT_REGEX: &str = "[0-9]+(\\.[0-9]+)?";
pub(crate) const TOKEN_NAME_REGEX: &str = "[A-Z]+";
pub(crate) const STRING_RGEX: &str = ".+";
pub(crate) const UINT_REGEX: &str = "[0-9]+";
pub(crate) const INT_REGEX: &str = "-?[0-9]+";
pub(crate) const ETH_ADDR_REGEX: &str = "0x[0-9a-fA-F]{40}";
pub(crate) const EMAIL_ADDR_REGEX: &str =
    "[a-zA-Z0-9!#$%&'\\*\\+-/=\\?^_`{\\|}~\\.]+@[a-zA-Z0-9]+\\.[a-zA-Z0-9\\.-]+";
pub(crate) const SEND_COMMAND: &str = "Send";
pub(crate) const EXECUTE_COMMAND: &str = "Execute";
pub(crate) const INSTALL_COMMAND: &str = "Install";
pub(crate) const UNINSTALL_COMMAND: &str = "Uninstall";
pub(crate) const EXIT_COMMAND: &str = "Exit";
pub(crate) const DKIM_COMMAND: &str = "DKIM";
