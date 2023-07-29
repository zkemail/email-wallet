// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

interface IVerifier {
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) external view returns (bool);

    function verifyEmailProof(
        bytes32 senderRelayerHash,
        bytes32 senderPointer,
        bytes32 senderIndicator,
        bytes32 recipientRelayerHash,
        bytes32 recipientPointer,
        bytes32 recipientIndicator,
        bytes32 emailNullifier,
        bytes32 dkimPublicKeyHash,
        string memory domainName,
        string memory maskedSubjectStr,
        bool hasRecipient,
        bool isRecipientExternal,
        bytes memory proof
    ) external view returns (bool);
}
