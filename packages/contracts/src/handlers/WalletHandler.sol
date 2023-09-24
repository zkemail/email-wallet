// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/proxy/Clones.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "../Wallet.sol";
import "../utils/TokenRegistry.sol";
import "../interfaces/Types.sol";
import "../interfaces/IVerifier.sol";
import "../interfaces/Constants.sol";

contract WalletHandler {
    TokenRegistry public immutable tokenRegistry;

    address public immutable walletImplementation;

    constructor(address _tokenRegistry) {
        tokenRegistry = TokenRegistry(_tokenRegistry);

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

    function _transferERC20(
        address senderAddress,
        address recipientAddress,
        address tokenAddress,
        uint256 amount
    ) internal returns (bool success, bytes memory returnData) {
        require(tokenAddress != address(0), "invalid token address");
        require(amount > 0, "invalid amount");
        require(senderAddress != address(0), "invalid sender address");
        require(recipientAddress != address(0), "invalid recipient address");

        Wallet sender = Wallet(payable(senderAddress));

        (success, returnData) = sender.execute(
            tokenAddress,
            0,
            abi.encodeWithSignature("transfer(address,uint256)", recipientAddress, amount)
        );
    }

    function _transferERC20(
        address senderAddress,
        address recipientAddress,
        string memory tokenName,
        uint256 amount
    ) internal {
        address tokenAddress = tokenRegistry.getTokenAddress(tokenName);
        _transferERC20(senderAddress, recipientAddress, tokenAddress, amount);
    }
}
