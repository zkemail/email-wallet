pragma solidity ^0.8.12;

import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "@uniswap/v3-core/contracts/libraries/TickMath.sol";
import "@uniswap/v3-core/contracts/libraries/FixedPoint96.sol";
import "@uniswap/v3-core/contracts/libraries/FullMath.sol";
import "../interfaces/IPriceOracle.sol";

/// @title UniswapTWAPOracle
/// @notice Helper methods to get TWAP price of a token (against WETH) from Uniswap V3
/// @dev This contract assumes uniswap pool fee of `3000`
contract UniswapTWAPOracle is IPriceOracle {
    IUniswapV3Factory public factory;
    address public weth;
    uint24 public constant UNISWAP_DEFAULT_POOL_FEE = 3000;

    /// @param factoryAddr Adress of Uniswap V3 factory - https://docs.uniswap.org/contracts/v3/reference/deployments
    /// @param wethAddr Addres of WETH contract
    constructor(address factoryAddr, address wethAddr) {
        factory = IUniswapV3Factory(factoryAddr);
        weth = wethAddr;
    }

    /// @notice Return the price of a token against WETH in wei format
    /// @param tokenAddr Address of the ERC20 token
    /// @param twapInterval TWAP interval in seconds (time window for average price)
    /// @return price Price of the token in wei
    function getTWAPInWETH(address tokenAddr, uint32 twapInterval) public view returns (uint256 price) {
        IUniswapV3Pool pool = IUniswapV3Pool(factory.getPool(tokenAddr, weth, UNISWAP_DEFAULT_POOL_FEE));

        (bool success, bytes memory data) = (pool.token0()).staticcall(abi.encodeWithSignature("decimals()"));
        require(success, "token decimals call failed");
        uint8 decimals = abi.decode(data, (uint8));

        uint32[] memory secondsAgos = new uint32[](2);
        secondsAgos[0] = twapInterval;
        secondsAgos[1] = 0; // till now

        (int56[] memory tickCumulatives, ) = pool.observe(secondsAgos);

        // Get averge tick based on the interval. Price = 1.0001 ** tick
        int24 avgTick = int24((tickCumulatives[1] - tickCumulatives[0]) / int56(uint56(twapInterval)));

        uint160 sqrtPriceX96 = TickMath.getSqrtRatioAtTick(avgTick);

        // sqrtPriceX96 should be divided by 2**96 twice, and multiplied by token decimals to get the actual price
        uint256 partialPrice = FullMath.mulDiv(sqrtPriceX96, sqrtPriceX96, FixedPoint96.Q96);
        price = FullMath.mulDiv(partialPrice, 10 ** decimals, FixedPoint96.Q96);
    }

    /// @inheritdoc IPriceOracle
    function getRecentPriceInETH(address token) public view override returns (uint256) {
        return getTWAPInWETH(token, 3600); // 1 hour
    }
}
