// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "../interfaces/Types.sol";

import "./PoolFinder.sol";

contract UniswapExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
    EmailWalletCore public core;
    ISwapRouter public router;
    TokenRegistry public tokenRegistry;
    PoolFinder public poolFinder;

    // For this example, we will set the pool fee to 0.3%.
    uint24 public constant poolFee = 3000;
    // It's a default slippage, we will set the maximum slippage to 0.5%.
    uint24 public constant defaultSlippagePoints = 50;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates;

    modifier onlyCore() {
        require((msg.sender == address(core)) || (msg.sender == address(core.unclaimsHandler())), "invalid sender");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(address coreAddr, address _tokenReg, address _router, address _factory) initializer public {
        __Ownable_init();
        core = EmailWalletCore(payable(coreAddr));
        tokenRegistry = TokenRegistry(_tokenReg);
        router = ISwapRouter(_router);
        templates = new string[][](4);
        templates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        templates[1] = ["Swap", "{tokenAmount}", "to", "{string}", "with", "{amount}", "slippage"];
        templates[2] = ["Swap", "{tokenAmount}", "to", "{string}", "under", "{uint}", "sqrt", "price", "limit"];
        templates[3] = [
            "Swap",
            "{tokenAmount}",
            "to",
            "{string}",
            "with",
            "{amount}",
            "slippage",
            "under",
            "{uint}",
            "sqrt",
            "price",
            "limit"
        ];
        poolFinder = new PoolFinder(IUniswapV3Factory(_factory));
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        onlyOwner
        override
    {}

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
        require(templateIndex <= 3, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        uint256 tokenInAmount;
        address tokenInAddr;
        address tokenOutAddr;
        uint24 slippagePoints;
        uint160 sqrtPriceLimitX96;

        {
            // subjectParams[0] and subjectParams[1] are same for all templates
            (uint256 tokenInAmountParam, string memory tokenIn) = abi.decode(subjectParams[0], (uint256, string));
            tokenInAmount = tokenInAmountParam;
            string memory tokenOut = abi.decode(subjectParams[1], (string));
            tokenInAddr = tokenRegistry.getTokenAddress(tokenIn);
            tokenOutAddr = tokenRegistry.getTokenAddress(tokenOut);
            require(tokenOutAddr != address(0), "invalid out token name");
        }

        // Check if the pool exists
        bool isPoolExists = poolFinder.isPoolExists(tokenInAddr, tokenOutAddr, poolFee);

        // Each template has different input parameters
        if (templateIndex == 0) {
            slippagePoints = defaultSlippagePoints;

            sqrtPriceLimitX96 = 0;
        }

        if (templateIndex == 1) {
            uint256 slippagePoints256 = abi.decode(subjectParams[2], (uint256));
            // This value is user input * 10^18, we need to revert it as (user input * 10^2).
            slippagePoints256 = slippagePoints256 / 10 ** 16;
            require(slippagePoints256 <= type(uint24).max, "slippagePoints256 argument overflow detected");
            slippagePoints = uint24(slippagePoints256);

            sqrtPriceLimitX96 = 0;
        }

        if (templateIndex == 2) {
            require(isPoolExists, "sqrtPriceLimitX96 can not be set because the pool does not exist");

            slippagePoints = defaultSlippagePoints;

            uint256 sqrtPriceLimitX96Uint256;
            sqrtPriceLimitX96Uint256 = abi.decode(subjectParams[2], (uint256));
            require(sqrtPriceLimitX96Uint256 <= type(uint160).max, "sqrtPriceLimitX96 argument overflow detected");
            sqrtPriceLimitX96 = uint160(sqrtPriceLimitX96Uint256);
        }

        if (templateIndex == 3) {
            require(isPoolExists, "sqrtPriceLimitX96 can not be set because the pool does not exist");

            uint256 slippagePoints256 = abi.decode(subjectParams[2], (uint256));
            // This value is user input * 10^18, we need to revert it as (user input * 10^2).
            slippagePoints256 = slippagePoints256 / 10 ** 16;
            require(slippagePoints256 <= type(uint24).max, "slippagePoints256 argument overflow detected");
            slippagePoints = uint24(slippagePoints256);

            uint256 sqrtPriceLimitX96Uint256;
            sqrtPriceLimitX96Uint256 = abi.decode(subjectParams[3], (uint256));
            require(sqrtPriceLimitX96Uint256 <= type(uint160).max, "sqrtPriceLimitX96 argument overflow detected");
            sqrtPriceLimitX96 = uint160(sqrtPriceLimitX96Uint256);
        }

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
        if (!isPoolExists) {
            // If the pool does not exist, we need to swap the token to WETH first, and then swap WETH to the token.
            ISwapRouter.ExactInputSingleParams memory swapParams1 = ISwapRouter.ExactInputSingleParams({
                tokenIn: tokenInAddr,
                tokenOut: wethAddr,
                fee: poolFee,
                recipient: address(this),
                deadline: block.timestamp,
                amountIn: tokenInAmount,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: getSqrtPriceLimitX96(tokenInAddr, wethAddr, slippagePoints, sqrtPriceLimitX96)
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
                sqrtPriceLimitX96: getSqrtPriceLimitX96(wethAddr, tokenOutAddr, slippagePoints, sqrtPriceLimitX96)
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
                sqrtPriceLimitX96: getSqrtPriceLimitX96(tokenInAddr, tokenOutAddr, slippagePoints, sqrtPriceLimitX96)
            });
            router.exactInputSingle(swapParams);
        }
    }

    /// @notice Get the price limit for the swap if sqrtPriceLimitX96 is not set, this function uses the current price.
    ///         If the pool does not exist, anyone can not estimate the sqrt price limit.
    /// @param tokenIn Token to be swapped
    /// @param tokenOut Token to be received
    /// @param slippagePoints The slippage points for the swap
    /// @param sqrtPriceLimitX96 The price limit for the swap
    /// @return The price limit for the swap with slippage
    /// @dev minPriceX96 The minimum price for the swap it used for token0(tokenIn) -> token1(tokenOut)
    /// @dev maxPriceX96 The maximum price for the swap it used for token1(tokenIn) -> token0(tokenOut)
    function getSqrtPriceLimitX96(
        address tokenIn,
        address tokenOut,
        uint24 slippagePoints,
        uint160 sqrtPriceLimitX96
    ) internal view returns (uint160) {
        bool zeroForOne = tokenIn < tokenOut;

        if (sqrtPriceLimitX96 == 0) {
            bool isPoolExists = poolFinder.isPoolExists(tokenIn, tokenOut, poolFee);
            if (isPoolExists) {
                (uint160 sqrtPriceX96, , , , , , ) = poolFinder.getPoolSlot0(tokenIn, tokenOut, poolFee);
                sqrtPriceLimitX96 = sqrtPriceX96;
            } else {
                return 0;
            }
        }

        uint160 minPriceX96 = sqrtPriceLimitX96 - ((sqrtPriceLimitX96 * slippagePoints) / 10000);
        uint160 maxPriceX96 = sqrtPriceLimitX96 + ((sqrtPriceLimitX96 * slippagePoints) / 10000);

        if (zeroForOne) {
            return minPriceX96;
        } else {
            return maxPriceX96;
        }
    }
}
