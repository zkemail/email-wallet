// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Extension} from "../../src/interfaces/Extension.sol";
import {EmailWalletCore} from "../../src/EmailWalletCore.sol";
import "../../src/interfaces/Types.sol";

contract DummyApes is ERC721 {
    constructor() ERC721("DummyApes", "APE") {}

    function freeMint(address to, uint256 tokenId) external {
        _mint(to, tokenId);
    }
}

contract TestNFTExtension is Extension, IERC721Receiver {
    EmailWalletCore public core;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](1);

    modifier onlyCore() {
        require(msg.sender == address(core), "invalid sender");
        _;
    }

    constructor(address coreAddr) {
        core = EmailWalletCore(payable(coreAddr));

        // Deploy a NFT contract
        DummyApes dummyApes = new DummyApes();
        addressOfNFTName["APE"] = address(dummyApes);

        templates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
    }

    /// @inheritdoc IERC721Receiver
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        return IERC721Receiver.onERC721Received.selector;
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32
    ) external override onlyCore {
        if (templateIndex != 0) {
            return; // Just to support testing with unsupported subjects
        }

        uint256 tokenId = abi.decode(subjectParams[0], (uint256));
        string memory nftName = abi.decode(subjectParams[1], (string));

        require(addressOfNFTName[nftName] != address(0), "invalid NFT");
        require(tokenId != 0, "invalid tokenId");

        if (hasEmailRecipient) {
            bytes memory unclaimedState = abi.encode("(address, uint256)", addressOfNFTName[nftName], tokenId);
            core.registerUnclaimedStateAsExtension(address(this), unclaimedState);
        } else {
            require(recipientETHAddr != address(0), "invalid recipientETHAddr");

            bytes memory data = abi.encodeWithSignature(
                "transferFrom(address,address,uint256)",
                wallet,
                recipientETHAddr,
                tokenId
            );
            core.executeAsExtension(addressOfNFTName[nftName], data);
        }
    }

    function registerUnclaimedState(
        UnclaimedState memory unclaimedState,
        bool 
    ) public override onlyCore returns (bool) {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token to this
        bytes memory data = abi.encodeWithSignature(
            "transferFrom(address,address,uint256)",
            unclaimedState.sender,
            address(this),
            tokenId
        );

        try core.executeAsExtension(nftAddr, data) {
        } catch {
            return false;
        }

        return true;
    }

    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external override onlyCore {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token to wallet
        ERC721(nftAddr).transferFrom(address(this), wallet, tokenId);
    }

    function voidUnclaimedState(UnclaimedState memory unclaimedState) external override onlyCore {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token back to sender
        ERC721(nftAddr).transferFrom(address(this), unclaimedState.sender, tokenId);
    }
}
