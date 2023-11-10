import "./Types.sol";

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

abstract contract Extension {
    /// Execute the extension logic
    /// @param templateIndex Index of the subjectTemplate to which the subject was matched
    /// @param subjectParams Array of params decoded from email subject based on the template, in the same order matchers
    /// @param wallet Address of users wallet
    /// @param hasEmailRecipient Whether the email subject has a recipient (email address)
    /// @param recipientETHAddr The ETH address of the recipient in email (if any, and hasEmailRecipient = false)
    /// @param emailNullifier Nullifier of the email
    /// @dev Implementations should not send tokens to `wallet` directly and use `EmailWalletCore.depositTokenToAccount()` instead
    /// @dev Decode {tokenAmount} in template as `abi.decode(uint256,string)` (`tokenName` and `tokenAmount`)
    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external virtual;

    /// Register unclaimed state for a recipient emailCommitment
    /// @param unclaimedState Unclaimed state that is registered
    /// @param isInternal A flag whether the unclaimed state is registered from `registerUnclaimedStateAsExtension` and the caller and callee extensions are the same.
    function registerUnclaimedState(UnclaimedState memory unclaimedState, bool isInternal) public virtual {
        unclaimedState;
        isInternal;
    }

    /// Claim an unclaimed state to recipient user
    /// @param unclaimedState Unclaimed state that is being claimed
    /// @param wallet Address of users wallet
    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external virtual {
        unclaimedState;
        wallet;
        revert("Not implemented");
    }

    /// Revert an expired inclaimed state
    /// @param unclaimedState Unclaimed state that is expired
    function voidUnclaimedState(UnclaimedState memory unclaimedState) external virtual {
        unclaimedState;
        revert("Not implemented");
    }
}
