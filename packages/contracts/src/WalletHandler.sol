// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./TokenRegistry.sol";
import "./Wallet.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Constants.sol";

contract WalletHandler is TokenRegistry {
    function getAddressOfSalt(bytes32 salt) public view returns (address) {
        return Create2.computeAddress(salt, keccak256(type(Wallet).creationCode));
    }

    function _processETHTransferRequest(
        bytes32 senderSalt,
        bytes32 recipientSalt,
        uint256 amount
    ) internal {
        Wallet sender = Wallet(payable(getAddressOfSalt(senderSalt)));

        (bool success, bytes memory returnData) = sender.execute(
            getAddressOfSalt(recipientSalt),
            amount,
            ""
        );

        require(success, string(returnData));
    }

    function _processERC20TransferRequest(
        bytes32 senderSalt,
        bytes32 recipientSalt,
        string memory tokenName,
        uint256 amount
    ) internal {
        address tokenAddress = getTokenAddress(tokenName);
        require(tokenAddress != address(0), "unsupported token");

        IERC20 token = IERC20(tokenAddress);

        token.transferFrom(
            getAddressOfSalt(senderSalt),
            getAddressOfSalt(recipientSalt),
            amount
        );
    }

    function _processTransferRequest(
        bytes32 senderSalt,
        bytes32 recipientSalt,
        string memory tokenName,
        uint256 amount
    ) internal {
        if (Strings.equal(tokenName, Constants.ETH_TOKEN_NAME)) {
            _processETHTransferRequest(senderSalt, recipientSalt, amount);
        } else {
            _processERC20TransferRequest(senderSalt, recipientSalt, tokenName, amount);
        }
    }
}
