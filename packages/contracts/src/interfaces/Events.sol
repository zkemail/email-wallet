// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

interface EmailWalletEvents {
    event RelayerRegistered(address indexed addr, bytes32 randHash, string emailAddr, string hostname);

    event RelayerConfigUpdated(address indexed addr, string hostname);

    event AccountCreated(
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 indexed walletSalt,
        bytes psiPoint
    );

    event AccountInitialized(bytes32 emailAddrPointer, bytes32 accountKeyCommit, bytes32 indexed walletSalt);

    event AccountTransported(bytes32 oldAccountKeyCommit, bytes32 newEmailAddrPointer, bytes32 newAccountKeyCommit);

    event UnclaimedFundRegistered(
        bytes32 indexed emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedFundClaimed(bytes32 indexed emailAddrCommit, address tokenAddr, uint256 amount, address recipient);

    event UnclaimedFundVoided(bytes32 indexed emailAddrCommit, address tokenAddr, uint256 amount, address sender);

    event UnclaimedStateRegistered(
        bytes32 indexed emailAddrCommit,
        address extensionAddr,
        address sender,
        uint256 expiryTime,
        bytes state,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedStateClaimed(bytes32 indexed emailAddrCommit, address recipient);

    event UnclaimedStateVoided(bytes32 indexed emailAddrCommit, address sender);

    event ExtensionPublished(
        string indexed name,
        address indexed extensionAddr,
        string[][] subjectTemplates,
        uint256 maxExecutionGas
    );
}
