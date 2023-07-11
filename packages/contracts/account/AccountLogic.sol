// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./AccountStorage.sol";
import "./IAccount.sol";
import "../ICore.sol";
import "../verifier/IVerifier.sol";
import "../extension/IExtension.sol";
import "../utils/Create2.sol";
import "../utils/Constants.sol";

contract AccountLogic is IAccount, AccountStorage, Initializable {
    using Create2 for bytes32;

    modifier onlyCore() {
        require(
            msg.sender == coreAddr,
            "Only the core contract can call this function."
        );
        _;
    }

    modifier onlyRelayer() {
        require(
            msg.sender == getRelayerAddr(),
            "Only the current relayer can call this function."
        );
        _;
    }

    modifier onlySelf() {
        require(
            msg.sender == address(this),
            "Only myself can call this function."
        );
        _;
    }

    function initialize(address _relayer) public initializer {
        require(_relayer != address(0), "_relayer must not be zero address");
        ICore core = ICore(msg.sender);
        require(
            core.isRegisteredAccount(address(this)),
            "Not registered account."
        );
        ICore.AccountData memory accountData = core.getAccountData(
            address(this)
        );
        uint256[] memory initExtensionIds = core.initExtensionIds();
        nonce = 0;
        accountLogicAddr = accountData.initAccountLogic;
        verifierAddr = accountData.initVerifier;
        for (uint idx = 0; idx < initExtensionIds.length; idx++) {
            changeExtension(
                initExtensionIds[idx],
                accountData.initExtensions[idx]
            );
        }
        require(accountData.relayer == _relayer, "_relayer is invalid.");
        coreAddr = msg.sender;
    }

    function getCoreAddr() public view returns (address) {
        return coreAddr;
    }

    function getRelayerAddr() public view returns (address) {
        ICore core = ICore(coreAddr);
        ICore.AccountData memory accountData = core.getAccountData(
            address(this)
        );
        return accountData.relayer;
    }

    function getAccountLogicAddr() public view returns (address) {
        return accountLogicAddr;
    }

    function getVerifierAddr() public view returns (address) {
        return verifierAddr;
    }

    function getExtensionAddr(uint extensionId) public view returns (address) {
        return extensionAddrOfId[extensionId];
    }

    function isUsedEmailNullifier(
        bytes32 emailNullifier
    ) public view returns (bool) {
        return emailNullifiers[emailNullifier];
    }

    function validateUserOp(
        bytes memory verifierParams,
        bytes memory proof,
        uint256 extensionId,
        bytes memory extensionParams
    ) public view {
        ICore core = ICore(coreAddr);
        IVerifier verifier = IVerifier(verifierAddr);
        (
            IVerifier.SenderPublicInput memory senderPublicInput,
            bytes memory remainingVerifierParams
        ) = abi.decode(verifierParams, (IVerifier.SenderPublicInput, bytes));
        (bytes memory senderProof, bytes memory remainingProof) = abi.decode(
            proof,
            (bytes, bytes)
        );
        ICore.AccountData memory fromAccountData = core.getAccountData(
            address(this)
        );
        /// 1. Verify the initialization of `fromAccount`.
        if (nonce == 0) {
            /// [TODO] Verify that the deployment code in `UserOp` is not zero.
            require(
                (fromAccountData.initDepositEmailNullifier == bytes32(0)) ||
                    (fromAccountData.initDepositEmailNullifier ==
                        senderPublicInput.fromEmailNullifier),
                "fromEmailNullifier is invalid."
            );
        }

        /// 2. Verify the sender proof.
        require(
            verifier.verifySenderProof(senderPublicInput, senderProof),
            "senderProof is invalid."
        );
        require(
            senderPublicInput.fromRelayerHash == fromAccountData.relayerHash,
            "fromRelayerHash is invalid."
        );
        require(
            senderPublicInput.fromSalt == fromAccountData.salt,
            "fromSalt is invalid."
        );
        require(
            !emailNullifiers[senderPublicInput.fromEmailNullifier],
            "fromEmailNullifier is already used."
        );
        require(
            keccak256(senderPublicInput.pubKey) ==
                keccak256(fromAccountData.pubKey),
            "pubKey is invalid."
        );
        /// [TODO] Verify `paymaster==relayer`.

        /// 3. Check the subject.
        address extensionAddr = extensionAddrOfId[extensionId];
        require(extensionAddr != address(0), "extensionId is not registered.");
        IExtension extension = IExtension(extensionAddr);
        string memory subjectExpected = string.concat(
            SUBJECT_PREFIX,
            extension.commandName(),
            ": ",
            extension.buildSubject(extensionParams)
        );
        require(
            keccak256(bytes(senderPublicInput.maskedSubjectStr)) ==
                keccak256(bytes(subjectExpected)),
            "The subject is not equal to the expected one."
        );

        /// 4. If nonce==0, verify `psiProof`.
        if (nonce == 0) {
            (
                IVerifier.PsiPublicInput memory psiPublicInput,
                bytes memory _remainingVerifierParams
            ) = abi.decode(
                    remainingVerifierParams,
                    (IVerifier.PsiPublicInput, bytes)
                );
            remainingVerifierParams = _remainingVerifierParams;
            (bytes memory psiProof, bytes memory _remainingProof) = abi.decode(
                remainingProof,
                (bytes, bytes)
            );
            remainingProof = _remainingProof;
            require(
                verifier.verifyPsiProof(psiPublicInput, psiProof),
                "psiProof is invalid."
            );
            require(
                senderPublicInput.fromAddrCommit == psiPublicInput.addrCommit,
                "addrCommit is invalid."
            );
            require(
                psiPublicInput.relayerHash == fromAccountData.relayerHash,
                "relayerHash is invalid."
            );
        }

        /// 5. Verify that the same `subjectAddrCommit` is used in the recipient proof if necessary.
        if (!senderPublicInput.isSubjectAddrNull) {
            (IVerifier.RecipientPublicInput memory recipientPublicInput, ) = abi
                .decode(
                    remainingVerifierParams,
                    (IVerifier.RecipientPublicInput, bytes)
                );
            (bytes memory recipientProof, ) = abi.decode(
                remainingProof,
                (bytes, bytes)
            );
            require(
                senderPublicInput.subjectAddrCommit ==
                    recipientPublicInput.subjectAddrCommit,
                "subjectAddrCommit is invalid."
            );
        }
    }

    function processRecipientProof(
        IVerifier.RecipientPublicInput memory recipientPublicInput,
        bytes memory recipientProof,
        bytes32 subjectEmailNullifier
    ) public {
        ICore core = ICore(coreAddr);
        require(
            core.isRegisteredAccount(msg.sender),
            "Not registered account."
        );
        IVerifier verifier = IVerifier(verifierAddr);
        require(
            verifier.verifyRecipientProof(recipientPublicInput, recipientProof),
            "recipientProof is invalid."
        );
        /// [TODO] The following process does not work when this account is registered on-chain but not deployed.
        /// It requires the core contract to store the subjectEmailNullifier of the non-deployed accounts.
        ICore.AccountData memory subjectAccountData = core.getAccountData(
            address(this)
        );
        require(
            recipientPublicInput.subjectSalt == subjectAccountData.salt,
            "subjectSalt is invalid."
        );
        require(
            recipientPublicInput.subjectRelayerHash ==
                subjectAccountData.relayerHash,
            "subjectRelayerHash is invalid."
        );
        require(
            !emailNullifiers[subjectEmailNullifier],
            "subjectEmailNullifier is already used."
        );
        emailNullifiers[subjectEmailNullifier] = true;
    }

    function executeUserOp(
        bytes memory verifierParams,
        bytes memory proof,
        uint256 extensionId,
        bytes memory extensionParams
    ) public {
        ICore core = ICore(coreAddr);
        IVerifier verifier = IVerifier(verifierAddr);
        (
            IVerifier.SenderPublicInput memory senderPublicInput,
            bytes memory remainingVerifierParams
        ) = abi.decode(verifierParams, (IVerifier.SenderPublicInput, bytes));
        (, bytes memory remainingProof) = abi.decode(proof, (bytes, bytes));
        ICore.AccountData memory fromAccountData = core.getAccountData(
            address(this)
        );
        /// 1. Store fromEmailNullifier.
        emailNullifiers[senderPublicInput.fromEmailNullifier] = true;

        /// 2. If nonce==0, store `psiPoint`.
        if (nonce == 0) {
            (
                IVerifier.PsiPublicInput memory psiPublicInput,
                bytes memory _remainingVerifierParams
            ) = abi.decode(
                    remainingVerifierParams,
                    (IVerifier.PsiPublicInput, bytes)
                );
            remainingVerifierParams = _remainingVerifierParams;
            (bytes memory psiProof, bytes memory _remainingProof) = abi.decode(
                remainingProof,
                (bytes, bytes)
            );
            remainingProof = _remainingProof;
            require(
                !core.isRegisteredPsiPoint(
                    fromAccountData.relayer,
                    psiPublicInput.psiPoint
                ),
                "psiPoint is already registered."
            );
            core.registerPsiPoint(psiPublicInput.psiPoint);
        }
        nonce++;

        address subjectAccountAddr = address(0);
        /// 3. Process when `isSubjectAddrNull==false`.
        if (!senderPublicInput.isSubjectAddrNull) {
            (IVerifier.RecipientPublicInput memory recipientPublicInput, ) = abi
                .decode(
                    remainingVerifierParams,
                    (IVerifier.RecipientPublicInput, bytes)
                );
            subjectAccountAddr = core.getAccountOfSalt(
                recipientPublicInput.subjectSalt
            );
            (bytes memory recipientProof, ) = abi.decode(
                remainingProof,
                (bytes, bytes)
            );
            if (subjectAccountAddr == address(0)) {
                /// [TODO] Fix a logic to decide `pubKey` provided to `registerNewAccount`.
                core.registerNewAccount(
                    senderPublicInput.pubKey,
                    recipientPublicInput.subjectSalt
                );
                subjectAccountAddr = core.getAccountOfSalt(
                    recipientPublicInput.subjectSalt
                );
            }
            IAccount subjectAccount = IAccount(subjectAccountAddr);
            subjectAccount.processRecipientProof(
                recipientPublicInput,
                recipientProof,
                senderPublicInput.subjectEmailNullifier
            );
        }

        /// 4. Call an extension contract.
        address extensionAddr = extensionAddrOfId[extensionId];
        IExtension extension = IExtension(extensionAddr);
        IExtension.CallContext memory callCtx = IExtension.CallContext(
            extensionId,
            senderPublicInput.fromEmailNullifier,
            subjectAccountAddr
        );
        IExtension.ForwardContext memory forwardCtx = IExtension.ForwardContext(
            false,
            0
        );
        if (
            callCtx.extensionId == Constants.WALLET_EXTENSION_ID ||
            callCtx.extensionId == Constants.CONFIG_EXTENSION_ID ||
            callCtx.extensionId == Constants.EXT_EXTENSION_ID ||
            callCtx.extensionId == Constants.TRANSPORT_EXTENSION_ID
        ) {
            _delegateExtCall(
                address(extension),
                callCtx,
                forwardCtx,
                extensionParams
            );
        } else {
            extension.execute(callCtx, forwardCtx, extensionParams);
        }
    }

    function isExtensionInstalled(
        uint extensionId
    ) external view returns (bool) {
        return extensionAddrOfId[extensionId] != address(0);
    }

    function forwardCall(
        IExtension.CallContext memory callCtx,
        bytes memory extensionParams
    ) public {
        uint256 callerExtensionId = extensionIdOfAddr[msg.sender];
        require(
            callerExtensionId != 0,
            "The id 0 extension cannot forward a call."
        );
        uint256 calleeExtensionId = callCtx.extensionId;
        require(
            forwardPermissions[callerExtensionId][calleeExtensionId],
            "Not permitted forward"
        );
        address calleeAddr = extensionAddrOfId[calleeExtensionId];
        require(calleeAddr != address(0), "Not registered calleeExtensionId");
        IExtension.ForwardContext memory forwardCtx = IExtension.ForwardContext(
            true,
            callerExtensionId
        );
        if (
            calleeExtensionId == Constants.WALLET_EXTENSION_ID ||
            calleeExtensionId == Constants.CONFIG_EXTENSION_ID ||
            calleeExtensionId == Constants.EXT_EXTENSION_ID ||
            calleeExtensionId == Constants.TRANSPORT_EXTENSION_ID
        ) {
            _delegateExtCall(calleeAddr, callCtx, forwardCtx, extensionParams);
        } else {
            IExtension callee = IExtension(calleeAddr);
            callee.execute(callCtx, forwardCtx, extensionParams);
        }
    }

    function upgradeAccountLogic(address newLogic) public onlySelf {
        /// [TODO] Upgrade the account logic.
    }

    function changeVerifier(address newVerifier) public onlySelf {
        require(newVerifier != address(0), "newVerifier is not zero address.");
        verifierAddr = newVerifier;
    }

    function changeExtension(
        uint extensionId,
        address extensionAddr
    ) public onlySelf {
        require(
            extensionAddr != address(0),
            "extensionAddr is not zero address."
        );
        extensionAddrOfId[extensionId] = extensionAddr;
        extensionIdOfAddr[extensionAddr] = extensionId;
    }

    function addPermission(uint256 fromId, uint256 toId) external onlySelf {
        require(!forwardPermissions[fromId][toId], "Already permitted");
        forwardPermissions[fromId][toId] = true;
    }

    function removePermission(uint256 fromId, uint256 toId) external onlySelf {
        require(forwardPermissions[fromId][toId], "Not permitted");
        forwardPermissions[fromId][toId] = false;
    }

    function _delegateExtCall(
        address extensionAddr,
        IExtension.CallContext memory callCtx,
        IExtension.ForwardContext memory forwardCtx,
        bytes memory extensionParams
    ) private {
        (bool success, bytes memory returnData) = extensionAddr.delegatecall(
            abi.encodeWithSignature(
                "execute((uint256,bytes32,address),(bool,uint256),bytes)",
                callCtx,
                forwardCtx,
                extensionParams
            )
        );
        if (!success) {
            if (returnData.length == 0) revert();
            assembly {
                revert(add(32, returnData), mload(returnData))
            }
        }
    }
}
