// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import '@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol';
import '@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';

contract PoolFinder {
    IUniswapV3Factory public immutable factory;

    constructor(IUniswapV3Factory _factory) {
        factory = _factory;
    }

    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external view returns (IUniswapV3Pool) {
        address poolAddress = factory.getPool(tokenA, tokenB, fee);
        require(poolAddress != address(0), "Pool not found");
        return IUniswapV3Pool(poolAddress);
    }
}