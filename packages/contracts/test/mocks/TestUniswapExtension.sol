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
    function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint amountOut);

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
    function exactInput(ExactInputParams calldata params) external payable returns (uint amountOut);
}

contract TestUniswapExtension is Extension {
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
        recipientETHAddr;
        emailNullifier;
        (uint256 tokenInAmount, string memory tokenIn) = abi.decode(subjectParams[0], (uint256, string));
        string memory tokenOut = abi.decode(subjectParams[1], (string));
        address tokenInAddr = core.getTokenAddrFromName(tokenIn);
        address tokenOutAddr = core.getTokenAddrFromName(tokenOut);
        require(msg.sender == address(core), "invalid sender");
        require(templateIndex == 0, "invalid templateIndex");
        require(tokenOutAddr != address(0), "invalid out token name");
        require(!hasEmailRecipient, "recipient is not supported");
        uint balanceIn = IERC20(tokenInAddr).balanceOf(address(this));
        core.requestTokenAsExtension(tokenInAddr, tokenInAmount);
        require(
            IERC20(tokenInAddr).balanceOf(address(this)) - balanceIn == tokenInAmount,
            "token is not sent from core"
        );
        require(
            IERC20(tokenInAddr).approve(address(router), tokenInAmount),
            "approve from the extension to router failed"
        );
        address wethAddr = core.getTokenAddrFromName("ETH");
        if (tokenInAddr != wethAddr && tokenOutAddr != wethAddr) {
            ISwapRouter.ExactInputSingleParams memory swapParams1 = ISwapRouter.ExactInputSingleParams({
                tokenIn: tokenInAddr,
                tokenOut: wethAddr,
                fee: poolFee,
                recipient: address(this),
                deadline: block.timestamp,
                amountIn: tokenInAmount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
            uint wethAmount = router.exactInputSingle(swapParams1);
            require(
                IERC20(wethAddr).approve(address(router), wethAmount),
                "approve from the extension to router failed"
            );
            ISwapRouter.ExactInputSingleParams memory swapParams2 = ISwapRouter.ExactInputSingleParams({
                tokenIn: wethAddr,
                tokenOut: tokenOutAddr,
                fee: poolFee,
                recipient: wallet,
                deadline: block.timestamp,
                amountIn: wethAmount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
            router.exactInputSingle(swapParams2);
        } else {
            ISwapRouter.ExactInputSingleParams memory swapParams = ISwapRouter.ExactInputSingleParams({
                tokenIn: tokenInAddr,
                tokenOut: tokenOutAddr,
                fee: poolFee,
                recipient: wallet,
                deadline: block.timestamp,
                amountIn: tokenInAmount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
            router.exactInputSingle(swapParams);
        }
    }
}
