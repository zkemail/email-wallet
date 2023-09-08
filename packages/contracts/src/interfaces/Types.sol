// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

// Struct to represent an operation from the user
struct EmailOperation {
    bytes32 senderEmailAddressPointer; // emailAddressPointer of sender's account
    bytes senderAccountProof; // Proof that senders pointer + vkCommitment + wallet salt is created by relayer
    //
    bool hasRecipient; // Whether the operation has a recipient (like in a transfer)
    bool isRecipientExternal; // Whether the recipient is non-email wallet account
    address recipientExternalAddress;
    address recipientRelayer;
    bytes32 recipientEmailAddressPointer; // emailAddressPointer of recipient's account
    bytes32 recipientEmailAddressWitness; // A commitment to recipient's email address to cross check with other proofs
    bytes recipientAccountProof; // Proof of recipient's account created by recipient relayer
    bytes32 recipientEmailAddressCommitment; // Commitment to recipient's email address to register unclaimed funds
    //
    address extensionAddress; // Address of the extension to set for the command (SET_EXTENSION)
    bytes extensionParams; // Params for the extension (like swap params)
    //
    string command; // Command name (like "wallet", "swap")
    bytes32 emailNullifier; // Nullifier of email to prevent re-run
    string emailDomain; // Domain name of the sender's email
    string maskedSubjectStr; // Subject string with recipient email masked
    uint256 amount; // Amount to transfer/swap (in wei) - extracted from subject
    string tokenName; // Name of the token to transfer (from subject) - could be "ETH"
    bytes emailProof; // ZK Proof of Email receipt
}

struct UnclaimedFund {
    string tokenName;
    uint256 amount;
    uint256 expiryTime;
    address sender;
}
