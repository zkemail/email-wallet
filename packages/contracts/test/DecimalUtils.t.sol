// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../src/libraries/DecimalUtils.sol";

contract DecimalUtilsTest is Test {
    function _checkDecimalString(uint256 value, string memory expected) internal {
        _checkDecimalString(value, expected, 18);
    }

    function _checkDecimalString(uint256 value, string memory expected, uint decimal) internal {
        string memory actual = DecimalUtils.uintToDecimalString(value, decimal);
        assertTrue(Strings.equal(actual, expected), string.concat(expected, "!=", actual));
    }

    function test_UintToDecimalString_Default() public {
        _checkDecimalString(1 ether, "1");
        _checkDecimalString(10 ether, "10");
        _checkDecimalString(200 ether, "200");
        _checkDecimalString(10.21 ether, "10.21");
        _checkDecimalString(3.1 ether, "3.1");
        _checkDecimalString(2.01 ether, "2.01");
        _checkDecimalString(0.1 ether, "0.1");
        _checkDecimalString(0.01 ether, "0.01");
        _checkDecimalString(4.2001 ether, "4.2001");
        _checkDecimalString(3.1004001 ether, "3.1004001");
        _checkDecimalString(22 ether, "22");
        _checkDecimalString(2.0071 ether, "2.0071");
        _checkDecimalString(3.0000081 ether, "3.0000081");
        _checkDecimalString(0.002 ether, "0.002");
        _checkDecimalString(0.14 ether, "0.14");
        _checkDecimalString(0.014 ether, "0.014");
        _checkDecimalString(0.0130021 ether, "0.0130021");
        _checkDecimalString(0.000000002 ether, "0.000000002");
        _checkDecimalString(0.000000000000000001 ether, "0.000000000000000001");
        _checkDecimalString(1.000000000000000001 ether, "1.000000000000000001");

        // Non-significant trailing zeros are not supported
        _checkDecimalString(21.0 ether, "21");
        _checkDecimalString(21.00 ether, "21");
        _checkDecimalString(3.490 ether, "3.49");
    }

    function test_UintToDecimalString_NotDefault() public {
        _checkDecimalString(1 * (10 ** 6), "1", 6);
        _checkDecimalString(10 * (10 ** 6), "10", 6);
        _checkDecimalString(200 * (10 ** 6), "200", 6);
        _checkDecimalString(10.21 * (10 ** 6), "10.21", 6);
        _checkDecimalString(3.1 * (10 ** 6), "3.1", 6);
        _checkDecimalString(2.01 * (10 ** 6), "2.01", 6);
        _checkDecimalString(0.1 * (10 ** 6), "0.1", 6);
        _checkDecimalString(0.01 * (10 ** 6), "0.01", 6);
        _checkDecimalString(4.2001 * (10 ** 6), "4.2001", 6);
        _checkDecimalString(22 * (10 ** 6), "22", 6);
        _checkDecimalString(2.0071 * (10 ** 6), "2.0071", 6);
        _checkDecimalString(3.000081 * (10 ** 6), "3.000081", 6);
        _checkDecimalString(0.002 * (10 ** 6), "0.002", 6);
        _checkDecimalString(0.14 * (10 ** 6), "0.14", 6);
        _checkDecimalString(0.014 * (10 ** 6), "0.014", 6);
        _checkDecimalString(0.013001 * (10 ** 6), "0.013001", 6);

        // Non-significant trailing zeros are not supported
        _checkDecimalString(21.0 * (10 ** 6), "21", 6);
        _checkDecimalString(21.00 * (10 ** 6), "21", 6);
        _checkDecimalString(3.490 * (10 ** 6), "3.49", 6);
    }
}
