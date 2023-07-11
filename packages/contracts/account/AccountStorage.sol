// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract AccountStorage {
    string public constant SUBJECT_PREFIX = "Email ";
    uint256 nonce;
    address public coreAddr;
    address public accountLogicAddr;
    address public verifierAddr;
    mapping(uint256 => address) public extensionAddrOfId;
    mapping(address => uint256) public extensionIdOfAddr;
    mapping(uint256 => mapping(uint256 => bool)) public forwardPermissions;
    mapping(bytes32 => bool) public emailNullifiers;
}
