// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "../verifier/IVerifier.sol";
import "../extension/IExtension.sol";
import "@openzeppelin/contracts/proxy/utils/Initializable.sol";

/// This contract should be implemented as an upgradable contract.
/// Its proxy should follow UUPS Proxy pattern to define its upgrading logic in the logic contract.
interface IAccount {
    /// Return a core contract address.
    /// It must not return zero address.
    function getCoreAddr() external view returns (address);

    /// Return a current relayer address.
    /// It must not return zero address.
    function getRelayerAddr() external view returns (address);

    /// Return a logic contract address used by this contract.
    /// It must not return zero address.
    function getAccountLogicAddr() external view returns (address);

    /// Return an address of the `IVerifier` contract registered in this account contract.
    /// It must not return zero address because any account contract must register a verifier contract.
    function getVerifierAddr() external view returns (address);

    /// Given an extension id, return an address of the corresponding `IExtension` contract registered in this account contract.
    /// It must return zero address if `extensionId` is not registered.
    function getExtensionAddr(uint extensionId) external view returns (address);

    /// Given an email nullifier, returns true if it is already used and stored in this account and false otherwise.
    function isUsedEmailNullifier(
        bytes32 emailNullifier
    ) external view returns (bool);

    /// A bundler in EIP4337 calls this function to verify `UserOp`.
    function validateUserOp(
        bytes memory verifierParams,
        bytes memory proof,
        uint256 extensionId,
        bytes memory extensionParams
    ) external view;

    /// This function verifies and processes the given proof of the recipient circuit.
    /// It is called in the `executeUserOp` function because 1) it needs to access to the storage of the `subjectAccount` and 2) `validateUserOp` is enought to prevent a malicious relayer from stealing fees by forging transactions.
    function processRecipientProof(
        IVerifier.RecipientPublicInput memory recipientPublicInput,
        bytes memory recipientProof,
        bytes32 subjectEmailNullifier
    ) external;

    /// A bundler in EIP4337 calls this function to execute `UserOp`. It internally calls `validateUserOpAddition` function.
    function executeUserOp(
        bytes memory verifierParams,
        bytes memory proof,
        uint256 extensionId,
        bytes memory extensionParams
    ) external;

    /// Only extension contracts registerd in this account contract can call this function to forward the user's call to another extension contract specified.
    function forwardCall(
        IExtension.CallContext memory callCtx,
        bytes memory extensionParams
    ) external;

    /// Only the setting extension contract can call this function to upgrade the account logic contract.
    function upgradeAccountLogic(address newLogic) external;

    /// Only the setting extension contract can call this function to change the registered verifier contract.
    function changeVerifier(address newVerifier) external;

    /// Only the extension-manager extension contract can call this function to change the registered extension contract.
    function changeExtension(uint extensionId, address extensionAddr) external;

    /// Only the extension-manager extension contract can call this function to add permissions when the user installs a new extension.
    function addPermission(uint256 fromId, uint256 toId) external;

    /// Only the extension-manager extension contract can call this function to add permissions when the user removes a new extension.
    function removePermission(uint256 fromId, uint256 toId) external;
}
