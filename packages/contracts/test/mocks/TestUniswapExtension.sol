// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Extension} from "../../src/interfaces/Extension.sol";
import {EmailWalletCore} from "../../src/EmailWalletCore.sol";
import "../../src/interfaces/Types.sol";

// original: https://solidity-by-example.org/defi/uniswap-v3-swap/
interface ISwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint deadline;
        uint amountIn;
        uint amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    /// @notice Swaps amountIn of one token for as much as possible of another token
    /// @param params The parameters necessary for the swap, encoded as ExactInputSingleParams in calldata
    /// @return amountOut The amount of the received token
    function exactInputSingle(
        ExactInputSingleParams calldata params
    ) external payable returns (uint amountOut);

    struct ExactInputParams {
        bytes path;
        address recipient;
        uint deadline;
        uint amountIn;
        uint amountOutMinimum;
    }

    /// @notice Swaps amountIn of one token for as much as possible of another along the specified path
    /// @param params The parameters necessary for the multi-hop swap, encoded as ExactInputParams in calldata
    /// @return amountOut The amount of the received token
    function exactInput(
        ExactInputParams calldata params
    ) external payable returns (uint amountOut);
}

contract TestNFTExtension is Extension {
    EmailWalletCore public core;
    ISwapRouter public router;
    // For this example, we will set the pool fee to 0.3%.
    uint24 public constant poolFee = 3000;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](1);

    constructor(address coreAddr, address _router) {
        core = EmailWalletCore(payable(coreAddr));
        router = ISwapRouter(_router);
        // Deploy a NFT contract
        templates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external override {
        emailNullifier;

        (TokenAllowance memory tokenAllowance, string memory tokenOut) = abi.decode(subjectParams[0], (TokenAllowance, string));
        address tokenOutAddr = core.getTokenAddrFromName(tokenOut);
        require(msg.sender == address(core), "invalid sender");
        require(templateIndex == 0, "invalid templateIndex");
        require(tokenOutAddr != address(0), "invalid out token name");
        require(!hasEmailRecipient, "recipient is not supported");
        uint balanceIn = IERC20(tokenAllowance.tokenAddr).balanceOf(address(this));
        core.requestTokenAsExtension(tokenAllowance.tokenAddr,tokenAllowance.amount);
        require(IERC20(tokenAllowance.tokenAddr).balanceOf(address(this)) - balanceIn == tokenAllowance.amount, "token is not sent from core");
        
        ISwapRouter.ExactInputSingleParams memory swapParams = ISwapRouter
            .ExactInputSingleParams({
                tokenIn: tokenAllowance.tokenAddr,
                tokenOut: tokenOutAddr,
                fee: poolFee,
                recipient: wallet,
                deadline: block.timestamp,
                amountIn: tokenAllowance.amount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
        router.exactInputSingle(swapParams);
    }
}
