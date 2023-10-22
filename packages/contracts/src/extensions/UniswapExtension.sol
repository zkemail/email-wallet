// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "../interfaces/Types.sol";

contract UniswapExtension is Extension {
    EmailWalletCore public core;
    ISwapRouter public router;
    TokenRegistry public tokenRegistry;

    // For this example, we will set the pool fee to 0.3%.
    uint24 public constant poolFee = 3000;

    mapping(string => address) public addressOfNFTName;

    string[][] public templates = new string[][](1);

    modifier onlyCore() {
        require(msg.sender == address(core), "invalid sender");
        _;
    }

    constructor(address coreAddr, address _tokenReg, address _router) {
        core = EmailWalletCore(payable(coreAddr));
        tokenRegistry = TokenRegistry(_tokenReg);
        router = ISwapRouter(_router);
        templates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
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
        (uint256 tokenInAmount, string memory tokenIn) = abi.decode(subjectParams[0], (uint256, string));
        string memory tokenOut = abi.decode(subjectParams[1], (string));
        address tokenInAddr = tokenRegistry.getTokenAddress(tokenIn);
        address tokenOutAddr = tokenRegistry.getTokenAddress(tokenOut);
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
