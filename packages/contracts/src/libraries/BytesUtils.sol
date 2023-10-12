// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

/// @title BytesUtils
/// @notice BytesUtils library for converting bytes to hex string
library BytesUtils {
    /// @notice Convert bytes to hex string without 0x prefix
    /// @param data bytes to convert
    function bytesToHexString(bytes memory data) public pure returns (string memory) {
        bytes memory hexChars = "0123456789abcdef";
        bytes memory hexString = new bytes(2 * data.length);

        for (uint256 i = 0; i < data.length; i++) {
            uint256 value = uint256(uint8(data[i]));
            hexString[2 * i] = hexChars[value >> 4];
            hexString[2 * i + 1] = hexChars[value & 0xf];
        }

        return string(hexString);
    }
}
