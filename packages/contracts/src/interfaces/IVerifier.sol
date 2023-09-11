// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

interface IVerifier {
    /// @notice Verify the proof to create an account
    /// @notice Verify `emailAddressPointer`, `viewingKeyCommitment` and `walletSalt`
    ///         are calculated from the same `emailAddress`, `viewingKey` and `relayerRandomness`
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddressPointer The hash of the relayer randomness and email address
    /// @param viewingKeyCommitment The hash of the viewing key, email address and relayer randomness
    /// @param walletSalt The hash of the viewing key and 01
    /// @param psiPoint The psi point of the email address
    function verifyAccountCreationProof(
        bytes32 relayerHash,
        bytes32 emailAddressPointer,
        bytes32 viewingKeyCommitment,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) external view returns (bool);

    /// @notice Verify the proof to initialize an account (reply to the invitation email)
    /// @notice This verify the relayer received an email from the user (with corresponding `emailAddressPointer`)
    ///         where `viewingKey` (of corresponding `viewingKeyCommitment`) was used in `x-reply-to` header
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddressPointer The hash of the relayer randomness and email address
    /// @param viewingKeyCommitment The hash of the viewing key, email address and relayer randomness
    /// @param emailDomain The domain of the user's email address
    /// @param dkimPublicKeyHash The hash of the DKIM public key of `emailDomain`
    /// @param emailNullifier The nullifier computed for the reply email
    /// @param proof Proof of email with above constraints
    /// @dev `viewingKeyCommitment`, `dkimPublicKeyHash` should be the values previously stored in the contract
    function verifyAccountInitializaionProof(
        bytes32 relayerHash,
        bytes32 emailAddressPointer,
        bytes32 viewingKeyCommitment,
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        bytes32 emailNullifier,
        bytes memory proof
    ) external view returns (bool);

    /// @notice Verify the proof of email from user - used to verify EmailOp
    /// @notice Verify that relayer received an email where:
    ///         sender's email address domain is `emailDomain`,
    ///         sender's email address and relayer randmness derives `emailAddressPointer`,
    ///         is DKIM signed by public key whose hash is `dkimPublicKeyHash`,
    ///         the subject is same as `maskedSubject` with email address masked (if any),
    ///         and email address in subject is used to derive `recipientEmailAddressCommitment`
    /// @param emailDomain The domain of the user's email address
    /// @param dkimPublicKeyHash The hash of the DKIM public key of `emailDomain`
    /// @param maskedSubject The subject of the email with (any) email address masked
    /// @param emailNullifier The nullifier computed for the email
    /// @param relayerHash The hash of the relayer randomness
    /// @param emailAddressPointer The hash of the relayer randomness and users's email address
    /// @param recipientEmailAddressCommitment The hash of recipeint's email address (from subject) and a randomness
    /// @param hasEmailRecipient Whether the email subject has a recipient (email address)
    /// @dev `relayerHash`, `emailAddressPointer`, `dkimPublicKeyHash` should be the values previously stored in the contract
    function verifyEmailProof(
        string memory emailDomain,
        bytes32 dkimPublicKeyHash,
        string memory maskedSubject,
        bytes32 emailNullifier,
        bytes32 relayerHash,
        bytes32 emailAddressPointer,
        bool hasEmailRecipient,
        bytes32 recipientEmailAddressCommitment,
        bytes memory proof
    ) external view returns (bool);

    /// @notice Verify the proof to claim and unclaimed to a recipient account
    /// @notice This verify that same email address is used in `recipientEmailAddressPointer` and `recipientEmailAddressCommitment`
    /// @param recipientRelayerHash The hash of the relayer randomness
    /// @param recipientEmailAddressPointer The hash of the relayer randomness and recipient email address
    /// @param recipientEmailAddressCommitment The hash(emailAddress, randomness) where randomness was set by sender and passed to recipient relayer
    /// @param proof ZK proof of the circuit
    function verifyClaimFundProof(
        bytes32 recipientRelayerHash,
        bytes32 recipientEmailAddressPointer,
        bytes32 recipientEmailAddressCommitment,
        bytes memory proof
    ) external view returns (bool);
}
