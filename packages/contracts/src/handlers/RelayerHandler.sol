// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";

contract RelayerHandler is Initializable, UUPSUpgradeable, OwnableUpgradeable {
    // Deployer
    address private deployer;

    // Mapping of relayer's wallet address to relayer config
    mapping(address => RelayerConfig) public relayers;

    // Mapping of relayer's randHash to relayer's wallet address
    mapping(bytes32 => address) public relayerOfRandHash;

    // Mapping of relayer's email address to relayer's wallet address
    mapping(string => address) public relayerOfEmailAddr;

    modifier onlyDeployer() {
        require(msg.sender == deployer, "caller is not a deployer");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize() initializer public {
        __Ownable_init();
        deployer = _msgSender();
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        onlyDeployer
        override
    {}

    function getRandHash(address relayer) public view returns (bytes32) {
        return relayers[relayer].randHash;
    }

    /// @notice Register as a relayer
    /// @param randHash hash of relayed internal randomness `relayerRand`
    /// @param emailAddr relayer's email address
    /// @param hostname hostname of relayer's server - used by other relayers for PSI communication
    function registerRelayer(bytes32 randHash, string calldata emailAddr, string calldata hostname) public {
        require(randHash != bytes32(0), "randHash cannot be empty");
        require(bytes(emailAddr).length != 0, "emailAddr cannot be empty");
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[msg.sender].randHash == bytes32(0), "relayer already registered");
        require(relayerOfRandHash[randHash] == address(0), "randHash already registered");
        require(relayerOfEmailAddr[emailAddr] == address(0), "emailAddr already registered");

        relayers[msg.sender] = RelayerConfig({randHash: randHash, emailAddr: emailAddr, hostname: hostname});
        relayerOfRandHash[randHash] = msg.sender;
        relayerOfEmailAddr[emailAddr] = msg.sender;

        emit EmailWalletEvents.RelayerRegistered(msg.sender, randHash, emailAddr, hostname);
    }

    /// @notice Update relayer's config (hostname only for now)
    /// @param hostname new hostname of relayer's server
    function updateRelayerConfig(string calldata hostname) public {
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");

        relayers[msg.sender].hostname = hostname;

        emit EmailWalletEvents.RelayerConfigUpdated(msg.sender, hostname);
    }
}
