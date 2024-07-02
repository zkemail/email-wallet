// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "accountabstraction/contracts/samples/callback/TokenCallbackHandler.sol";
import {IOauth} from "./interfaces/IOauth.sol";
import {EphemeralTx} from "./interfaces/Types.sol";
import {IWETHWithdraw} from "./interfaces/IWETHWithdraw.sol";
import {TokenRegistry} from "./utils/TokenRegistry.sol";
import {EmailWalletCore} from "./EmailWalletCore.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

/// @title EmailWallet
/// @notice Simple Wallet contract to be used as the EmailWallet for users
/// @notice This wallet can `execute` any function on any contract provided calle is `owner`
/// @notice The deployed is the `owner` by default (EmailWalletCore)
/// @dev External contracts should use `call` to deposit ETH if needed
contract Wallet is TokenCallbackHandler, OwnableUpgradeable, UUPSUpgradeable, IERC20 {
    address immutable weth;
    uint256 public epheTxNonce;
    address public curERC20Target;

    // Oauth core contract
    IOauth immutable oauth;

    /// @notice Fallback function to receive ETH
    /// @notice For convenience, this contract will convert ETH to WETH always
    /// @notice Conversion is not done if the sender is WETH (i.e when user call `weth.withdraw()`)
    fallback() external payable {
        if (msg.sender != weth) {
            _ethToWeth(msg.value);
        }
    }

    /// @notice Function to receive ETH
    /// @notice For convenience, this contract will convert ETH to WETH always
    /// @notice Conversion is not done if the sender is WETH (i.e when user call `weth.withdraw()`)
    receive() external payable {
        if (msg.sender != weth) {
            _ethToWeth(msg.value);
        }
    }

    /// @notice Modifier to allow only the owner or the contract itself
    modifier ownerOrSelf() {
        require(msg.sender == owner() || msg.sender == address(this), "only owner or self");
        _;
    }

    /// @param wethAddress Address of the WETH contract
    /// @param oauthAddress Address of the Oauth contract
    constructor(address wethAddress, address oauthAddress) {
        weth = wethAddress;
        oauth = IOauth(oauthAddress);
    }

    /// @notice Initialize the contract
    function initialize() public initializer {
        __Ownable_init();
    }

    function getOauth() external view returns (IOauth) {
        return oauth;
    }

    /// @notice Execute a function on an external contract
    /// @param target Address of the contract to call
    /// @param value Amount of ETH to send
    /// @param data Encoded data of the function to call
    function execute(address target, uint256 value, bytes calldata data) external ownerOrSelf {
        _execute(target, value, data);
    }

    /// @notice Execute a transaction from an ephemeral address
    /// @param txData Data of the ephemeral transaction
    function executeEphemeralTx(EphemeralTx calldata txData) external {
        require(txData.walletAddr == address(this), "invalid wallet address");
        require(txData.txNonce == epheTxNonce, "invalid nonce");
        bool isSudo = txData.target == address(this);
        address target = txData.target;
        EmailWalletCore core = EmailWalletCore(payable(owner()));
        oauth.validateEpheAddr(address(this), txData.epheAddr, txData.epheAddrNonce, isSudo);
        oauth.validateSignature(txData.epheAddr, hashEphemeralTx(txData), txData.signature);
        require(
            target != owner() &&
                target != address(core.relayerHandler()) &&
                target != address(core.accountHandler()) &&
                target != address(core.extensionHandler()) &&
                target != address(oauth),
            "invalid target"
        );
        TokenRegistry tokenRegistry = core.tokenRegistry();
        string memory tokenName = tokenRegistry.getTokenNameOfAddress(target);
        if (bytes(tokenName).length > 0) {
            require(txData.tokenAmount > 0, "token amount is 0");
            if(_checkAllowanceReduction(txData.data)) {
                oauth.reduceTokenAllowance(txData.epheAddrNonce, target, txData.tokenAmount);
            }
            curERC20Target = target;
        }
        if (txData.ethValue > 0) {
            oauth.reduceTokenAllowance(txData.epheAddrNonce, weth, txData.ethValue);
            IWETHWithdraw(weth).withdraw(txData.ethValue);
        }
        _execute(txData.target, txData.ethValue, txData.data);

        // Finalization
        curERC20Target = address(0);
        epheTxNonce++;
    }

    /// @notice Convert ETH to WETH
    /// @param amount Amount of ETH to convert
    function _ethToWeth(uint256 amount) internal {
        (bool success, ) = weth.call{value: amount}(abi.encodeWithSignature("deposit()"));
        require(success, "convert to weth failed");
    }

    /// @notice Upgrade the implementation of the proxy
    /// @param newImplementation Address of the new implementation
    function _authorizeUpgrade(address newImplementation) internal override ownerOrSelf {}

    function hashEphemeralTx(EphemeralTx calldata txData) public pure returns (bytes32) {
        EphemeralTx memory txDataTmp = txData;
        txDataTmp.signature = "";
        return keccak256(abi.encode(txDataTmp));
    }

    function _checkAllowanceReduction(bytes calldata data) internal view returns (bool) {
        bool result = false;

        bytes4 functionSelector = bytes4(data[:4]);
        if (functionSelector == bytes4(keccak256("transfer(address,uint256)"))) {
            result = true;
        } else if(functionSelector == bytes4(keccak256("approve(address,uint256)"))) {
            result = true;
        } else if(functionSelector == bytes4(keccak256("transferFrom(address,address,uint256)"))) {
            (address from, , ) = abi.decode(data[4:], (address, address, uint256));
            if(from == address(this)) {
                result = true;
            }
        }
        return result;
    }

    function _execute(address target, uint256 value, bytes calldata data) internal {
        if(curERC20Target == target) {
            // Traget is ERC20 contract.
            // In that case, call the function of this contract itself
            bytes4 functionSelector = bytes4(data[:4]);
            if (functionSelector == bytes4(keccak256("transfer(address,uint256)"))) {
                (address to, uint256 amount) = abi.decode(data[4:], (address, uint256));
                this.transfer(to, amount);
            } else if(functionSelector == bytes4(keccak256("allowance(address,address)"))) {
                (address owner, address spender) = abi.decode(data[4:], (address, address));
                this.allowance(owner, spender);
            } else if(functionSelector == bytes4(keccak256("approve(address,uint256)"))) {
                (address spender, uint256 amount) = abi.decode(data[4:], (address, uint256));
                this.approve(spender, amount);
            } else if(functionSelector == bytes4(keccak256("transferFrom(address,address,uint256)"))) {
                (address from, address to, uint256 amount) = abi.decode(data[4:], (address, address, uint256));
                this.transferFrom(from, to, amount);
            } else {
                revert("invalid ERC20 function selector");
            }
        } else {
            // Target is others
            (bool success, bytes memory result) = target.call{value: value}(data);
            if (!success) {
                assembly {
                    revert(add(result, 32), mload(result))
                }
            }
        }
    }

    /**
     * ERC20 implementation
     */ 

    // Dummy logic
    function totalSupply() external override pure returns (uint256) {
        return 0;  
    }

    // Dummy logic
    function balanceOf(address account) external override pure returns (uint256) {
        return 0;
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        return IERC20(curERC20Target).transfer(to, amount);
    }

    function allowance(address owner, address spender) external override view returns (uint256) {
        return IERC20(curERC20Target).allowance(owner, spender);
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        return IERC20(curERC20Target).approve(spender, amount);
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        return IERC20(curERC20Target).transferFrom(from, to, amount);
    }
}
