// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

struct RelayerConfig {
    string emailAddr; // relayer's email address
    string hostname; // hostname of relayer's server - used by other relayers for PSI communication
}

// Struct to represent an operation from the user
struct EmailOp {
    bytes32 walletSalt; // emailAddrPointer of sender's account
    string command; // Command name (like "wallet", "swap")
    bytes32 emailNullifier; // Nullifier of email to prevent re-run
    string emailDomain; // Domain name of the sender's email
    bytes32 dkimPublicKeyHash; // Hash of the DKIM public key used in email/proof
    string maskedSubject; // Subject string with email address masked
    uint256 skipSubjectPrefix; // Number of bytes to skip in the subject
    uint256 timestamp; // Timestamp of the email
    bool hasEmailRecipient; // a flag whether the recipient's email address is included in the subject
    bytes32 recipientEmailAddrCommit; // Commitment to recipient's email address if `hasEmailRecipient` is true
    uint256 numRecipientEmailAddrBytes; // Number of bytes of recipient's email address if `hasEmailRecipient` is true
    address recipientETHAddr; // ETH address of recipient - only used if `hasEmailRecipient` is false
    string feeTokenName; // Name of the token to pay the fee
    uint256 feePerGas; // Amount of wei to be charged per gas
    bytes executeCallData; // Encoded (target+calldata) hex if the the command is "Execute"
    string extensionName; // Name of the extension if the command is "Install Extension" / "Uninstall Extension"
    address newWalletOwner; // Address of the new owner if the command is "Exit Email Wallet"
    address newDkimRegistry; // Address of the new dkim registry if the command is "DKIM"
    WalletParams walletParams; // Params when command = "Send"
    ExtensionParams extensionParams; // Serialized params for the extension based on the template
    bytes emailProof; // ZK Proof of Email receipt
}

// When command = "Send"
struct WalletParams {
    string tokenName; // Name of the token to transfer (from subject) - could be "ETH"
    uint256 amount; // Amount to transfer/swap (in wei) - extracted from subject
}

struct ExtensionParams {
    uint8 subjectTemplateIndex; // Index of the extension subject template
    bytes[] subjectParams; // Match params extracted from the subject in the same order; {tokenAmount} should be encoded as (uint256,string)
}

// Struct to represent a fund transfer that is not claimed by the recipient (relayer)
struct UnclaimedFund {
    uint256 id;
    bytes32 emailAddrCommit;
    address sender;
    address tokenAddr;
    uint256 amount;
    uint256 expiryTime;
}

struct UnclaimedState {
    uint256 id;
    bytes32 emailAddrCommit;
    address extensionAddr;
    address sender;
    bytes state;
    uint256 expiryTime;
}

// A struct to represent commong args in a proof of email
// Useful for methods thats need fewer inputs to avoid stack too deep error
struct EmailProof {
    string domain; // Domain name of the sender's email
    uint256 timestamp; // Timestamp of the email
    bytes32 nullifier; // Nullifier of email to prevent re-run
    bytes32 dkimPublicKeyHash; // Hash of the DKIM public key used in email/proof
    bytes proof; // ZK Proof of Email
}

// Struct to store token allowance for an extension in context
struct TokenAllowance {
    address tokenAddr;
    uint256 amount;
}

// Struct to store context when executing an EmailOp
struct ExecutionContext {
    address walletAddr; // Wallet address of the user
    address extensionAddr; // Address of extension in use
    bool unclaimedFundRegistered; // Flag to indicate whether the unclaimed state has been registered
    bool unclaimedStateRegistered; // Flag to indicate whether the unclaimed state has been registered
    uint256 registeredUnclaimId; // Id of the registered unclaimed fund/state
    bytes32 recipientEmailAddrCommit; // Commitment to recipient's email address when there is email recipient
    TokenAllowance[] tokenAllowances; // token/amount allowed to be consumed by the extension
}
