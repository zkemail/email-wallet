// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/interfaces/IVerifier.sol";

/**
    A mock verifier that returns true if proof[0] == 0x01, otherwise false
 */
contract TestVerifier is IVerifier {
    function verifyAccountCreationProof(
        bytes32 /* relayerHash */,
        bytes32 /* emailAddrPointer */,
        bytes32 /* accountKeyCommit */,
        bytes32 /* walletSalt */,
        bytes memory /* psiPoint */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyAccountInitializaionProof(
        bytes32 /* relayerHash */,
        bytes32 /* emailAddrPointer */,
        bytes32 /* accountKeyCommit */,
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        bytes32 /* emailNullifier */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyEmailProof(
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        string memory /* maskedSubject */,
        bytes32 /* emailNullifier */,
        bytes32 /* relayerHash */,
        bytes32 /* emailAddrPointer */,
        bool /* hasEmailRecipient */,
        bytes32 /* recipientEmailAddrCommit */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyClaimFundProof(
        bytes32 /* recipientRelayerHash */,
        bytes32 /* recipientEmailAddrPointer */,
        bytes32 /* recipientEmailAddrCommit */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifiyAccountTransportProof(
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        bytes32 /* emailNullifier */,
        bytes32 /* oldRelayerRandHash */,
        bytes32 /* oldAccountKeyCommit */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }
}
