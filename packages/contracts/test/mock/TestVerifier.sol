// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/interfaces/IVerifier.sol";

/**
    A mock verifier that returns true if proof[0] == 0x01, otherwise false
 */
contract TestVerifier is IVerifier {
    function verifyAccountCreationProof(
        bytes32 /* relayerHash */,
        bytes32 /* emailAddressPointer */,
        bytes32 /* viewingKeyCommitment */,
        bytes32 /* walletSalt */,
        bytes memory proof
    ) external view returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyAccountInitializaionProof(
        bytes32 /* relayerHash */,
        bytes32 /* emailAddressPointer */,
        bytes32 /* viewingKeyCommitment */,
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        bytes memory proof
    ) external view returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyRecipientAccountProof(
        bytes32 /* relayerHash */,
        bytes32 /* emailAddressPointer */,
        bytes32 /* viewingKeyCommitment */,
        bytes32 /* walletSalt */,
        bytes32 /* emailAddressWitness */,
        bytes memory proof
    ) external view returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyEmailProof(
        bytes32 /* senderRelayerHash */,
        bytes32 /* senderEmailAddressPointer */,
        bytes32 /* senderViewingKeyCommitment */,
        bool /* hasRecipient */,
        bool /* isRecipientExternal */,
        bytes32 /* recipientEmailAddressWitness */,
        string memory /* maskedSubjectStr */,
        bytes32 /* emailNullifier */,
        string memory /* senderEmailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        bytes memory proof
    ) external view returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyClaimFundProof(
        bytes32 /* recipientRelayerHash */,
        bytes32 /* recipientEmailAddressPointer */,
        bytes32 /* recipientViewingKeyCommitment */,
        bytes32 /* recipientWalletSalt */,
        bytes32 /* recipientEmailAddressCommitment */,
        bytes memory proof
    ) external view returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }
}
