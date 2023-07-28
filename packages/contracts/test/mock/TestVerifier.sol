// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/interfaces/IVerifier.sol";

/**
    A mock verifier that returns true if proof[0] == 0x01, otherwise false
 */
contract TestVerifier is IVerifier {
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 viewingKey,
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) external pure returns (bool) {
        relayerHash;
        viewingKey;
        pointer;
        indicator;

        if (proof[0] == 0x01) {
            return true;
        }

        return false;
    }
}
