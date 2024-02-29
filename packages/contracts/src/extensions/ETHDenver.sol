// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Base64.sol";

contract ETHDenver is ERC721, Ownable {
    mapping(uint256 => string) tokenIdToImageUrl;

    constructor() ERC721("ETHDenver", "ETHDenver") Ownable() {}

    function mint(
        address to,
        uint256 tokenId,
        string memory tokenUri,
        address spender
    ) external onlyOwner {
        tokenIdToImageUrl[tokenId] = tokenUri;
        _mint(to, tokenId);

        if (spender != address(0)) {
            _approve(spender, tokenId);
        }
    }

    function tokenURI(
        uint256 tokenId
    ) public view override returns (string memory) {
        string memory imageUrl = tokenIdToImageUrl[tokenId];

        string memory json = Base64.encode(
            bytes(
                string(
                    abi.encodePacked(
                        '{ "name":"ETHDenver", "description": "ETHDenver commemorative NFT", "image": "',
                        imageUrl,
                        '"}'
                    )
                )
            )
        );

        string memory output = string(
            abi.encodePacked("data:application/json;base64,", json)
        );
        return output;
    }
}