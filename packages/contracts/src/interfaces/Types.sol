// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

// Struct to represent an operation from the user
struct EmailOperation {
  bytes32 senderPointer;        // Pointer = hash(relayerRand, emailAddr)
  bytes32 senderIndicator;      // Indicator = hash("INDICATOR", viewingKey, emailAddr, hash(relayerRand))
  WalletSaltProof senderWalletSaltProof;  // proof the salt is derived from the same VK as in indicator

  bool hasRecipient;            // Whether the operation has a recipient (like in a transfer)
  bool isRecipientExternal;     // Whether the recipient is non-email wallet account
  bool isRecipientInitialized;

  address recipientRelayer;
  bytes32 recipientPointer;
  bytes32 recipientIndicator;
  WalletSaltProof recipientWalletSaltProof;
  address recipientExternalAddress;

  string command;               // Command name (like "wallet", "swap")
   
  bytes32 emailNullifier;       // Nullifier of email to prevent re-run
  bytes32 dkimPublicKeyHash;    // Hash of DKIM public key of sender's domain - Hash used as original key is large
  string domainName;            // Domain name of the sender's email
  string maskedSubjectStr;      // Subject string with recipient email masked
  bytes emailProof;             // ZK Proof of Email

  uint256 amount;               // Amount to transfer/swap (in wei) - extracted from subject
  string tokenName;             // Name of the token to transfer (from subject) - could be "ETH"
}

// Struct to represent a proof of salt for a wallet
struct WalletSaltProof {
  uint256 randomNonce;
  bytes32 walletSalt;
  bytes proof;
}
