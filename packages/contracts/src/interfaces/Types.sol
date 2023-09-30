// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

struct RelayerConfig {
    bytes32 randHash; // Hash of the relayer's randomnes - the one used to create pointers and account key commitments
    string emailAddr; // relayer's email address
    string hostname; // hostname of relayer's server - used by other relayers for PSI communication
}

// Struct to represent an operation from the user
struct EmailOp {
    bytes32 emailAddrPointer; // emailAddrPointer of sender's account
    bool hasEmailRecipient; // a flag whether the recipient's email address is included in the subject
    bytes32 recipientEmailAddrCommit; // Commitment to recipient's email address if `hasEmailRecipient` is true
    address recipientETHAddr; // ETH address of recipient - only used if `hasEmailRecipient` is false
    string command; // Command name (like "wallet", "swap")
    bytes32 emailNullifier; // Nullifier of email to prevent re-run
    string emailDomain; // Domain name of the sender's email
    uint256 timestamp; // Timestamp of the email
    string maskedSubject; // Subject string with email address masked
    string feeTokenName; // Name of the token to pay the fee
    uint256 feePerGas; // Amount of ETH to be charged per gas
    uint8 extensionSubjectTemplateIndex; // Index of the extension subject template
    WalletParams walletParams; // Params when command = "Transfer" / "Send"
    ExtensionManagerParams extManagerParams; // Params when command = "Install Extension" / "Uninstall Extension"
    bytes extensionParams; // Serialized params for the extension based on the template
    bytes emailProof; // ZK Proof of Email receipt
}

// When command = "Transfer" / "Send"
struct WalletParams {
    string tokenName; // Name of the token to transfer (from subject) - could be "ETH"
    uint256 amount; // Amount to transfer/swap (in wei) - extracted from subject
}

// When command = "Install Extension" / "Uninstall Extension"
struct ExtensionManagerParams {
    string command; // Command name to set the extension for (like "swap")
    string extensionName; // Name of the extension to install/uninstall (like "uniswap")
}

// Struct to represent a fund transfer that is not claimed by the recipient (relayer)
struct UnclaimedFund {
    bytes32 emailAddrCommit;
    address senderAddress;
    address tokenAddress;
    uint256 amount;
    uint256 expiryTime;
}

struct UnclaimedState {
    bytes32 emailAddrCommit;
    address extensionAddress;
    address senderAddress;
    bytes state;
}

struct AccountKeyInfo {
    address relayer;
    bool initialized;
    bool nullified;
    // Flag that tracks whether wallet salt is non-zero (invariant: walletSaltSet == (walletSalt
    // != bytes32(0)), can help save a fresh SLOAD in certain methods.
    bool walletSaltSet;
    bytes32 walletSalt;
}

// Struct to store context when executing an EmailOp
struct ExecutionContext {
    address walletAddress; // Wallet address of the user
    address extensionAddress; // Address of extension in use
    bool unclaimedFundRegistered; // Flag to indicate whether the unclaimed state has been registered
    bool unclaimedStateRegistered; // Flag to indicate whether the unclaimed state has been registered
    uint256 receivedETH; // Amount of ETH sent by the relayer in the transaction
}
