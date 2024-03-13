// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";
import "../interfaces/Extension.sol";
import "../Wallet.sol";
import "./RelayerHandler.sol";
import "../interfaces/IVerifier.sol";
import "./AccountHandler.sol";

contract UnclaimsHandler is ReentrancyGuard, Initializable, UUPSUpgradeable, OwnableUpgradeable {
    using SafeERC20 for IERC20;

    // Deployer
    address private deployer;

    // Verifier contract
    IVerifier public verifier;

    // AccountHandler contract
    AccountHandler public accountHandler;

    // RelayerHandler contract
    RelayerHandler public relayerHandler;

    // Gas required to claim unclaimed funds. User (their relayer) who register unclaimed funds
    // need to lock this amount which is relesed to the relayer who claims it
    uint256 public unclaimedFundClaimGas;

    // Gas required to claim unclaimed state
    uint256 public unclaimedStateClaimGas;

    // Default expiry duration for unclaimed funds and states
    uint256 public unclaimsExpiryDuration;

    // Max fee per gas in wei that relayer can set in a UserOp
    uint256 public maxFeePerGas;

    /// The total number of registered unclaimed funds.
    uint256 public numUnclaimedFunds;

    /// The total number of registered unclaimed states.
    uint256 public numUnclaimedStates;

    // Mapping of id to the unclaimed fund
    mapping(uint256 => UnclaimedFund) public unclaimedFundOfId;

    // Mapping of id to unclaimed state
    mapping(uint256 => UnclaimedState) public unclaimedStateOfId;

    uint256 constant ETH_TRANSFER_GAS = 21000;
    uint256 constant WETH_DEPOSIT_GAS = 27938;

    modifier onlyDeployer() {
        require(msg.sender == deployer, "caller is not a deployer");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _relayerHandler,
        address _accountHandler,
        address _verifier,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas,
        uint256 _unclaimsExpiryDuration,
        uint256 _maxFeePerGas
    ) public initializer {
        __Ownable_init();
        deployer = _msgSender();
        relayerHandler = RelayerHandler(_relayerHandler);
        accountHandler = AccountHandler(_accountHandler);
        verifier = IVerifier(_verifier);
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;
        unclaimsExpiryDuration = _unclaimsExpiryDuration;
        maxFeePerGas = _maxFeePerGas;
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyDeployer {}

    // UnclaimHandler can receive ETH only from the core contract = owner
    receive() external payable {
        require(msg.sender == owner(), "only owner can send ETH");
    }

    /// @notice Register unclaimed fund for the recipient - can be called by Core contract directly
    /// @param emailAddrCommit Hash of the recipient's email address and a random number.
    /// @param tokenAddr Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    function registerUnclaimedFundInternal(
        address sender,
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount
    ) public onlyOwner returns (uint256) {
        require(unclaimedFundOfId[numUnclaimedFunds].amount == 0, "unclaimed fund exists");
        uint256 expiryTime = block.timestamp + unclaimsExpiryDuration;

        UnclaimedFund memory fund = UnclaimedFund({
            id: numUnclaimedFunds,
            emailAddrCommit: emailAddrCommit,
            tokenAddr: tokenAddr,
            amount: amount,
            expiryTime: expiryTime,
            sender: sender
        });

        unclaimedFundOfId[fund.id] = fund;
        numUnclaimedFunds++;
        emit EmailWalletEvents.UnclaimedFundRegistered(
            fund.id,
            emailAddrCommit,
            tokenAddr,
            amount,
            sender,
            expiryTime,
            0,
            ""
        );
        return fund.id;
    }

    /// @notice Register unclaimed fund for the recipient - for external users to deposit tokens to an email address.
    /// @param emailAddrCommit Hash of the recipient's email address and a random number.
    /// @param tokenAddr Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param expiryTime Expiry time to claim the unclaimed fund. Set `0` to use default expiry.
    /// @param announceCommitRandomness Randomness used to generate the `emailAddrCommit` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    /// @dev   `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @dev   `announceCommitRandomness` and `announceEmailAddr` are optional. They are not validated as well.
    function registerUnclaimedFund(
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitRandomness,
        string calldata announceEmailAddr
    ) public payable returns (uint256) {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming / expiring the unclaimed fee
        require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "invalid unclaimed fund fee");
        require(amount > 0, "amount should be greater than 0");
        require(tokenAddr != address(0), "invalid token contract");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedFundOfId[numUnclaimedFunds].amount == 0, "unclaimed fund exists");

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        IERC20(tokenAddr).safeTransferFrom(msg.sender, address(this), amount);

        UnclaimedFund memory fund = UnclaimedFund({
            id: numUnclaimedFunds,
            emailAddrCommit: emailAddrCommit,
            tokenAddr: tokenAddr,
            amount: amount,
            expiryTime: expiryTime,
            sender: msg.sender
        });

        unclaimedFundOfId[fund.id] = fund;
        numUnclaimedFunds++;
        emit EmailWalletEvents.UnclaimedFundRegistered(
            fund.id,
            emailAddrCommit,
            tokenAddr,
            amount,
            msg.sender,
            expiryTime,
            announceCommitRandomness,
            announceEmailAddr
        );
        return fund.id;
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param id The id of the unclaimed fund to claim.
    /// @param recipientWalletSalt The recipient's wallet salt.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    /// @dev Relayer should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function claimUnclaimedFund(uint256 id, bytes32 recipientWalletSalt, bytes calldata proof) public nonReentrant {
        UnclaimedFund memory fund = unclaimedFundOfId[id];

        require(id < numUnclaimedFunds, "invalid id");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(recipientWalletSalt != bytes32(0), "invalid wallet salt");

        (string memory relayerEmailAddr,) = relayerHandler.relayers(msg.sender);
        require(bytes(relayerEmailAddr).length != 0, "caller is not a relayer");

        require(verifier.verifyClaimFundProof(fund.emailAddrCommit, recipientWalletSalt, proof), "invalid proof");

        address recipientAddr = accountHandler.getWalletOfSalt(recipientWalletSalt);
        delete unclaimedFundOfId[id];

        // Transfer token from Core contract to recipient's wallet
        IERC20(fund.tokenAddr).safeTransfer(recipientAddr, fund.amount);

        // Transfer claim fee to the sender (relayer)
        payable(msg.sender).transfer(unclaimedFundClaimGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedFundClaimed(
            id,
            fund.emailAddrCommit,
            fund.tokenAddr,
            fund.amount,
            recipientAddr
        );
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param id The id of the unclaimed fund to void.
    /// @dev Callee should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function voidUnclaimedFund(uint256 id) public nonReentrant {
        uint256 initialGas = gasleft();

        UnclaimedFund memory fund = unclaimedFundOfId[id];

        require(id < numUnclaimedFunds, "invalid id");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfId[id];

        // Transfer token from Core contract to sender's wallet
        IERC20(fund.tokenAddr).safeTransfer(fund.sender, fund.amount);

        // Gas consumed so far + approx. cost for 1 ETH transfers and 1 deposit to WETH; Ignoring event emission gas
        uint256 consumedGas = initialGas - gasleft() + ETH_TRANSFER_GAS + WETH_DEPOSIT_GAS;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        (bool success, ) = payable(fund.sender).call{value: (unclaimedFundClaimGas - consumedGas) * maxFeePerGas}("");
        require(success, "ETH transfer to fund.sender failed");
        payable(msg.sender).transfer(consumedGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedFundVoided(id, fund.emailAddrCommit, fund.tokenAddr, fund.amount, fund.sender);
    }

    /// Register unclaimed state of an extension for the recipient email address commitment
    /// @param emailAddrCommit Email address commitment of the recipient
    /// @param extensionAddr Address of the extension contract
    /// @param state State to be registered
    /// @param expiryTime Expiry time to claim the unclaimed state.
    /// @param announceCommitRandomness Randomness used to generate the `emailAddrCommit` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    function registerUnclaimedState(
        bytes32 emailAddrCommit,
        address extensionAddr,
        bytes calldata state,
        uint256 expiryTime,
        uint256 announceCommitRandomness,
        string calldata announceEmailAddr
    ) public payable returns (uint256) {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "invalid unclaimed state fee");

        require(state.length > 0, "state cannot be empty");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedStateOfId[numUnclaimedStates].sender == address(0), "unclaimed state exists");

        UnclaimedState memory us = UnclaimedState({
            id: numUnclaimedStates,
            emailAddrCommit: emailAddrCommit,
            extensionAddr: extensionAddr,
            sender: msg.sender,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        unclaimedStateOfId[us.id] = us;
        numUnclaimedStates++;

        try extension.registerUnclaimedState(us, false) {} catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        emit EmailWalletEvents.UnclaimedStateRegistered(
            us.id,
            emailAddrCommit,
            extensionAddr,
            msg.sender,
            expiryTime,
            state,
            announceCommitRandomness,
            announceEmailAddr
        );

        return us.id;
    }

    /// Register unclaimed state from an extension
    /// @param extensionAddr Address of the extension contract to which the state is registered
    /// @param sender Address of the sender of the unclaimed state
    /// @param recipientEmailAddrCommit Email address commitment of the recipient
    /// @param state State to be registered
    /// @param isInternal A flag whether the unclaimed state is registered from `registerUnclaimedStateAsExtension` and the caller and callee extensions are the same.
    function registerUnclaimedStateInternal(
        address extensionAddr,
        address sender,
        bytes32 recipientEmailAddrCommit,
        bytes calldata state,
        bool isInternal
    ) public onlyOwner returns (uint256) {
        require(unclaimedStateOfId[numUnclaimedStates].sender == address(0), "unclaimed state exists");
        require(state.length > 0, "state cannot be empty");

        uint256 expiryTime = block.timestamp + unclaimsExpiryDuration;

        UnclaimedState memory us = UnclaimedState({
            id: numUnclaimedStates,
            emailAddrCommit: recipientEmailAddrCommit,
            extensionAddr: extensionAddr,
            sender: sender,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        unclaimedStateOfId[us.id] = us;
        numUnclaimedStates++;

        try extension.registerUnclaimedState(us, isInternal) {} catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        emit EmailWalletEvents.UnclaimedStateRegistered(
            us.id,
            recipientEmailAddrCommit,
            extensionAddr,
            sender,
            expiryTime,
            state,
            0,
            ""
        );

        return us.id;
    }

    /// Claim unclaimed state to the recipient's (initialized) wallet.
    /// @param id The id of the unclaimed state to claim.
    /// @param recipientWalletSalt The recipient's wallet salt.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedState(
        uint256 id,
        bytes32 recipientWalletSalt,
        bytes calldata proof
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();
        require(id < numUnclaimedStates, "invalid id");

        UnclaimedState memory us = unclaimedStateOfId[id];

        require(us.sender != address(0), "unclaimed state not registered");
        require(us.extensionAddr != address(0), "invalid extension address");
        require(us.expiryTime > block.timestamp, "unclaimed state expired");
        require(recipientWalletSalt != bytes32(0), "invalid wallet salt");

        (string memory relayerEmailAddr,) = relayerHandler.relayers(msg.sender);
        require(bytes(relayerEmailAddr).length != 0, "caller is not a relayer");

        require(verifier.verifyClaimFundProof(us.emailAddrCommit, recipientWalletSalt, proof), "invalid proof");

        address recipientAddr = accountHandler.getWalletOfSalt(recipientWalletSalt);

        Extension extension = Extension(us.extensionAddr);

        delete unclaimedStateOfId[id];

        // Deducated consumed gas + 21k for eth transer from `unclaimedStateClaimGas` + gas to store one value and pass to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - ETH_TRANSFER_GAS;
        require(gasleft() * 63 > gasForExt * 64, "insufficient gas left");

        // Relayer should get claim fee (gas reimbursement) even if extension call fails
        // Simulation wont work, as extension logic will depend on global variables
        try extension.claimUnclaimedState{gas: gasForExt}(us, recipientAddr) {
            success = true;
        } catch Error(string memory reason) {
            success = false;
            returnData = bytes(reason);
        } catch {
            success = false;
        }

        // Transfer claim fee to the sender (relayer)
        payable(msg.sender).transfer(unclaimedStateClaimGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedStateClaimed(id, us.emailAddrCommit, recipientAddr);
    }

    /// @notice Return unclaimed state after expiry time
    /// @param id The id of the unclaimed state to claim.
    function voidUnclaimedState(uint256 id) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();
        require(id < numUnclaimedStates, "invalid id");

        UnclaimedState memory us = unclaimedStateOfId[id];

        require(us.sender != address(0), "unclaimed state not registered");
        require(us.expiryTime < block.timestamp, "unclaimed state not expired");

        Extension extension = Extension(us.extensionAddr);

        delete unclaimedStateOfId[id];

        // Gas consumed for verification and next steps is deducated from `unclaimedStateClaimGas`
        // and rest is passed to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - ETH_TRANSFER_GAS - WETH_DEPOSIT_GAS;
        require(gasleft() * 63 > gasForExt * 64, "insufficient gas left");

        // Callee should get gas reimbursement even if extension call fails
        // Simulation wont work, as extension logic can depend on global variables
        try extension.voidUnclaimedState{gas: gasForExt}(us) {
            success = true;
        } catch Error(string memory reason) {
            success = false;
            returnData = bytes(reason);
        } catch {
            success = false;
        }

        // Gas consumed so far + cost for 2 ETH transfers
        uint256 consumedGas = initialGas - gasleft() + ETH_TRANSFER_GAS + WETH_DEPOSIT_GAS;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        (success, ) = payable(us.sender).call{value: (unclaimedStateClaimGas - consumedGas) * maxFeePerGas}("");
        require(success, "ETH transfer to us.sender failed");
        payable(msg.sender).transfer(consumedGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedStateVoided(id, us.emailAddrCommit, us.sender);
    }

    function getUnclaimedFund(uint256 id) public view returns (UnclaimedFund memory) {
        return unclaimedFundOfId[id];
    }

    function getUnclaimedState(uint256 id) public view returns (UnclaimedState memory) {
        return unclaimedStateOfId[id];
    }
}
