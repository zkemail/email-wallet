// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../interfaces/IVerifier.sol";
import "./verifiers/AccountCreationVerifier.sol";
import "./verifiers/AccountInitVerifier.sol";
import "./verifiers/AccountTransportVerifier.sol";
import "./verifiers/ClaimVerifier.sol";
import "./verifiers/EmailSenderVerifier.sol";

contract AllVerifiers is IVerifier {
    AccountCreationVerifier public immutable accountCreationVerifier;
    AccountInitVerifier public immutable accountInitVerifier;
    AccountTransportVerifier public immutable accountTransportVerifier;
    ClaimVerifier public immutable claimVerifier;
    EmailSenderVerifier public immutable emailSenderVerifier;

    uint256 public constant DOMAIN_BYTES = 255;
    uint256 public constant DOMAIN_FIELDS = 9;
    uint256 public constant SUBJECT_BYTES = 512;
    uint256 public constant SUBJECT_FIELDS = 17;

    constructor(
        address _accountCreationVerifier,
        address _accountInitVerifier,
        address _accountTransportVerifier,
        address _claimVerifier,
        address _emailSenderVerifier
    ) {
        accountCreationVerifier = AccountCreationVerifier(_accountCreationVerifier);
        accountInitVerifier = AccountInitVerifier(_accountInitVerifier);
        accountTransportVerifier = AccountTransportVerifier(_accountTransportVerifier);
        claimVerifier = ClaimVerifier(_claimVerifier);
        emailSenderVerifier = EmailSenderVerifier(_emailSenderVerifier);
    }

    /// @notice Verify the proof to create an account
    /// @notice Verify `emailAddrPointer`, `accountKeyCommit` and `walletSalt`
    ///         are calculated from the same `emailAddress`, `accountKey` and `relayerRandomness`
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddrPointer The hash of the relayer randomness and email address
    /// @param accountKeyCommit The hash of the account key, email address and relayer randomness
    /// @param walletSalt The hash of the account key and 01
    /// @param psiPoint The psi point of the email address
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) =
            abi.decode(proof, (uint256[2], uint256[2][2], uint256[2]));
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

    /// @notice Verify the proof to initialize an account (reply to the invitation email)
    /// @notice This verify the relayer received an email from the user (with corresponding `emailAddrPointer`)
    ///         where `accountKey` (of corresponding `accountKeyCommit`) was used in `x-reply-to` header
    /// @param emailDomain The domain of the user's email address
    /// @param dkimPublicKeyHash The hash of the DKIM public key of `emailDomain`
    /// @param timestamp The timestamp of the email
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddrPointer The hash of the relayer randomness and email address
    /// @param accountKeyCommit The hash of the account key, email address and relayer randomness
    /// @param emailNullifier The nullifier computed for the reply email
    /// @param proof Proof of email with above constraints
    /// @dev `accountKeyCommit`, `dkimPublicKeyHash` should be the values previously stored in the contract
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
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) =
            abi.decode(proof, (uint256[2], uint256[2][2], uint256[2]));
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

    /// @notice Verify the proof of email from user - used to verify EmailOp
    /// @notice Verify that relayer received an email where:
    ///         sender's email address domain is `emailDomain`,
    ///         sender's email address and relayer randmness derives `emailAddrPointer`,
    ///         is DKIM signed by public key whose hash is `dkimPublicKeyHash`,
    ///         the subject is same as `maskedSubject` with email address masked (if any),
    ///         and email address in subject is used to derive `recipientEmailAddrCommit`
    /// @param emailDomain The domain of the user's email address
    /// @param dkimPublicKeyHash The hash of the DKIM public key of `emailDomain`
    /// @param timestamp The timestamp of the email
    /// @param maskedSubject The subject of the email with (any) email address masked
    /// @param emailNullifier The nullifier computed for the email
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddrPointer The hash of the relayer randomness and users's email address
    /// @param recipientEmailAddrCommit The hash of recipeint's email address (from subject) and a randomness
    /// @param hasEmailRecipient Whether the email subject has a recipient (email address)
    /// @dev `relayerHash`, `emailAddrPointer`, `dkimPublicKeyHash` should be the values previously stored in the contract
    function verifyEmailProof(
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
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) =
            abi.decode(proof, (uint256[2], uint256[2][2], uint256[2]));
        uint256[33] memory pubSignals = genPubSignalsOfEmailProof(emailDomain, dkimPublicKeyHash, timestamp, maskedSubject, emailNullifier, relayerHash, emailAddrPointer, hasEmailRecipient, recipientEmailAddrCommit);
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

    /// @notice Verify the proof to claim and unclaimed to a recipient account
    /// @notice This verify that same email address is used in `recipientEmailAddrPointer` and `recipientEmailAddrCommit`
    /// @param recipientRelayerHash The hash of the relayer randomness
    /// @param recipientEmailAddrPointer The hash of the relayer randomness and recipient email address
    /// @param recipientEmailAddrCommit The hash(emailAddress, randomness) where randomness was set by sender and passed to recipient relayer
    /// @param proof ZK proof of the circuit
    function verifyClaimFundProof(
        bytes32 recipientRelayerHash,
        bytes32 recipientEmailAddrPointer,
        bytes32 recipientEmailAddrCommit,
        bytes memory proof
    ) external view returns (bool) {
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) =
            abi.decode(proof, (uint256[2], uint256[2][2], uint256[2]));
        uint256[3] memory pubSignals;
        pubSignals[0] = uint256(recipientRelayerHash);
        pubSignals[1] = uint256(recipientEmailAddrPointer);
        pubSignals[2] = uint256(recipientEmailAddrCommit);
        return claimVerifier.verifyProof(pA, pB, pC, pubSignals);
    }

    /// @notice Verify the proof to transport account from one relayer to another
    /// @notice This will verify that relayer received an email from user with their account key somewhere in header
    ///         and the email is DKIM signed by the public key of `emailDomain` whose hash is `dkimPublicKeyHash`.
    ///         Also proved that the accountKeyCommit of old relayer with hash `oldRelayerRandHash` is same as `oldAccountKeyCommit`
    /// @param emailDomain The domain of the user's email address
    /// @param dkimPublicKeyHash The hash of the DKIM public key of `emailDomain`
    /// @param timestamp The timestamp of the email
    /// @param emailNullifier The nullifier computed for the email
    /// @param oldRelayerRandHash The hash of the old relayer randomness
    /// @param oldAccountKeyCommit The hash of the account key, email address and old relayer randomness
    /// @param proof Proof of email with above constraints
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
        (uint256[2] memory pA, uint256[2][2] memory pB, uint256[2] memory pC) =
            abi.decode(proof, (uint256[2], uint256[2][2], uint256[2]));
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
