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

    /// @notice Register as a relayer
    /// @param emailAddr relayer's email address
    /// @param hostname hostname of relayer's server - used by other relayers for PSI communication
    function registerRelayer(string calldata emailAddr, string calldata hostname) public {
        require(bytes(emailAddr).length != 0, "emailAddr cannot be empty");
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayerOfEmailAddr[emailAddr] == address(0), "emailAddr already registered");

        relayers[msg.sender] = RelayerConfig({emailAddr: emailAddr, hostname: hostname});
        relayerOfEmailAddr[emailAddr] = msg.sender;

        emit EmailWalletEvents.RelayerRegistered(msg.sender, emailAddr, hostname);
    }

    /// @notice Update relayer's config (hostname only for now)
    /// @param hostname new hostname of relayer's server
    function updateRelayerConfig(string calldata hostname) public {
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(bytes(relayers[msg.sender].emailAddr).length != 0, "relayer not registered");
        relayers[msg.sender].hostname = hostname;

        emit EmailWalletEvents.RelayerConfigUpdated(msg.sender, hostname);
    }
}
