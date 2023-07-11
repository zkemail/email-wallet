// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IVerifier {
    struct SenderPublicInput {
        bytes32 fromRelayerHash;
        bytes32 fromSalt;
        bytes32 fromEmailNullifier;
        string maskedSubjectStr;
        bool isSubjectAddrNull;
        bytes32 subjectEmailNullifier;
        bytes pubKey;
        bytes32 fromAddrCommit;
        bytes32 subjectAddrCommit;
        bytes others;
    }

    struct RecipientPublicInput {
        bytes32 subjectRelayerHash;
        bytes32 subjectSalt;
        bytes32 subjectAddrCommit;
        bytes others;
    }

    struct TransportPublicInput {
        bytes32 emailNullifier;
        bytes32 fromAddrCommit;
        bytes others;
    }

    struct PsiPublicInput {
        bytes32 relayerHash;
        bytes32 addrCommit;
        bytes psiPoint;
        bytes others;
    }

    function verifySenderProof(
        SenderPublicInput memory publicInput,
        bytes calldata proof
    ) external view returns (bool);

    function verifyRecipientProof(
        RecipientPublicInput memory publicInput,
        bytes calldata proof
    ) external view returns (bool);

    function verifyTransportProof(
        TransportPublicInput memory publicInput,
        bytes calldata proof
    ) external view returns (bool);

    function verifyPsiProof(
        PsiPublicInput memory publicInput,
        bytes calldata proof
    ) external view returns (bool);
}
