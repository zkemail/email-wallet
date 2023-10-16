import "./Extension.sol";
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

abstract contract ExtensionQueryable is Extension {
    /// Subject template for the query
    /// @dev The first word in each template must be a command name.
    /// @dev An actual email subject to query on-chain data has a prefix, e.g., "Query:".
    string[][] public querySubjectTemplates;

    /// Query the on-chain information based on the email subject
    /// @param templateIndex Index of the subjectTemplate to which the subject was matched
    /// @param subjectParams Array of params decoded from email subject based on the template, in the same order matchers
    /// @param wallet Address of users wallet
    /// @param hasEmailRecipient Whether the email subject has a recipient (email address)
    /// @param recipientETHAddr The ETH address of the recipient in email (if any, and hasEmailRecipient = false)
    /// @param emailNullifier Nullifier of the email
    /// @dev This function is intended to be called from a relayer off-chain
    /// @dev Decode {tokenAmount} in template as `abi.decode(uint256,string)` (`tokenName` and `tokenAmount`)
    function query(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external view virtual;

}
