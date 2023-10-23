// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract DummyNFT is ERC721 {
    constructor() ERC721("DummyApes", "APE") {}

    function freeMint(address to, uint256 tokenId) external {
        _mint(to, tokenId);
    }
}
