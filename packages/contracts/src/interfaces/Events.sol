// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

library EmailWalletEvents {
    event RelayerRegistered(address indexed addr, string emailAddr, string hostname);

    event RelayerConfigUpdated(address indexed addr, string hostname);

    event AccountCreated(
        bytes32 indexed walletSalt,
        bytes psiPoint
    );

    event UnclaimedFundRegistered(
        uint256 indexed id,
        bytes32 indexed emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedFundClaimed(
        uint256 indexed id,
        bytes32 indexed emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address recipient
    );

    event UnclaimedFundVoided(
        uint256 indexed id,
        bytes32 indexed emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address sender
    );

    event UnclaimedStateRegistered(
        uint256 indexed id,
        bytes32 indexed emailAddrCommit,
        address extensionAddr,
        address sender,
        uint256 expiryTime,
        bytes state,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedStateClaimed(uint256 indexed id, bytes32 indexed emailAddrCommit, address recipient);

    event UnclaimedStateVoided(uint256 indexed id, bytes32 indexed emailAddrCommit, address sender);

    event ExtensionPublished(
        string indexed name,
        address indexed extensionAddr,
        string[][] subjectTemplates,
        uint256 maxExecutionGas
    );

    event EmailOpHandled(
        bool indexed success,
        uint256 indexed registeredUnclaimId,
        bytes32 indexed emailNullifier,
        bytes32 walletSalt,
        bytes32 recipientEmailAddrCommit,
        address recipientETHAddr,
        bytes err
    );
}
