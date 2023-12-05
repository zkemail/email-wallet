// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";
import "../interfaces/Commands.sol";

contract ExtensionHandler is Initializable, UUPSUpgradeable, OwnableUpgradeable {
    bool _defaultExtensionsSet;

    // Deployer
    address private deployer;

    // Mapping from extensio name to extension address, as published by the developer
    mapping(string => address) public addressOfExtensionName;

    // Global mapping of command name to extension address enabled for all users by default
    mapping(string => address) public defaultExtensionOfCommand;

    // Mapping of extension address to list of subjectTemplates
    // Each subject template is array of strings, where each item is a "matcher" or constant string
    // Eg: `["Swap", "{tokenAmount}", "to", "{string}"]`
    mapping(address => string[][]) public subjectTemplatesOfExtension;

    // Mapping of extension address to maximum gas that will be consumed by `extension.execute()`
    // Relayer can use this to ensure user has enough tokens to pay for the gas
    mapping(address => uint256) public maxGasOfExtension;

    // User level mapping of command name to extension address (walletAddress -> (command -> extension))
    mapping(address => mapping(string => address)) public userExtensionOfCommand;

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

    /// Set default extensions for all users
    /// @param defaultExtensions Array of default extensions encoded as (string name, address addr, string[][] templates, uint256 maxGas)
    function setDefaultExtensions(bytes[] calldata defaultExtensions) public onlyOwner {
        require(!_defaultExtensionsSet, "default extensions already set");

        for (uint256 i = 0; i < defaultExtensions.length; i++) {
            (string memory name, address addr, string[][] memory templates, uint256 maxGas) = abi.decode(
                defaultExtensions[i],
                (string, address, string[][], uint256)
            );

            addressOfExtensionName[name] = addr;
            subjectTemplatesOfExtension[addr] = templates;
            maxGasOfExtension[addr] = maxGas;
            defaultExtensionOfCommand[templates[0][0]] = addr;
        }

        _defaultExtensionsSet = true;
    }

    /// Register a new extension
    /// @param name Name of the extension
    /// @param addr Address of the extension contract
    /// @param subjectTemplates Subject templates for the extension
    /// @param maxExecutionGas Max gas allowed for the extension
    /// @dev First word of each subject template should be same and is called "command"; command should be one word
    function publishExtension(
        string calldata name,
        address addr,
        string[][] memory subjectTemplates,
        uint256 maxExecutionGas
    ) public {
        require(addressOfExtensionName[name] == address(0), "extension name already used");
        require(addr != address(0), "invalid extension address");
        require(bytes(name).length > 0, "invalid extension name");
        require(maxExecutionGas > 0, "maxExecutionGas must be larger than zero");
        require(subjectTemplates.length > 0, "subjectTemplates array cannot be empty");
        require(maxGasOfExtension[addr] == 0, "extension already published");

        // Check if all subjectTemplates have same command (first item in array)
        string memory command;
        for (uint i = 0; i < subjectTemplates.length; i++) {
            require(subjectTemplates[i].length > 0, "subjectTemplate cannot be empty");
            if (i == 0) {
                command = subjectTemplates[i][0];
            } else {
                require(Strings.equal(command, subjectTemplates[i][0]), "subjectTemplates must have same command");
            }
            uint numRecipient = 0;
            for (uint j = 1; j < subjectTemplates[i].length; j++) {
                if (Strings.equal(subjectTemplates[i][j], Commands.RECIPIENT_TEMPLATE)) {
                    numRecipient++;
                }
            }
            require(numRecipient <= 1, "recipient template can only be used once");
        }

        // Check if command is only one word (no spaces)
        for (uint i = 0; i < bytes(command).length; i++) {
            require(bytes(command)[i] != 0x20, "command should be one word");
        }

        // Check if command is not a reserved name
        require(
            !Strings.equal(command, Commands.SEND) &&
                !Strings.equal(command, Commands.EXECUTE) &&
                !Strings.equal(command, Commands.INSTALL_EXTENSION) &&
                !Strings.equal(command, Commands.UNINSTALL_EXTENSION) &&
                !Strings.equal(command, Commands.EXIT_EMAIL_WALLET) &&
                !Strings.equal(command, Commands.DKIM),
            "command cannot be a reserved name"
        );

        // Check if command is not a template
        require(
            !Strings.equal(command, Commands.TOKEN_AMOUNT_TEMPLATE) &&
                !Strings.equal(command, Commands.AMOUNT_TEMPLATE) &&
                !Strings.equal(command, Commands.STRING_TEMPLATE) &&
                !Strings.equal(command, Commands.UINT_TEMPLATE) &&
                !Strings.equal(command, Commands.INT_TEMPLATE) &&
                !Strings.equal(command, Commands.ADDRESS_TEMPLATE) &&
                !Strings.equal(command, Commands.RECIPIENT_TEMPLATE),
            "command cannot be a template matcher"
        );

        addressOfExtensionName[name] = addr;
        subjectTemplatesOfExtension[addr] = subjectTemplates;
        maxGasOfExtension[addr] = maxExecutionGas;

        emit EmailWalletEvents.ExtensionPublished(name, addr, subjectTemplates, maxExecutionGas);
    }

    /// @notice Set the default extension for a command
    /// @param walletAddr The user's wallet address
    /// @param command Command for which the extension address is to be set
    /// @param extensionAddr Address of the extension
    function setExtensionForCommand(
        address walletAddr,
        string memory command,
        address extensionAddr
    ) external onlyOwner {
        userExtensionOfCommand[walletAddr][command] = extensionAddr;
    }

    /// @notice Return the extension address for a command and user
    /// @param walletAddr The user's wallet address
    /// @param command Command for which the extension address is to be returned
    function getExtensionForCommand(address walletAddr, string memory command) public view returns (address) {
        address extensionAddr;
        address userextensionAddr = userExtensionOfCommand[walletAddr][command];

        if (userextensionAddr != address(0)) {
            extensionAddr = userextensionAddr;
        } else {
            extensionAddr = defaultExtensionOfCommand[command]; // Global extension installed by default for all users
        }

        return extensionAddr;
    }

    /// @notice Return the subject templates for an extension
    /// @param extensionAddr Address of the extension
    function getSubjectTemplatesOfExtension(address extensionAddr) public view returns (string[][] memory) {
        return subjectTemplatesOfExtension[extensionAddr];
    }
}
