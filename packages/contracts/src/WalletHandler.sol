// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./TokenRegistry.sol";
import "./Wallet.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Constants.sol";

contract WalletHandler is TokenRegistry {
    struct Transfer {
        uint256 amount;
        address token;
        address sender;
        address recipient;
    }

    // Mapping of transfers that are refundable after block number
    // if the recipient account is not initialized
    mapping(uint256 => Transfer[]) public refundableTransfersAfterBlock;

    function getAddressOfSalt(bytes32 salt) public view returns (address) {
        return
            Create2.computeAddress(salt, keccak256(type(Wallet).creationCode));
    }

    // Deploy a wallet for the user account with the given salt
    // TODO: Use clone factory to deploy proxy Wallet contracts
    function _deployWallet(bytes32 salt) internal returns (address) {
        address walletAddress = Create2.deploy(
            0,
            salt,
            type(Wallet).creationCode
        );

        return walletAddress;
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

        Wallet sender = Wallet(payable(getAddressOfSalt(senderSalt)));

        sender.execute(
            tokenAddress,
            0,
            abi.encodeWithSignature(
                "transfer(address,uint256)",
                getAddressOfSalt(recipientSalt),
                amount
            )
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
            _processERC20TransferRequest(
                senderSalt,
                recipientSalt,
                tokenName,
                amount
            );
        }
    }
}
