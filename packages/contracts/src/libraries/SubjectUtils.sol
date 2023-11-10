// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import "./DecimalUtils.sol";
import "../interfaces/Types.sol";
import "../interfaces/Commands.sol";
import "../utils/TokenRegistry.sol";
import "../handlers/ExtensionHandler.sol";
import "../EmailWalletCore.sol";

library SubjectUtils {
    bytes16 private constant LOWER_HEX_DIGITS = "0123456789abcdef";
    bytes16 private constant UPPER_HEX_DIGITS = "0123456789ABCDEF";

    function addressToChecksumHexString(address addr) internal pure returns (string memory) {
        string memory lowerCaseAddrWithOx = Strings.toHexString(addr);

        bytes memory lowerCaseAddr = new bytes(40); // Remove 0x added by the OZ lib
        for (uint8 i = 2; i < 42; i++) {
            lowerCaseAddr[i - 2] = bytes(lowerCaseAddrWithOx)[i];
        }

        // Hash of lowercase addr
        uint256 lowerCaseHash = uint256(keccak256(abi.encodePacked(lowerCaseAddr)));

        // Result hex = 42 chars with 0x prefix
        bytes memory result = new bytes(42);
        result[0] = "0";
        result[1] = "x";

        // Shift 24 bytes (96 bits) to the right; as we only need first 20 bytes of the hash to compare
        lowerCaseHash >>= 24 * 4;

        uint256 intAddr = uint256(uint160(addr));

        for (uint8 i = 41; i > 1; --i) {
            uint8 hashChar = uint8(lowerCaseHash & 0xf); // Get last char of the hex
            uint8 addrChar = uint8(intAddr & 0xf); // Get last char of the address

            if (hashChar >= 8) {
                result[i] = UPPER_HEX_DIGITS[addrChar];
            } else {
                result[i] = LOWER_HEX_DIGITS[addrChar];
            }

            // Remove last char from both hash and addr
            intAddr >>= 4;
            lowerCaseHash >>= 4;
        }

        return string(result);
    }

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

    /// @notice Calculate the masked subject for an EmailOp from command and other params
    ///         This also do sanity checks of certain parameters used in the subject
    /// @param emailOp EmailOp to compute masked subject for
    /// @param walletAddr Address of the user's wallet
    /// @param core EmailWalletCore contract to read some states for validation
    function computeMaskedSubjectForEmailOp(
        EmailOp memory emailOp,
        address walletAddr,
        EmailWalletCore core
    ) public view returns (string memory maskedSubject, bool isExtension) {
        ExtensionHandler extensionHandler = ExtensionHandler(core.extensionHandler());

        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            ERC20 token = ERC20(TokenRegistry(core.tokenRegistry()).getTokenAddress(emailOp.walletParams.tokenName));

            require(token != ERC20(address(0)), "token not supported");
            require(emailOp.walletParams.amount > 0, "send amount should be >0");
            require(token.balanceOf(walletAddr) >= walletParams.amount, "insufficient balance");

            maskedSubject = string.concat(
                Commands.SEND,
                " ",
                DecimalUtils.uintToDecimalString(walletParams.amount, token.decimals()),
                " ",
                walletParams.tokenName,
                " to "
            );

            if (emailOp.recipientETHAddr != address(0)) {
                maskedSubject = string.concat(maskedSubject, addressToChecksumHexString(emailOp.recipientETHAddr));
            }
        }
        // Sample: Execute 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.EXECUTE)) {
            require(emailOp.executeCallData.length > 0, "executeCallData cannot be empty");

            (address target, , bytes memory data) = abi.decode(emailOp.executeCallData, (address, uint256, bytes));

            require(target != address(0), "invalid execute target");
            require(Address.isContract(target), "target is not a contract");

            require(
                target != address(core) &&
                    target != address(core.unclaimsHandler()) &&
                    target != address(core.accountHandler()) &&
                    target != address(core.relayerHandler()) &&
                    target != address(core.extensionHandler()),
                "cannot execute on core or handlers"
            );

            require(target != walletAddr, "cannot execute on wallet");
            require(
                bytes(TokenRegistry(core.tokenRegistry()).getTokenNameOfAddress(target)).length == 0,
                "cannot execute on token"
            );
            require(data.length > 0, "execute data cannot be empty");

            maskedSubject = string.concat(Commands.EXECUTE, " 0x", bytesToHexString(emailOp.executeCallData));
        }
        // Sample: Install extension Uniswap
        else if (Strings.equal(emailOp.command, Commands.INSTALL_EXTENSION)) {
            address extAddr = extensionHandler.addressOfExtensionName(emailOp.extensionName);
            require(extAddr != address(0), "extension not registered");

            maskedSubject = string.concat(Commands.INSTALL_EXTENSION, " extension ", emailOp.extensionName);
        }
        // Sample: Remove extension Uniswap
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            address extAddr = extensionHandler.addressOfExtensionName(emailOp.extensionName);
            string memory command = extensionHandler.subjectTemplatesOfExtension(extAddr, 0, 0);

            require(extAddr != address(0), "extension not registered");
            require(
                extensionHandler.userExtensionOfCommand(walletAddr, command) != address(0),
                "extension not installed"
            );

            maskedSubject = string.concat(Commands.UNINSTALL_EXTENSION, " extension ", emailOp.extensionName);
        }
        // Sample: Exit email wallet. Change owner to 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.EXIT_EMAIL_WALLET)) {
            require(emailOp.newWalletOwner != address(0), "newWalletOwner cannot be empty");

            maskedSubject = string.concat(
                Commands.EXIT_EMAIL_WALLET,
                " Email Wallet. Change ownership to ",
                addressToChecksumHexString(emailOp.newWalletOwner)
            );
        }
        // Sample: DKIM registry as 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.DKIM)) {
            require(emailOp.newDkimRegistry != address(0), "newDkimRegistry cannot be empty");

            maskedSubject = string.concat(
                Commands.DKIM,
                " registry set to ",
                addressToChecksumHexString(emailOp.newDkimRegistry)
            );
        }
        // The command is for an extension
        else {
            isExtension = true;

            address extensionAddr = extensionHandler.getExtensionForCommand(walletAddr, emailOp.command);
            require(extensionAddr != address(0), "invalid command or extension");

            string[] memory subjectTemplate = extensionHandler.getSubjectTemplatesOfExtension(extensionAddr)[
                emailOp.extensionParams.subjectTemplateIndex
            ];

            uint8 nextParamIndex;
            for (uint8 i = 0; i < subjectTemplate.length; i++) {
                string memory matcher = subjectTemplate[i];
                string memory value;

                // {tokenAmount} is combination of tokenName and amount, encoded as (uint256,string). Eg: `30.23 DAI`
                if (Strings.equal(matcher, Commands.TOKEN_AMOUNT_TEMPLATE)) {
                    (uint256 amount, string memory tokenName) = abi.decode(
                        emailOp.extensionParams.subjectParams[nextParamIndex],
                        (uint256, string)
                    );
                    address tokenAddr = TokenRegistry(core.tokenRegistry()).getTokenAddress(tokenName);

                    require(tokenAddr != address(0), "token not supported");
                    // We are not validating token balance here, as tokenAmount might not be always for debiting from wallet

                    value = string.concat(
                        DecimalUtils.uintToDecimalString(amount, ERC20(tokenAddr).decimals()),
                        " ",
                        tokenName
                    );
                    nextParamIndex++;
                }
                // {amount} is number in wei format (decimal format in subject)
                else if (Strings.equal(matcher, Commands.AMOUNT_TEMPLATE)) {
                    uint256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (uint256));
                    value = DecimalUtils.uintToDecimalString(num);
                    nextParamIndex++;
                }
                // {string} is plain string
                else if (Strings.equal(matcher, Commands.STRING_TEMPLATE)) {
                    value = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (string));
                    nextParamIndex++;
                }
                // {uint} is number parsed the same way as mentioned in subject (decimals not allowed) - use {amount} for decimals
                else if (Strings.equal(matcher, Commands.UINT_TEMPLATE)) {
                    uint256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (uint256));
                    value = Strings.toString(num);
                    nextParamIndex++;
                }
                // {int} for negative values
                else if (Strings.equal(matcher, Commands.INT_TEMPLATE)) {
                    int256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (int256));
                    value = Strings.toString(num);
                    nextParamIndex++;
                }
                // {addres} for wallet address
                else if (Strings.equal(matcher, Commands.ADDRESS_TEMPLATE)) {
                    address addr = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (address));
                    value = addressToChecksumHexString(addr);
                    nextParamIndex++;
                }
                // {recipient} is either the recipient's ETH address or zero bytes with the same length of the email address
                else if (Strings.equal(matcher, Commands.RECIPIENT_TEMPLATE)) {
                    if (!emailOp.hasEmailRecipient) {
                        value = addressToChecksumHexString(emailOp.recipientETHAddr);
                    } else {
                        bytes memory zeros = new bytes(emailOp.numRecipientEmailAddrBytes);
                        value = string(zeros);
                    }
                }
                // Constant string otherwise
                else {
                    value = matcher;
                }

                if (bytes(maskedSubject).length == 0) {
                    maskedSubject = value;
                } else {
                    maskedSubject = string.concat(maskedSubject, " ", value);
                }
            }

            // We should have used all items in subjectParams by now
            require(nextParamIndex == emailOp.extensionParams.subjectParams.length, "invalid subject params length");
        }
    }
}
