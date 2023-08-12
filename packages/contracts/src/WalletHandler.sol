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
    // // Time in block count for a transfer to be refundable (for uninitialized recipient)
    // uint256 public constant REFUND_PERIOD_IN_BLOCKS = 5 * 60 * 24 * 30;  // 30 days (5 blocks per minute)

    // // Mapping of transfers that are refundable after block number
    // mapping(uint256 => TransferNote[]) public refundableTransfersAfterBlock;

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

    // function registerRefundableTransfer(
    //     address sender,
    //     address recipient,
    //     string memory tokenName,
    //     uint256 amount
    // ) internal {
    //     uint256 refundableAfterBlock = block.number + REFUND_PERIOD_IN_BLOCKS;
    //     refundableTransfersAfterBlock[refundableAfterBlock].push(
    //         TransferNote({
    //             amount: amount,
    //             tokenName: tokenName,
    //             sender: sender,
    //             recipient: recipient
    //         })
    //     );
    // }

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
}
