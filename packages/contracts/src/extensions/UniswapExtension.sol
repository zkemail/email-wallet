// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "../interfaces/Types.sol";

import "./PoolFinder.sol";

contract UniswapExtension is Extension {
    EmailWalletCore public core;
    ISwapRouter public router;
    TokenRegistry public tokenRegistry;
    PoolFinder public poolFinder;

    // For this example, we will set the pool fee to 0.3%.
    uint24 public constant poolFee = 3000;
    // It's a default slippage, we will set the maximum slippage to 0.5%.
    uint24 public constant slippageBasisPoints = 50;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](2);

    modifier onlyCore() {
        require((msg.sender == address(core)) || (msg.sender == address(core.unclaimsHandler())), "invalid sender");
        _;
    }

    constructor(address coreAddr, address _tokenReg, address _router, address _factory) {
        core = EmailWalletCore(payable(coreAddr));
        tokenRegistry = TokenRegistry(_tokenReg);
        router = ISwapRouter(_router);
        templates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        templates[1] = ["Swap", "{tokenAmount}", "to", "{string}", "under", "{uint}", "sqrt price limit"];
        poolFinder = new PoolFinder(IUniswapV3Factory(_factory));
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external override onlyCore {
        recipientETHAddr;
        emailNullifier;
        require(templateIndex <= 1, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        (uint256 tokenInAmount, string memory tokenIn) = abi.decode(subjectParams[0], (uint256, string));

        uint160 sqrtPriceLimitX96;
        string memory tokenOut = abi.decode(subjectParams[1], (string));
        if (templateIndex == 0) {
            sqrtPriceLimitX96 = 0;
        } else {
            uint256 sqrtPriceLimitX96Uint256 = abi.decode(subjectParams[2], (uint256));
            require(sqrtPriceLimitX96Uint256 <= type(uint160).max, "sqrtPriceLimitX96 argument overflow detected");
            sqrtPriceLimitX96 = uint160(sqrtPriceLimitX96Uint256);
        }
        address tokenInAddr = tokenRegistry.getTokenAddress(tokenIn);
        address tokenOutAddr = tokenRegistry.getTokenAddress(tokenOut);
        require(tokenOutAddr != address(0), "invalid out token name");
        // To avoid stack too deep, we use the following bracket
        {
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
        }
        address wethAddr = tokenRegistry.getTokenAddress("ETH");
        if (tokenInAddr != wethAddr && tokenOutAddr != wethAddr) {
            ISwapRouter.ExactInputSingleParams memory swapParams1 = ISwapRouter.ExactInputSingleParams({
                tokenIn: tokenInAddr,
                tokenOut: wethAddr,
                fee: poolFee,
                recipient: address(this),
                deadline: block.timestamp,
                amountIn: tokenInAmount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: getSqrtPriceLimitX96(tokenInAddr, wethAddr, sqrtPriceLimitX96)
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
                sqrtPriceLimitX96: getSqrtPriceLimitX96(wethAddr, tokenOutAddr, sqrtPriceLimitX96)
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
                sqrtPriceLimitX96: getSqrtPriceLimitX96(tokenInAddr, tokenOutAddr, sqrtPriceLimitX96)
            });
            router.exactInputSingle(swapParams);
        }
    }

    /// @notice Get the price limit for the swap if sqrtPriceLimitX96 is not set, this function uses the current price
    /// @param tokenIn Token to be swapped
    /// @param tokenOut Token to be received
    /// @param sqrtPriceLimitX96 The price limit for the swap
    /// @return The price limit for the swap with slippage
    /// @dev minPriceX96 The minimum price for the swap it used for token0(tokenIn) -> token1(tokenOut)
    /// @dev maxPriceX96 The maximum price for the swap it used for token1(tokenIn) -> token0(tokenOut)
    function getSqrtPriceLimitX96(
        address tokenIn,
        address tokenOut,
        uint160 sqrtPriceLimitX96
    ) internal view returns (uint160) {
        bool zeroForOne = tokenIn < tokenOut;

        IUniswapV3Pool pool = poolFinder.getPool(tokenIn, tokenOut, poolFee);
        (uint160 sqrtPriceX96, , , , , , ) = pool.slot0();

        if (sqrtPriceLimitX96 == 0) {
            sqrtPriceLimitX96 = sqrtPriceX96;
        }

        uint160 minPriceX96 = sqrtPriceLimitX96 - ((sqrtPriceLimitX96 * slippageBasisPoints) / 10000);
        uint160 maxPriceX96 = sqrtPriceLimitX96 + ((sqrtPriceLimitX96 * slippageBasisPoints) / 10000);

        if (zeroForOne) {
            return minPriceX96;
        } else {
            return maxPriceX96;
        }
    }
}
