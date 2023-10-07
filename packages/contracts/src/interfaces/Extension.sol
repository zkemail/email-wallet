import "./Types.sol";

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

abstract contract Extension {
    /// @notice Returns the command that this extension responds to. Eg: `Swap`
    /// @return command Command name
    function getCommand() external pure virtual returns (string memory);

    /// @notice Returns the email subject template with expected types of params
    /// @notice Eg: `Swap (amount) (string) to (string) and send to (recipient)`
    /// @return emailSubjectTemplate Email subject template
    function getSubjectTemplates() external pure virtual returns (string[] memory);

    /// @notice Returns the expected email subject from the given params
    /// @param templateIndex Index of the subjectTemplate to which the subject was matched
    /// @param tokenAmounts token/amount pairs extracted from the subject
    /// @param subjectParams params decoded from email subject based on the template
    /// @return emailSubject Expected email subject
    /// @dev To calculate, get the actual value of matcher in subject template from `params` arg in same order,
    ///      except for tokenAmounts, which should be taken from `tokenAmounts` arg in the same order.
    function computeEmailSubject(
        uint8 templateIndex,
        TokenAmount[] memory tokenAmounts,
        bytes memory subjectParams
    ) external pure virtual returns (string memory);

    /// Execute the extension logic
    /// @param templateIndex Index of the subjectTemplate to which the subject was matched
    /// @param tokenAmounts token/amount pairs extracted from the subject
    /// @param subjectParams params decoded from email subject based on the template
    /// @param wallet Address of users wallet
    /// @param hasEmailRecipient Whether the email subject has a recipient (email address)
    /// @param recipientETHAddr The ETH address of the recipient in email (if any, and hasEmailRecipient = false)
    /// @param emailNullifier Nullifier of the email
    /// @dev Implementations should not send tokens to `wallet` directly and use `EmailWalletCore.depositTokenToAccount()` instead
    function execute(
        uint8 templateIndex,
        TokenAmount[] memory tokenAmounts,
        bytes memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external virtual;

    /// Register unclaimed state for a recipient emailCommitment
    /// @param unclaimedState Unclaimed state that is registered
    /// @param isInternal A flag whether the unclaimed state is registered from `registerUnclaimedStateAsExtension`.
    function registerUnclaimedState(UnclaimedState memory unclaimedState, bool isInternal) public virtual returns (bool) {
        unclaimedState;
        isInternal;
        revert("Not implemented");
    }

    /// Claim an unclaimed state to recipient user
    /// @param unclaimedState Unclaimed state that is being claimed
    /// @param wallet Address of users wallet
    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external virtual;

    /// Revert an expired inclaimed state
    /// @param unclaimedState Unclaimed state that is expired
    function revertUnclaimedState(UnclaimedState memory unclaimedState) external virtual {
        unclaimedState;
        revert("Not implemented");
    }
}
