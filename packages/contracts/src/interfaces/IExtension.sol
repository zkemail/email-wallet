// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

interface IExtension {
    /// @notice Returns the command that this extension responds to. Eg: `Swap`
    /// @return command Command name
    function getCommand() external pure returns (string memory);

    /// @notice Returns the email subject template with expected types of params
    /// @notice Eg: `Swap (amount) (string) to (string) and send to (recipient)`
    /// @return emailSubjectTemplate Email subject template
    function getSubjectTemplate() external pure returns (string memory);

    /// @notice Returns the expected email subject from the given params
    /// @param params params decoded from email subject based on the template
    /// @return emailSubject Expected email subject
    function computeEmailSubject(bytes memory params) external pure returns (string memory);

    /// @notice Compute and return the calldata to be executed by the wallet contract.
    /// @param params params decoded from the email subject
    /// @return target address of the contract to be called
    /// @return data calldata to be called on the target
    function getExecutionCalldata(bytes memory params) external returns (address, bytes memory);
}
