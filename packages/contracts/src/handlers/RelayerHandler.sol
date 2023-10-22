// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";

contract RelayerHandler is Ownable {
    // Mapping of relayer's wallet address to relayer config
    mapping(address => RelayerConfig) public relayers;

    // Mapping of relayer's randHash to relayer's wallet address
    mapping(bytes32 => address) public relayerOfRandHash;

    // Mapping of relayer's email address to relayer's wallet address
    mapping(string => address) public relayerOfEmailAddr;

    function getRandHash(address relayer) public view returns (bytes32) {
        return relayers[relayer].randHash;
    }

    /// @notice Register as a relayer
    /// @param randHash hash of relayed internal randomness `relayerRand`
    /// @param emailAddr relayer's email address
    /// @param hostname hostname of relayer's server - used by other relayers for PSI communication
    function registerRelayer(address relayerAddr, bytes32 randHash, string calldata emailAddr, string calldata hostname) onlyOwner public {
        require(randHash != bytes32(0), "randHash cannot be empty");
        require(bytes(emailAddr).length != 0, "emailAddr cannot be empty");
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[relayerAddr].randHash == bytes32(0), "relayer already registered");
        require(relayerOfRandHash[randHash] == address(0), "randHash already registered");
        require(relayerOfEmailAddr[emailAddr] == address(0), "emailAddr already registered");

        relayers[relayerAddr] = RelayerConfig({randHash: randHash, emailAddr: emailAddr, hostname: hostname});
        relayerOfRandHash[randHash] = relayerAddr;
        relayerOfEmailAddr[emailAddr] = relayerAddr;

        emit EmailWalletEvents.RelayerRegistered(relayerAddr, randHash, emailAddr, hostname);
    }

    /// @notice Update relayer's config (hostname only for now)
    /// @param hostname new hostname of relayer's server
    function updateRelayerConfig(address relayerAddr, string calldata hostname) onlyOwner public {
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[relayerAddr].randHash != bytes32(0), "relayer not registered");

        relayers[relayerAddr].hostname = hostname;

        emit EmailWalletEvents.RelayerConfigUpdated(relayerAddr, hostname);
    }
}
