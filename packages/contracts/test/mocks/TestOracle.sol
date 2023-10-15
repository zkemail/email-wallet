// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IPriceOracle} from "../../src/interfaces/IPriceOracle.sol";
import "../../src/interfaces/Types.sol";

contract TestOracle is IPriceOracle {
    function getRecentPriceInETH(address tokenAddr) external view returns (uint256) {
        string memory name = ERC20(tokenAddr).name();

        if (Strings.equal(name, "DAI") || Strings.equal(name, "USDC")) {
            return 1500 ether;
        }

        return 1 ether;
    }
}
