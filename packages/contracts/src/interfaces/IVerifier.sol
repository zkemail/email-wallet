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


    /**
        This proof will verify that:
            - relayer received an email from seder which contained the maskedSubjectStr
            and DKIM signed by public key whose hash is dkimPublicKeyHash
            - senderEmailAddressPointer, senderViewingKeyCommitment are calculated
            from the same emailAddress
            - emailNullifier is hash of the email headers
            - hasRecipient is true if the subject has a recipient
            - isRecipientExternal is true if the recipient is ETH address instead of email 
            - recipientEmailAddressWitness is hash of emailAddress and a randomness

        Note: relayerHash, senderEmailAddressPointer, senderViewingKeyCommitment are previously
        registered by the relayer. dkimPublicKeyHash is from the stored mapping.
     */
    function verifyEmailProof(
        bytes32 senderRelayerHash,
        bytes32 senderEmailAddressPointer,
        bytes32 senderViewingKeyCommitment,
        bool hasRecipient,
        bool isRecipientExternal,
        bytes32 recipientEmailAddressWitness,
        string memory maskedSubjectStr,
        bytes32 emailNullifier,
        string memory senderEmailDomain,
        bytes32 dkimPublicKeyHash,
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
