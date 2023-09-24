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
    /// @param params params decoded from email subject based on the template
    /// @return emailSubject Expected email subject
    function computeEmailSubject(
        bytes memory params,
        uint8 templateIndex
    ) external pure virtual returns (string memory);

    /// Execute the extension logic
    /// @param params Extension params in the EmailOp (decoded from email subject)
    /// @param templateIndex Index of the subjectTemplate to which the subject was matched
    /// @param wallet Address of users wallet
    /// @param emailNullifier Nullifier of the email
    function execute(bytes memory params, uint8 templateIndex, address wallet, bytes32 emailNullifier) external virtual;

    /// Register unclaimed state for a recipient emailCommitment
    /// @param unclaimedState Unclaimed state that is registered
    function registerUnclaimedState(UnclaimedState memory unclaimedState) public virtual returns (bool) {
        unclaimedState;
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
