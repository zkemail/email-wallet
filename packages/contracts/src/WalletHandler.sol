// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/proxy/Clones.sol";
import "./TokenRegistry.sol";
import "./Wallet.sol";
import "./interfaces/Types.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Constants.sol";

contract WalletHandler is TokenRegistry {
    address public immutable walletImplementation;

    constructor() {
        Wallet wallet = new Wallet();
        wallet.initialize();
        walletImplementation = address(wallet);
    }

    function getAddressOfSalt(bytes32 salt) public view returns (address) {
        return Clones.predictDeterministicAddress(walletImplementation, salt);
    }

    function _deployWallet(bytes32 salt) internal returns (address) {
        Wallet wallet = Wallet(payable(Clones.cloneDeterministic(walletImplementation, salt)));
        wallet.initialize();
        return address(wallet);
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

        (bool success, bytes memory returnData) = sender.execute(
            tokenAddress,
            0,
            abi.encodeWithSignature("transfer(address,uint256)", recipientAddress, amount)
        );

        require(success, string(returnData));

        if (returnData.length > 0) {
            bool internalSuccess;
            assembly {
                internalSuccess := mload(add(returnData, 0x20))
            }
            require(internalSuccess, "ERC20 transfer failed");
        }
    }

    function _processTransferRequest(
        address senderAddress,
        address recipientAddress,
        string memory tokenName,
        uint256 amount
    ) internal {
        if (Strings.equal(tokenName, Constants.ETH_TOKEN_NAME)) {
            _processETHTransferRequest(senderAddress, recipientAddress, amount);
        } else {
            _processERC20TransferRequest(senderAddress, recipientAddress, tokenName, amount);
        }
    }

    function _executeExtensionCalldata(
        address senderAddress,
        address target,
        bytes memory extensionCallData
    ) internal {
        Wallet sender = Wallet(payable(senderAddress));

        (bool success, bytes memory returnData) = sender.execute(target, 0, extensionCallData);

        require(success, string(returnData));
    }
}
