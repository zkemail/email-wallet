// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/interfaces/IVerifier.sol";

/**
    A mock verifier that returns true if proof[0] == 0x01, otherwise false
 */
contract TestVerifier is IVerifier {
    function verifyEmailProof(EmailProof calldata emailProof) external pure returns (bool) {
        if (emailProof.proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyClaimFundProof(
        bytes32 /* recipientEmailAddrCommit */,
        bytes32 /* recipientAccountSalt */,
        bytes memory proof
    ) external pure returns (bool) {
        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }

    function verifyPsiPointProof(
        bytes32 /*accountSalt*/,
        bytes memory /*psiPoint*/,
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
