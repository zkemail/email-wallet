// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../interfaces/IVerifier.sol";
import "./AccountCreationVerifier.sol";
import "./AccountInitVerifier.sol";
import "./AccountTransportVerifier.sol";
import "./ClaimVerifier.sol";
import "./EmailSenderVerifier.sol";
import "./AnnouncementVerifier.sol";

contract AllVerifiers is IVerifier {
    AccountCreationVerifier public immutable accountCreationVerifier;
    AccountInitVerifier public immutable accountInitVerifier;
    AccountTransportVerifier public immutable accountTransportVerifier;
    ClaimVerifier public immutable claimVerifier;
    EmailSenderVerifier public immutable emailSenderVerifier;
    AnnouncementVerifier public immutable announcementVerifier;

    uint256 public constant DOMAIN_BYTES = 255;
    uint256 public constant DOMAIN_FIELDS = 9;
    uint256 public constant SUBJECT_BYTES = 512;
    uint256 public constant SUBJECT_FIELDS = 17;
    uint256 public constant EMAIL_ADDR_BYTES = 256;
    uint256 public constant EMAIL_ADDR_FIELDS = 9;

    constructor() {
        accountCreationVerifier = new AccountCreationVerifier();
        accountInitVerifier = new AccountInitVerifier();
        accountTransportVerifier = new AccountTransportVerifier();
        claimVerifier = new ClaimVerifier();
        emailSenderVerifier = new EmailSenderVerifier();
        announcementVerifier = new AnnouncementVerifier();
    }

    /// @inheritdoc IVerifier
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[6] memory pubSignals;
        pubSignals[0] = uint256(relayerHash);
        pubSignals[1] = uint256(emailAddrPointer);
        pubSignals[2] = uint256(accountKeyCommit);
        pubSignals[3] = uint256(walletSalt);
        (uint256 x, uint256 y) = abi.decode(psiPoint, (uint256, uint256));
        pubSignals[4] = x;
        pubSignals[5] = y;
        return accountCreationVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    /// @inheritdoc IVerifier
    function verifyAccountInitializaionProof(
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        uint256 timestamp,
        bytes32 relayerHash,
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 emailNullifier,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[15] memory pubSignals;
        uint256[] memory domainFields = _packBytes2Fields(bytes(emailDomain), DOMAIN_BYTES);
        for (uint256 i = 0; i < DOMAIN_FIELDS; i++) {
            pubSignals[i] = domainFields[i];
        }
        pubSignals[DOMAIN_FIELDS] = uint256(dkimPublicKeyHash);
        pubSignals[DOMAIN_FIELDS + 1] = uint256(relayerHash);
        pubSignals[DOMAIN_FIELDS + 2] = uint256(emailNullifier);
        pubSignals[DOMAIN_FIELDS + 3] = uint256(emailAddrPointer);
        pubSignals[DOMAIN_FIELDS + 4] = uint256(accountKeyCommit);
        pubSignals[DOMAIN_FIELDS + 5] = uint256(timestamp);
        return accountInitVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    /// @inheritdoc IVerifier
    function verifyEmailOpProof(
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        uint256 timestamp,
        string memory maskedSubject,
        bytes32 emailNullifier,
        bytes32 relayerHash,
        bytes32 emailAddrPointer,
        bool hasEmailRecipient,
        bytes32 recipientEmailAddrCommit,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[33] memory pubSignals = genPubSignalsOfEmailProof(
            emailDomain,
            dkimPublicKeyHash,
            timestamp,
            maskedSubject,
            emailNullifier,
            relayerHash,
            emailAddrPointer,
            hasEmailRecipient,
            recipientEmailAddrCommit
        );
        return emailSenderVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    function genPubSignalsOfEmailProof(
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        uint256 timestamp,
        string memory maskedSubject,
        bytes32 emailNullifier,
        bytes32 relayerHash,
        bytes32 emailAddrPointer,
        bool hasEmailRecipient,
        bytes32 recipientEmailAddrCommit
    ) internal pure returns (uint256[33] memory) {
        uint256[33] memory pubSignals;
        uint256[] memory stringFields;
        stringFields = _packBytes2Fields(bytes(maskedSubject), SUBJECT_BYTES);
        for (uint256 i = 0; i < SUBJECT_FIELDS; i++) {
            pubSignals[i] = stringFields[i];
        }
        delete stringFields;
        stringFields = _packBytes2Fields(bytes(emailDomain), DOMAIN_BYTES);
        for (uint256 i = 0; i < DOMAIN_FIELDS; i++) {
            pubSignals[SUBJECT_FIELDS + i] = stringFields[i];
        }
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS] = uint256(dkimPublicKeyHash);
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 1] = uint256(relayerHash);
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 2] = uint256(emailNullifier);
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 3] = uint256(emailAddrPointer);
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 4] = hasEmailRecipient ? 1 : 0;
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 5] = uint256(recipientEmailAddrCommit);
        pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 6] = timestamp;
        return pubSignals;
    }

    /// @inheritdoc IVerifier
    function verifyClaimFundProof(
        bytes32 recipientRelayerHash,
        bytes32 recipientEmailAddrPointer,
        bytes32 recipientEmailAddrCommit,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[3] memory pubSignals;
        pubSignals[0] = uint256(recipientRelayerHash);
        pubSignals[1] = uint256(recipientEmailAddrPointer);
        pubSignals[2] = uint256(recipientEmailAddrCommit);
        return claimVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    /// @inheritdoc IVerifier
    function verifyAccountTransportProof(
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        uint256 timestamp,
        bytes32 emailNullifier,
        bytes32 oldRelayerRandHash,
        bytes32 newRelayerRandHash,
        bytes32 oldAccountKeyCommit,
        bytes32 newAccountKeyCommit,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[16] memory pubSignals;
        uint256[] memory domainFields = _packBytes2Fields(bytes(emailDomain), DOMAIN_BYTES);
        for (uint256 i = 0; i < DOMAIN_FIELDS; i++) {
            pubSignals[i] = domainFields[i];
        }
        pubSignals[DOMAIN_FIELDS] = uint256(dkimPublicKeyHash);
        pubSignals[DOMAIN_FIELDS + 1] = uint256(emailNullifier);
        pubSignals[DOMAIN_FIELDS + 2] = uint256(oldAccountKeyCommit);
        pubSignals[DOMAIN_FIELDS + 3] = uint256(newAccountKeyCommit);
        pubSignals[DOMAIN_FIELDS + 4] = uint256(newRelayerRandHash);
        pubSignals[DOMAIN_FIELDS + 5] = uint256(timestamp);
        pubSignals[DOMAIN_FIELDS + 6] = uint256(oldRelayerRandHash);
        return accountTransportVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    function verifyAnnouncementProof(
        string memory emailAddr,
        bytes32 rand,
        bytes32 emailAddrCommit,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) = abi.decode(
            proof,
            (uint256[2], uint256[2][2], uint256[2])
        );
        uint256[EMAIL_ADDR_FIELDS + 2] memory pubSignals;
        uint256[] memory emailAddrFields = _packBytes2Fields(bytes(emailAddr), EMAIL_ADDR_BYTES);
        for (uint256 i = 0; i < EMAIL_ADDR_FIELDS; i++) {
            pubSignals[i] = emailAddrFields[i];
        }
        pubSignals[EMAIL_ADDR_FIELDS] = uint256(emailAddrCommit);
        pubSignals[EMAIL_ADDR_FIELDS + 1] = uint256(rand);
        return announcementVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    function _packBytes2Fields(bytes memory _bytes, uint256 _paddedSize) public pure returns (uint256[] memory) {
        uint256 remain = _paddedSize % 31;
        uint256 numFields = (_paddedSize - remain) / 31;
        if (remain > 0) {
            numFields += 1;
        }
        uint256[] memory fields = new uint[](numFields);
        uint256 idx = 0;
        uint256 byteVal = 0;
        for (uint256 i = 0; i < numFields; i++) {
            for (uint256 j = 0; j < 31; j++) {
                idx = i * 31 + j;
                if (idx >= _paddedSize) {
                    break;
                }
                if (idx >= _bytes.length) {
                    byteVal = 0;
                } else {
                    byteVal = uint256(uint8(_bytes[idx]));
                }
                if (j == 0) {
                    fields[i] = byteVal;
                } else {
                    fields[i] += (byteVal << (8 * j));
                }
            }
        }
        return fields;
    }
}
