// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

interface IVerifier {
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
    ) external view returns (bool);

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
    ) external view returns (bool);

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
    ) external view returns (bool);

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
    ) external view returns (bool);

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
    ) external view returns (bool);
}
