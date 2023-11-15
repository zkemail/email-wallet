// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";

contract PoolFinder {
    IUniswapV3Factory public immutable factory;

    constructor(IUniswapV3Factory _factory) {
        factory = _factory;
    }

    function getPoolSlot0(
        address tokenA,
        address tokenB,
        uint24 fee
    )
        external
        view
        returns (
            uint160 sqrtPriceX96,
            int24 tick,
            uint16 observationIndex,
            uint16 observationCardinality,
            uint16 observationCardinalityNext,
            uint8 feeProtocol,
            bool unlocked
        )
    {
        address poolAddress = factory.getPool(tokenA, tokenB, fee);
        require(poolAddress != address(0), "Pool not found");
        return IUniswapV3Pool(poolAddress).slot0();
    }

    function isPoolExists(address tokenA, address tokenB, uint24 fee) external view returns (bool) {
        return factory.getPool(tokenA, tokenB, fee) != address(0);
    }
}
