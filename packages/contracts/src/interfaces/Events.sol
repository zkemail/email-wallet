// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

interface EmailWalletEvents {
    event RelayerRegistered(bytes32 randHash, string emailAddr, string hostname);

    event RelayerConfigUpdated(bytes32 randHash, string hostname);

    event AccountCreated(bytes32 emailAddrPointer, bytes32 accountKeyCommit, bytes32 walletSalt, bytes psiPoint);

    event AccountInitialized(bytes32 emailAddrPointer, bytes32 accountKeyCommit, bytes32 walletSalt);

    event AccountTransported(bytes32 oldAccountKeyCommit, bytes32 newEmailAddrPointer, bytes32 newAccountKeyCommit);

    event UnclaimedFundRegistered(
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedFundClaimed(bytes32 emailAddrCommit, address tokenAddr, uint256 amount, address recipient);

    event UnclaimedFundVoided(bytes32 emailAddrCommit, address tokenAddr, uint256 amount, address sender);

    event UnclaimedStateRegistered(
        bytes32 emailAddrCommit,
        address extensionAddr,
        address sender,
        uint256 expiryTime,
        bytes state,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedStateClaimed(bytes32 emailAddrCommit, address recipient);

    event UnclaimedStateVoided(bytes32 emailAddrCommit, address sender);

    event ExtensionPublished(string name, address extensionAddr, string[][] subjectTemplates, uint256 maxExecutionGas);
}