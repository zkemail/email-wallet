// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Extension} from "../../src/interfaces/Extension.sol";
import {EmailWalletCore} from "../../src/EmailWalletCore.sol";
import "../../src/interfaces/Types.sol";

contract TestNFTExtension is Extension, IERC721Receiver {
    EmailWalletCore core;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](0);

    constructor(address coreAddr) {
        EmailWalletCore core = EmailWalletCore(payable(coreAddr));

        // Deploy a NFT contract
        ERC721 dummyApes = new ERC721("DummyApes", "APE");
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
        bytes32 emailNullifier
    ) external override {
        (uint256 tokenId, string memory nftName) = abi.decode(subjectParams[0], (uint256, string));

        require(msg.sender == address(core), "invalid sender");
        require(templateIndex == 0, "invalid templateIndex");
        require(addressOfNFTName[nftName] != address(0), "invalid NFT");
        require(tokenId != 0, "invalid tokenId");

        if (hasEmailRecipient) {
            bytes memory unclaimedState = abi.encode("(address, uint256)", addressOfNFTName[nftName], tokenId);
            core.registerUnclaimedStateAsExtension(address(this), unclaimedState);
        } else {
            require(recipientETHAddr != address(0), "invalid recipientETHAddr");

            bytes memory data = abi.encodeWithSignature(
                "safeTransferFrom(address,address,uint256)",
                wallet,
                recipientETHAddr,
                tokenId
            );
            core.executeAsExtension(addressOfNFTName[nftName], data);
        }
    }

    function registerUnclaimedState(
        UnclaimedState memory unclaimedState,
        bool isInternal
    ) public override returns (bool) {
        require(msg.sender == address(core), "invalid sender");

        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token to this
        bytes memory data = abi.encodeWithSignature(
            "safeTransferFrom(address,address,uint256)",
            unclaimedState.sender,
            address(this),
            tokenId
        );
        core.executeAsExtension(nftAddr, data);

        return true;
    }

    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external override {
        require(msg.sender == address(core), "invalid sender");

        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token to wallet
        ERC721(nftAddr).safeTransferFrom(address(this), wallet, tokenId);
    }

    function voidUnclaimedState(UnclaimedState memory unclaimedState) external override {
        require(msg.sender == address(core), "invalid sender");

        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer token back to sender
        ERC721(nftAddr).safeTransferFrom(address(this), unclaimedState.sender, tokenId);
    }
}
