// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

interface IVerifier {
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 viewingKey,
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) external view returns (bool);
}
