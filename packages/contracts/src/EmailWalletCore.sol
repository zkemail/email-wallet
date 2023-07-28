// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "./interfaces/IVerifier.sol";

contract EmailWalletCore {
    IVerifier public globalVerifier;

    // Mapping of relayer's wallet address to hash(relayerRand)
    mapping(address => bytes32) public relayers;

    // Mapping from pointer (emailAddress commitment) to indicator (viewingKey commitment)
    mapping(bytes32 => bytes32) public indicatorOfPointer;

    // Flag to indicate whether a ViewingKey is initialized
    mapping(bytes32 => bool) public isInitialized;

    // Mapping from ViewingKey to RelayerHash - to track the current relayer of a user
    mapping(bytes32 => bytes32) public relayerOfViewingKey;

    constructor(address _globalVerifier) {
        globalVerifier = IVerifier(_globalVerifier);
    }

    function registerRelayer(bytes32 _relayerHash) public {
        require(
            relayers[msg.sender] == bytes32(0),
            "relayer already registered"
        );

        require(_relayerHash != bytes32(0), "relayer hash must not be zero");

        relayers[msg.sender] = _relayerHash;
    }

    function createAccount(
        bytes32 viewingKey,
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) public {
        bytes32 relayerHash = relayers[msg.sender];
        require(relayerHash != bytes32(0), "relayer not registered");

        require(pointer != bytes32(0), "pointer must not be zero");
        require(indicator != bytes32(0), "indicator must not be zero");
        require(proof.length > 0, "proof must not be empty");

        require(
            indicatorOfPointer[pointer] == bytes32(0),
            "pointer already exists"
        );

        // Verify proof
        require(
            globalVerifier.verifyAccountCreationProof(
                relayerHash,
                viewingKey,
                pointer,
                indicator,
                proof
            ),
            "invalid account creation proof"
        );

        indicatorOfPointer[pointer] = indicator;
        relayerOfViewingKey[viewingKey] = relayers[msg.sender];
    }
}
