// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";
import "../interfaces/Extension.sol";
import "../Wallet.sol";
import "./RelayerHandler.sol";
import "../interfaces/IVerifier.sol";
import "./AccountHandler.sol";

contract UnclaimsHandler is ReentrancyGuard, Ownable {
    // Verifier contract
    IVerifier public immutable verifier;

    // AccountHandler contract
    AccountHandler public immutable accountHandler;

    // RelayerHandler contract
    RelayerHandler public immutable relayerHandler;

    // Gas required to claim unclaimed funds. User (their relayer) who register unclaimed funds
    // need to lock this amount which is relesed to the relayer who claims it
    uint256 public immutable unclaimedFundClaimGas;

    // Gas required to claim unclaimed state
    uint256 public immutable unclaimedStateClaimGas;

    // Default expiry duration for unclaimed funds and states
    uint256 public immutable unclaimsExpiryDuration;

    // Max fee per gas in wei that relayer can set in a UserOp
    uint256 public immutable maxFeePerGas;

    // Mapping of recipient's emailAddrCommit (hash(email, randomness)) to the unclaimedFund
    mapping(bytes32 => UnclaimedFund) public unclaimedFundOfEmailAddrCommit;

    // Mapping of emailAddrCommit to unclaimed state
    mapping(bytes32 => UnclaimedState) public unclaimedStateOfEmailAddrCommit;

    constructor(
        address _relayerHandler,
        address _accountHandler,
        address _verifier,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas,
        uint256 _unclaimsExpiryDuration,
        uint256 _maxFeePerGas
    ) {
        relayerHandler = RelayerHandler(_relayerHandler);
        accountHandler = AccountHandler(_accountHandler);
        verifier = IVerifier(_verifier);
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;
        unclaimsExpiryDuration = _unclaimsExpiryDuration;
        maxFeePerGas = _maxFeePerGas;
    }

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
    ) public onlyOwner {
        uint256 expiryTime = block.timestamp + unclaimsExpiryDuration;

        UnclaimedFund memory fund = UnclaimedFund({
            emailAddrCommit: emailAddrCommit,
            tokenAddr: tokenAddr,
            amount: amount,
            expiryTime: expiryTime,
            sender: sender
        });

        unclaimedFundOfEmailAddrCommit[emailAddrCommit] = fund;

        emit EmailWalletEvents.UnclaimedFundRegistered(emailAddrCommit, tokenAddr, amount, sender, expiryTime, 0, "");
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
    ) public payable {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming / expiring the unclaimed fee
        require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "invalid unclaimed fund fee");
        require(amount > 0, "amount should be greater than 0");
        require(tokenAddr != address(0), "invalid token contract");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedFundOfEmailAddrCommit[emailAddrCommit].amount == 0, "unclaimed fund exists");

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        ERC20(tokenAddr).transferFrom(msg.sender, address(this), amount);

        UnclaimedFund memory fund = UnclaimedFund({
            emailAddrCommit: emailAddrCommit,
            tokenAddr: tokenAddr,
            amount: amount,
            expiryTime: expiryTime,
            sender: msg.sender
        });

        unclaimedFundOfEmailAddrCommit[emailAddrCommit] = fund;

        emit EmailWalletEvents.UnclaimedFundRegistered(
            emailAddrCommit,
            tokenAddr,
            amount,
            msg.sender,
            expiryTime,
            announceCommitRandomness,
            announceEmailAddr
        );
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    /// @dev Relayer should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function claimUnclaimedFund(
        bytes32 emailAddrCommit,
        bytes32 recipientEmailAddrPointer,
        bytes calldata proof
    ) public nonReentrant {
        UnclaimedFund memory fund = unclaimedFundOfEmailAddrCommit[emailAddrCommit];
        bytes32 accountKeyCommit = accountHandler.accountKeyCommitOfPointer(recipientEmailAddrPointer);

        require(relayerHandler.getRandHash(msg.sender) != bytes32(0), "caller not relayer");
        require(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).relayer == msg.sender,
            "invalid relayer for account"
        );
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(
            accountHandler.accountKeyCommitOfPointer(recipientEmailAddrPointer) != bytes32(0),
            "invalid account key commit."
        );
        require(accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).initialized, "account not initialized");
        require(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).walletSalt != bytes32(0),
            "invalid wallet salt"
        );

        require(
            verifier.verifyClaimFundProof(
                relayerHandler.getRandHash(msg.sender),
                recipientEmailAddrPointer,
                emailAddrCommit,
                proof
            ),
            "invalid proof"
        );

        address recipientAddr = accountHandler.getWalletOfSalt(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).walletSalt
        );

        delete unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddr).transfer(recipientAddr, fund.amount);

        // Transfer claim fee to the sender (relayer)
        payable(msg.sender).transfer(unclaimedFundClaimGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedFundClaimed(emailAddrCommit, fund.tokenAddr, fund.amount, recipientAddr);
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @dev Callee should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function voidUnclaimedFund(bytes32 emailAddrCommit) public nonReentrant {
        uint256 initialGas = gasleft();

        UnclaimedFund memory fund = unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        // Transfer token from Core contract to sender's wallet
        ERC20(fund.tokenAddr).transfer(fund.sender, fund.amount);

        // Gas consumed so far + approx. cost for 2 ETH transfers; Ignoring event emission gas
        uint256 consumedGas = initialGas - gasleft() + 21000 + 21000;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        payable(fund.sender).transfer((unclaimedFundClaimGas - consumedGas) * maxFeePerGas);
        payable(msg.sender).transfer(consumedGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedFundVoided(emailAddrCommit, fund.tokenAddr, fund.amount, fund.sender);
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
    ) public payable {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "invalid unclaimed state fee");

        require(state.length > 0, "state cannot be empty");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedStateOfEmailAddrCommit[emailAddrCommit].sender == address(0), "unclaimed state exists");

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommit: emailAddrCommit,
            extensionAddr: extensionAddr,
            sender: msg.sender,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        try extension.registerUnclaimedState(us, false) {} catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        unclaimedStateOfEmailAddrCommit[emailAddrCommit] = us;

        emit EmailWalletEvents.UnclaimedStateRegistered(
            emailAddrCommit,
            extensionAddr,
            msg.sender,
            expiryTime,
            state,
            announceCommitRandomness,
            announceEmailAddr
        );
    }

    /// Register unclaimed state from an extension
    /// @param extensionAddr Address of the extension contract to which the state is registered
    /// @param sender Address of the sender of the unclaimed state
    /// @param recipientEmailAddrCommit Email address commitment of the recipient
    /// @param state State to be registered
    function registerUnclaimedStateInternal(
        address extensionAddr,
        address sender,
        bytes32 recipientEmailAddrCommit,
        bytes calldata state
    ) public onlyOwner {
        require(
            unclaimedStateOfEmailAddrCommit[recipientEmailAddrCommit].sender == address(0),
            "unclaimed state exists"
        );
        require(state.length > 0, "state cannot be empty");

        uint256 expiryTime = block.timestamp + unclaimsExpiryDuration;

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommit: recipientEmailAddrCommit,
            extensionAddr: extensionAddr,
            sender: sender,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        try extension.registerUnclaimedState(us, false) {} catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        unclaimedStateOfEmailAddrCommit[recipientEmailAddrCommit] = us;

        emit EmailWalletEvents.UnclaimedStateRegistered(
            recipientEmailAddrCommit,
            extensionAddr,
            sender,
            expiryTime,
            state,
            0,
            ""
        );
    }

    /// Claim unclaimed state to the recipient's (initialized) wallet.
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedState(
        bytes32 emailAddrCommit,
        bytes32 recipientEmailAddrPointer,
        bytes calldata proof
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        UnclaimedState memory us = unclaimedStateOfEmailAddrCommit[emailAddrCommit];
        bytes32 accountKeyCommit = accountHandler.accountKeyCommitOfPointer(recipientEmailAddrPointer);

        require(relayerHandler.getRandHash(msg.sender) != bytes32(0), "caller not relayer");
        require(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).relayer == msg.sender,
            "invalid relayer for account"
        );
        require(us.sender != address(0), "unclaimed state not registered");
        require(us.extensionAddr != address(0), "invalid extension address");
        require(us.expiryTime > block.timestamp, "unclaimed state expired");
        require(accountKeyCommit != bytes32(0), "invalid account key commit.");
        require(accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).initialized, "account not initialized");
        require(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).walletSalt != bytes32(0),
            "invalid wallet salt"
        );

        require(
            verifier.verifyClaimFundProof(
                relayerHandler.getRandHash(msg.sender),
                recipientEmailAddrPointer,
                emailAddrCommit,
                proof
            ),
            "invalid proof"
        );

        address recipientAddr = accountHandler.getWalletOfSalt(
            accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).walletSalt
        );

        delete unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        Extension extension = Extension(us.extensionAddr);

        // Deducated consumed gas + 21k for eth transer from `unclaimedStateClaimGas` and pass to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - 21000;

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

        emit EmailWalletEvents.UnclaimedStateClaimed(emailAddrCommit, recipientAddr);
    }

    /// @notice Return unclaimed state after expiry time
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed state was registered.
    function voidUnclaimedState(
        bytes32 emailAddrCommit
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        UnclaimedState memory us = unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        require(us.sender != address(0), "unclaimed state not registered");
        require(us.expiryTime < block.timestamp, "unclaimed state not expired");

        delete unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        Extension extension = Extension(us.extensionAddr);

        // Gas consumed for verification and next steps is deducated from `unclaimedStateClaimGas`
        // and rest is passed to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - 21000 - 21000;

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
        uint256 consumedGas = initialGas - gasleft() + 21000 + 21000;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        payable(us.sender).transfer((unclaimedStateClaimGas - consumedGas) * maxFeePerGas);
        payable(msg.sender).transfer(consumedGas * maxFeePerGas);

        emit EmailWalletEvents.UnclaimedStateVoided(emailAddrCommit, us.sender);
    }

    function getSenderOfUnclaimedFund(bytes32 emailAddrCommit) public view returns (address) {
        return unclaimedFundOfEmailAddrCommit[emailAddrCommit].sender;
    }

    function getSenderOfUnclaimedState(bytes32 emailAddrCommit) public view returns (address) {
        return unclaimedStateOfEmailAddrCommit[emailAddrCommit].sender;
    }
}
