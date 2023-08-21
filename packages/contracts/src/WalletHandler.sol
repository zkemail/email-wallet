// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./TokenRegistry.sol";
import "./Wallet.sol";
import "./interfaces/Types.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Constants.sol";

contract WalletHandler is TokenRegistry {
    function getAddressOfSalt(bytes32 salt) public view returns (address) {
        return Create2.computeAddress(salt, keccak256(type(Wallet).creationCode));
    }

    // Deploy a wallet for the user account with the given salt
    // TODO: Use clone factory to deploy proxy Wallet contracts
    function _deployWallet(bytes32 salt) internal returns (address) {
        address walletAddress = Create2.deploy(0, salt, type(Wallet).creationCode);

        return walletAddress;
    }

    function _processETHTransferRequest(
        address senderAddress,
        address recipientAddress,
        uint256 amount
    ) internal {
        Wallet sender = Wallet(payable(senderAddress));

        (bool success, bytes memory returnData) = sender.execute(recipientAddress, amount, "");

        require(success, string(returnData));
    }

    function _processERC20TransferRequest(
        address senderAddress,
        address recipientAddress,
        string memory tokenName,
        uint256 amount
    ) internal {
        address tokenAddress = getTokenAddress(tokenName);
        require(tokenAddress != address(0), "unsupported token");

        Wallet sender = Wallet(payable(senderAddress));

        sender.execute(
            tokenAddress,
            0,
            abi.encodeWithSignature("transfer(address,uint256)", recipientAddress, amount)
        );
    }

    function _processTransferRequest(
        bytes32 senderSalt,
        bytes32 recipientSalt,
        string memory tokenName,
        uint256 amount
    ) internal {
        address senderAddress = getAddressOfSalt(senderSalt);
        address recipientAddress = getAddressOfSalt(recipientSalt);

        if (Strings.equal(tokenName, Constants.ETH_TOKEN_NAME)) {
            _processETHTransferRequest(senderAddress, recipientAddress, amount);
        } else {
            _processERC20TransferRequest(senderAddress, recipientAddress, tokenName, amount);
        }
    }

    function _executeExtensionCalldata(
        bytes32 senderSalt,
        address target,
        bytes memory extensionCallData
    ) internal {
        address senderAddress = getAddressOfSalt(senderSalt);
        Wallet sender = Wallet(payable(senderAddress));

        (bool success, bytes memory returnData) = sender.execute(target, 0, extensionCallData);

        require(success, string(returnData));
    }
}
