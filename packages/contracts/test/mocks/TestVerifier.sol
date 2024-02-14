// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/interfaces/IVerifier.sol";

/**
    A mock verifier that returns true if proof[0] == 0x01, otherwise false
 */
contract TestVerifier is IVerifier {
    function verifyAccountCreationProof(
        string memory /* emailDomain */,
        bytes32 /* dkimPublicKeyHash */,
        bytes32 /* emailNullifier */,
        uint256 /* emailTimestamp */,
        bytes32 /* walletSalt */,
        bytes memory /* psiPoint */,
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
        bytes32 /* emailNullifier */,
        string memory /* maskedSubject */,
        bytes32 /* walletSalt */,
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
        bytes32 /* recipientEmailAddrCommit */,
        bytes32 /* recipientWalletSalt */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyAnnouncementProof(
        string memory /* emailAddr */,
        bytes32 /* rand */,
        bytes32 /* emailAddrCommit */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }
}
