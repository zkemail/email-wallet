// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import "../interfaces/Types.sol";

contract NFTExtension is Extension, IERC721Receiver, Ownable {
    EmailWalletCore public core;

    // Mapping from NFT name to its address
    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](2);

    modifier onlyCore() {
        require((msg.sender == address(core)) || (msg.sender == address(core.unclaimsHandler())), "invalid sender");
        _;
    }

    constructor(address coreAddr) {
        core = EmailWalletCore(payable(coreAddr));
        templates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        templates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
    }

    /// @inheritdoc IERC721Receiver
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        return IERC721Receiver.onERC721Received.selector;
    }

    /// @notice Set NFT address for its name
    /// @param  nftName NFT name
    /// @param addr Address of the NFT
    function setNFTAddress(string memory nftName, address addr) public onlyOwner {
        require(addressOfNFTName[nftName] == address(0), "NFT already registered");
        require(addr != address(0), "invalid address");
        addressOfNFTName[nftName] = addr;
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32
    ) external override onlyCore {
        // Parse subject tempaltes
        // This can be common for both templates as [0] is `tokenId` and [1] is `nftName` (`recipient` is not included in subjectParams)
        uint256 tokenId = abi.decode(subjectParams[0], (uint256));
        string memory nftName = abi.decode(subjectParams[1], (string));
        address nftAddr = addressOfNFTName[nftName];
        require(nftAddr != address(0), "invalid NFT");

        // NFT Send
        if (templateIndex == 0) {
            // If sending to email, approve for "this" (to transfer later) and register unclaimed state
            if (hasEmailRecipient) {
                bytes memory data = abi.encodeWithSignature("approve(address,uint256)", address(this), tokenId);
                core.executeAsExtension(nftAddr, data);

                bytes memory unclaimedState = abi.encode(nftAddr, tokenId);
                core.registerUnclaimedStateAsExtension(address(this), unclaimedState);
            }
            // If recipient is ETH addr, send directly to recipient
            else {
                require(recipientETHAddr != address(0), "should have recipientETHAddr");

                bytes memory data = abi.encodeWithSignature(
                    "safeTransferFrom(address,address,uint256)",
                    wallet,
                    recipientETHAddr,
                    tokenId
                );
                core.executeAsExtension(nftAddr, data);
            }

            return;
        }

        // NFT Approve
        if (templateIndex == 1) {
            require(recipientETHAddr != address(0), "should have ETH add for approve");

            bytes memory data = abi.encodeWithSignature("approve(address,uint256)", recipientETHAddr, tokenId);
            core.executeAsExtension(nftAddr, data);

            return;
        }

        revert("invalid templateIndex");
    }

    function registerUnclaimedState(UnclaimedState memory unclaimedState, bool) public override onlyCore {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        IERC721 nft = IERC721(nftAddr);
        require(IERC721(nftAddr).getApproved(tokenId) == address(this), "NFT not approved to extension");

        nft.safeTransferFrom(unclaimedState.sender, address(this), tokenId);
        require(nft.ownerOf(tokenId) == address(this), "NFT not transferred to extension");
    }

    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external override onlyCore {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer nft to wallet
        IERC721(nftAddr).safeTransferFrom(address(this), wallet, tokenId);
    }

    function voidUnclaimedState(UnclaimedState memory unclaimedState) external override onlyCore {
        (address nftAddr, uint256 tokenId) = abi.decode(unclaimedState.state, (address, uint256));

        // Transfer nft back to sender
        IERC721(nftAddr).safeTransferFrom(address(this), unclaimedState.sender, tokenId);
    }
}
