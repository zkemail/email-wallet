// SPDX-License-Identifier: MIT
// original: https://github.com/zkemail/email-wallet-sdk/blob/main/src/helpers/StringUtils.sol
pragma solidity ^0.8.13;

import "solidity-stringutils/src/strings.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

library StringUtils {
    using strings for *;

    function splitString(
        string memory str,
        string memory delimiter
    ) public pure returns (string[] memory) {
        strings.slice memory strSlice = str.toSlice();
        strings.slice memory delimiterSlice = delimiter.toSlice();
        uint size = strSlice.count(delimiterSlice) + 1;
        string[] memory result = new string[](size);
        for (uint i = 0; i < size; i++) {
            result[i] = strSlice.split(delimiterSlice).toString();
        }
        return result;
    }

    function hexToBytes32(
        string calldata hexStr
    ) public pure returns (bytes32 result) {
        require(
            hexStr.toSlice().startsWith("0x".toSlice()),
            "invalid hex prefix"
        );
        hexStr = hexStr[2:];
        require(bytes(hexStr).length == 64, "invalid hex string length");
        uint256[] memory ints = hex2Ints(hexStr);
        uint256 sum = 0;
        for (uint256 i = 0; i < 32; i++) {
            sum = (256 * sum + ints[i]);
        }
        return bytes32(sum);
    }

    function hex2Ints(
        string memory hexStr
    ) private pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](bytes(hexStr).length / 2);
        for (uint256 i = 0; i < result.length; i++) {
            result[i] =
                16 *
                hexChar2Int(bytes(hexStr)[2 * i]) +
                hexChar2Int(bytes(hexStr)[2 * i + 1]);
        }
        return result;
    }

    function hexChar2Int(bytes1 char) private pure returns (uint256) {
        uint8 charInt = uint8(char);
        if (charInt >= 48 && charInt <= 57) {
            return charInt - 48;
        } else if (charInt >= 97 && charInt <= 102) {
            return charInt - 87;
        } else {
            require(false, "invalid hex char");
        }
        return 0;
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` decimal representation.
     */
    function toString(uint256 value) internal pure returns (string memory) {
        return Strings.toString(value);
    }

    /**
     * @dev Converts a `int256` to its ASCII `string` decimal representation.
     */
    function toString(int256 value) internal pure returns (string memory) {
        return Strings.toString(value);
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation.
     */
    function toHexString(uint256 value) internal pure returns (string memory) {
        return Strings.toHexString(value);
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
     */
    function toHexString(
        uint256 value,
        uint256 length
    ) internal pure returns (string memory) {
        return Strings.toHexString(value, length);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its not checksummed ASCII `string` hexadecimal representation.
     */
    function toHexString(address addr) internal pure returns (string memory) {
        return Strings.toHexString(addr);
    }

    /**
     * @dev Returns true if the two strings are equal.
     */
    function equal(
        string memory a,
        string memory b
    ) internal pure returns (bool) {
        return Strings.equal(a, b);
    }
}