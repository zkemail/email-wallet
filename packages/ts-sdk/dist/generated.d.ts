export declare const emailWalletCoreAbi: readonly [{
    readonly type: "constructor";
    readonly inputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "fallback";
    readonly stateMutability: "payable";
}, {
    readonly type: "receive";
    readonly stateMutability: "payable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "accountHandler";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract AccountHandler";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "tokenAddr";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "amount";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "depositTokenAsExtension";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
    }];
    readonly name: "emailNullifiers";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bool";
        readonly type: "bool";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "emailValidityDuration";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "target";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "data";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "executeAsExtension";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "extensionHandler";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract ExtensionHandler";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "emailOp";
        readonly internalType: "struct EmailOp";
        readonly type: "tuple";
        readonly components: readonly [{
            readonly name: "accountSalt";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "command";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "emailNullifier";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "emailDomain";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "dkimPublicKeyHash";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "maskedSubject";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "skipSubjectPrefix";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "timestamp";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "hasEmailRecipient";
            readonly internalType: "bool";
            readonly type: "bool";
        }, {
            readonly name: "recipientEmailAddrCommit";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "numRecipientEmailAddrBytes";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "recipientETHAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "feeTokenName";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "feePerGas";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "executeCallData";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }, {
            readonly name: "extensionName";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "newWalletOwner";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "newDkimRegistry";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "walletParams";
            readonly internalType: "struct WalletParams";
            readonly type: "tuple";
            readonly components: readonly [{
                readonly name: "tokenName";
                readonly internalType: "string";
                readonly type: "string";
            }, {
                readonly name: "amount";
                readonly internalType: "uint256";
                readonly type: "uint256";
            }];
        }, {
            readonly name: "extensionParams";
            readonly internalType: "struct ExtensionParams";
            readonly type: "tuple";
            readonly components: readonly [{
                readonly name: "subjectTemplateIndex";
                readonly internalType: "uint8";
                readonly type: "uint8";
            }, {
                readonly name: "subjectParams";
                readonly internalType: "bytes[]";
                readonly type: "bytes[]";
            }];
        }, {
            readonly name: "emailProof";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }];
    }];
    readonly name: "handleEmailOp";
    readonly outputs: readonly [{
        readonly name: "success";
        readonly internalType: "bool";
        readonly type: "bool";
    }, {
        readonly name: "err";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }, {
        readonly name: "totalFeeInETH";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "registeredUnclaimId";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "payable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "_relayerHandler";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_accountHandler";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_unclaimsHandler";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_extensionHandler";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_verifier";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_tokenRegistry";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_priceOracle";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_wethContract";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "_maxFeePerGas";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "_emailValidityDuration";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "_unclaimedFundClaimGas";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "_unclaimedStateClaimGas";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "initialize";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "defaultExtensions";
        readonly internalType: "bytes[]";
        readonly type: "bytes[]";
    }];
    readonly name: "initializeExtension";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "maxFeePerGas";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "owner";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "priceOracle";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract IPriceOracle";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "proxiableUUID";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "extensionAddr";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "state";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "registerUnclaimedStateAsExtension";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "relayerHandler";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract RelayerHandler";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "renounceOwnership";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "tokenAddr";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "amount";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "requestTokenAsExtension";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "tokenRegistry";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract TokenRegistry";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newOwner";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "transferOwnership";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "unclaimedFundClaimGas";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "unclaimedStateClaimGas";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "unclaimsHandler";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract UnclaimsHandler";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newImplementation";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "upgradeTo";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newImplementation";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "data";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "upgradeToAndCall";
    readonly outputs: readonly [];
    readonly stateMutability: "payable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "emailOp";
        readonly internalType: "struct EmailOp";
        readonly type: "tuple";
        readonly components: readonly [{
            readonly name: "accountSalt";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "command";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "emailNullifier";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "emailDomain";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "dkimPublicKeyHash";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "maskedSubject";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "skipSubjectPrefix";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "timestamp";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "hasEmailRecipient";
            readonly internalType: "bool";
            readonly type: "bool";
        }, {
            readonly name: "recipientEmailAddrCommit";
            readonly internalType: "bytes32";
            readonly type: "bytes32";
        }, {
            readonly name: "numRecipientEmailAddrBytes";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "recipientETHAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "feeTokenName";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "feePerGas";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "executeCallData";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }, {
            readonly name: "extensionName";
            readonly internalType: "string";
            readonly type: "string";
        }, {
            readonly name: "newWalletOwner";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "newDkimRegistry";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "walletParams";
            readonly internalType: "struct WalletParams";
            readonly type: "tuple";
            readonly components: readonly [{
                readonly name: "tokenName";
                readonly internalType: "string";
                readonly type: "string";
            }, {
                readonly name: "amount";
                readonly internalType: "uint256";
                readonly type: "uint256";
            }];
        }, {
            readonly name: "extensionParams";
            readonly internalType: "struct ExtensionParams";
            readonly type: "tuple";
            readonly components: readonly [{
                readonly name: "subjectTemplateIndex";
                readonly internalType: "uint8";
                readonly type: "uint8";
            }, {
                readonly name: "subjectParams";
                readonly internalType: "bytes[]";
                readonly type: "bytes[]";
            }];
        }, {
            readonly name: "emailProof";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }];
    }];
    readonly name: "validateEmailOp";
    readonly outputs: readonly [];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "verifier";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract IVerifier";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "wethContract";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "previousAdmin";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: false;
    }, {
        readonly name: "newAdmin";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: false;
    }];
    readonly name: "AdminChanged";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "beacon";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "BeaconUpgraded";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "success";
        readonly internalType: "bool";
        readonly type: "bool";
        readonly indexed: true;
    }, {
        readonly name: "registeredUnclaimId";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: true;
    }, {
        readonly name: "emailNullifier";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
        readonly indexed: true;
    }, {
        readonly name: "accountSalt";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
        readonly indexed: false;
    }, {
        readonly name: "recipientEmailAddrCommit";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
        readonly indexed: false;
    }, {
        readonly name: "recipientETHAddr";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: false;
    }, {
        readonly name: "err";
        readonly internalType: "bytes";
        readonly type: "bytes";
        readonly indexed: false;
    }];
    readonly name: "EmailOpHandled";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "version";
        readonly internalType: "uint8";
        readonly type: "uint8";
        readonly indexed: false;
    }];
    readonly name: "Initialized";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "previousOwner";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "newOwner";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "OwnershipTransferred";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "implementation";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "Upgraded";
}];
export declare const iOauthAbi: readonly [{
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "getInfoOfWalletAndNonce";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "tokenAddr";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "getTokenAkkowancesOfWalletAndNonce";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "getUsernameOfWallet";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "string";
        readonly type: "string";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
    }];
    readonly name: "getWalletOfUsername";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "token";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "amount";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "reduceTokenAllowance";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "epheAddr";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "registerEpheAddr";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "expiry";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "tokenAllowances";
        readonly internalType: "struct TokenAllowance[]";
        readonly type: "tuple[]";
        readonly components: readonly [{
            readonly name: "tokenAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "amount";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }];
    }];
    readonly name: "signin";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
    }];
    readonly name: "signup";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "epheAddr";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly name: "validateEpheAddr";
    readonly outputs: readonly [];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "epheAddr";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "hash";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
    }, {
        readonly name: "signature";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "validateSignature";
    readonly outputs: readonly [];
    readonly stateMutability: "view";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "token";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "amount";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: true;
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: false;
    }];
    readonly name: "ReducedTokenAllowance";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "epheAddr";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: true;
    }, {
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
        readonly indexed: false;
    }];
    readonly name: "RegisteredEpheAddr";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
        readonly indexed: true;
    }, {
        readonly name: "nonce";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: true;
    }, {
        readonly name: "expiry";
        readonly internalType: "uint256";
        readonly type: "uint256";
        readonly indexed: false;
    }, {
        readonly name: "tokenAllowances";
        readonly internalType: "struct TokenAllowance[]";
        readonly type: "tuple[]";
        readonly components: readonly [{
            readonly name: "tokenAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "amount";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }];
        readonly indexed: false;
    }];
    readonly name: "Signin";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "wallet";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "username";
        readonly internalType: "string";
        readonly type: "string";
        readonly indexed: true;
    }];
    readonly name: "Signup";
}];
export declare const walletAbi: readonly [{
    readonly type: "constructor";
    readonly inputs: readonly [{
        readonly name: "wethAddress";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "oauthAddress";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "fallback";
    readonly stateMutability: "payable";
}, {
    readonly type: "receive";
    readonly stateMutability: "payable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "epheTxNonce";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "target";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "value";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "data";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "execute";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "txData";
        readonly internalType: "struct EphemeralTx";
        readonly type: "tuple";
        readonly components: readonly [{
            readonly name: "walletAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "txNonce";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "epheAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "epheAddrNonce";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "target";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "ethValue";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "data";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }, {
            readonly name: "tokenAmount";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "signature";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }];
    }];
    readonly name: "executeEphemeralTx";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "getOauth";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "contract IOauth";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "txData";
        readonly internalType: "struct EphemeralTx";
        readonly type: "tuple";
        readonly components: readonly [{
            readonly name: "walletAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "txNonce";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "epheAddr";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "epheAddrNonce";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "target";
            readonly internalType: "address";
            readonly type: "address";
        }, {
            readonly name: "ethValue";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "data";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }, {
            readonly name: "tokenAmount";
            readonly internalType: "uint256";
            readonly type: "uint256";
        }, {
            readonly name: "signature";
            readonly internalType: "bytes";
            readonly type: "bytes";
        }];
    }];
    readonly name: "hashEphemeralTx";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
    }];
    readonly stateMutability: "pure";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "initialize";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "uint256[]";
        readonly type: "uint256[]";
    }, {
        readonly name: "";
        readonly internalType: "uint256[]";
        readonly type: "uint256[]";
    }, {
        readonly name: "";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "onERC1155BatchReceived";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes4";
        readonly type: "bytes4";
    }];
    readonly stateMutability: "pure";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "onERC1155Received";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes4";
        readonly type: "bytes4";
    }];
    readonly stateMutability: "pure";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "onERC721Received";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes4";
        readonly type: "bytes4";
    }];
    readonly stateMutability: "pure";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "owner";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "proxiableUUID";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bytes32";
        readonly type: "bytes32";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [];
    readonly name: "renounceOwnership";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "interfaceId";
        readonly internalType: "bytes4";
        readonly type: "bytes4";
    }];
    readonly name: "supportsInterface";
    readonly outputs: readonly [{
        readonly name: "";
        readonly internalType: "bool";
        readonly type: "bool";
    }];
    readonly stateMutability: "view";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "";
        readonly internalType: "uint256";
        readonly type: "uint256";
    }, {
        readonly name: "";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }, {
        readonly name: "";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "tokensReceived";
    readonly outputs: readonly [];
    readonly stateMutability: "pure";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newOwner";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "transferOwnership";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newImplementation";
        readonly internalType: "address";
        readonly type: "address";
    }];
    readonly name: "upgradeTo";
    readonly outputs: readonly [];
    readonly stateMutability: "nonpayable";
}, {
    readonly type: "function";
    readonly inputs: readonly [{
        readonly name: "newImplementation";
        readonly internalType: "address";
        readonly type: "address";
    }, {
        readonly name: "data";
        readonly internalType: "bytes";
        readonly type: "bytes";
    }];
    readonly name: "upgradeToAndCall";
    readonly outputs: readonly [];
    readonly stateMutability: "payable";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "previousAdmin";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: false;
    }, {
        readonly name: "newAdmin";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: false;
    }];
    readonly name: "AdminChanged";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "beacon";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "BeaconUpgraded";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "version";
        readonly internalType: "uint8";
        readonly type: "uint8";
        readonly indexed: false;
    }];
    readonly name: "Initialized";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "previousOwner";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }, {
        readonly name: "newOwner";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "OwnershipTransferred";
}, {
    readonly type: "event";
    readonly anonymous: false;
    readonly inputs: readonly [{
        readonly name: "implementation";
        readonly internalType: "address";
        readonly type: "address";
        readonly indexed: true;
    }];
    readonly name: "Upgraded";
}];
