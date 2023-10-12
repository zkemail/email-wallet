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
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        uint256 /* timestamp */,
        bytes32 /* relayerHash */,
        bytes32 /* emailAddrPointer */,
        bytes32 /* accountKeyCommit */,
        bytes32 /* emailNullifier */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyEmailOpProof(
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        uint256 /* timestamp */,
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

    function verifyAccountTransportProof(
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        uint256 /* timestamp */,
        bytes32 /* emailNullifier */,
        bytes32 /* oldRelayerRandHash */,
        bytes32 /* newRelayerRandHash */,
        bytes32 /* oldAccountKeyCommit */,
        bytes32 /* newAccountKeyCommit */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }
}
