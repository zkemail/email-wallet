pub use oauth_core::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod oauth_core {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("epheAddrOfNonceHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("epheAddrOfNonceHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expiryOfNonceHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("expiryOfNonceHash"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInfoOfWalletAndNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInfoOfWalletAndNonce",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenAllowancesOfWalletAndNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTokenAllowancesOfWalletAndNonce",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUsernameOfWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getUsernameOfWallet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("wallet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWalletOfUsername"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getWalletOfUsername",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("username"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextNonceOfWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextNonceOfWallet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reduceTokenAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reduceTokenAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAllowances"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct TokenAllowance[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signup"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signup"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("username"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenAllowancesOfNonceHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenAllowancesOfNonceHash",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("usernameOfWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("usernameOfWallet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSignature"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("walletOfUsername"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("walletOfUsername"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("beacon"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReducedTokenAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ReducedTokenAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RegisteredEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RegisteredEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Signin"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Signin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAllowances"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Signup"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Signup"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OAUTHCORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[Pa\0\x1Da\0\"V[a\0\xE1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\0\xDFW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x80Qa,\xD0a\x01\x18`\09`\0\x81\x81a\x08\x8B\x01R\x81\x81a\t!\x01R\x81\x81a\n\\\x01R\x81\x81a\n\xF2\x01Ra\x0E\x16\x01Ra,\xD0`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x80W`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xD6W\x80c\xE9\x1D\xED,\x11a\0\x7FW\x80c\xF2\xFD\xE3\x8B\x11a\0YW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB5W\x80c\xF6CM\xA5\x14a\x04\xD5W\x80c\xFB\xFF\xA1k\x14a\x04\xF5W`\0\x80\xFD[\x80c\xE9\x1D\xED,\x14a\x04'W\x80c\xEB\x12\x02\xFF\x14a\x04uW\x80c\xF0\xE3\x84\xA5\x14a\x04\x95W`\0\x80\xFD[\x80c\xB5p:U\x11a\0\xB0W\x80c\xB5p:U\x14a\x03vW\x80c\xCFZ\xE5\x1D\x14a\x03\xC2W\x80c\xDBpF\xA5\x14a\x03\xFAW`\0\x80\xFD[\x80c\x81)\xFC\x1C\x14a\x03\x16W\x80c\x8D\xA5\xCB[\x14a\x03+W\x80c\x97\x16\x04\xC6\x14a\x03VW`\0\x80\xFD[\x80cQ\x9Ccw\x11a\x018W\x80cqP\x18\xA6\x11a\x01\x12W\x80cqP\x18\xA6\x14a\x02\xB4W\x80cr\x18\xFD\xA4\x14a\x02\xC9W\x80c\x80l\xD0j\x14a\x02\xF6W`\0\x80\xFD[\x80cQ\x9Ccw\x14a\x02_W\x80cR\xD1\x90-\x14a\x02\x7FW\x80c]\xCB\x7F\x88\x14a\x02\x94W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01iW\x80c6Y\xCF\xE6\x14a\x01\xF1W\x80c9\xB2QT\x14a\x02\x11W\x80cO\x1E\xF2\x86\x14a\x02LW`\0\x80\xFD[\x80c$a\x9A|\x14a\x01\x85W\x80c6M\xA6\xAF\x14a\x01\xA7W[`\0\x80\xFD[4\x80\x15a\x01\x91W`\0\x80\xFD[Pa\x01\xA5a\x01\xA06`\x04a$\"V[a\x058V[\0[4\x80\x15a\x01\xB3W`\0\x80\xFD[Pa\x01\xC7a\x01\xC26`\x04a%\x91V[a\x086V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFDW`\0\x80\xFD[Pa\x01\xA5a\x02\x0C6`\x04a%\xC6V[a\x08tV[4\x80\x15a\x02\x1DW`\0\x80\xFD[Pa\x02>a\x02,6`\x04a%\xE1V[`i` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01\xE8V[a\x01\xA5a\x02Z6`\x04a%\xFAV[a\nEV[4\x80\x15a\x02kW`\0\x80\xFD[Pa\x01\xA5a\x02z6`\x04a%\x91V[a\x0C\x07V[4\x80\x15a\x02\x8BW`\0\x80\xFD[Pa\x02>a\r\xFCV[4\x80\x15a\x02\xA0W`\0\x80\xFD[Pa\x01\xA5a\x02\xAF6`\x04a&HV[a\x0E\xCEV[4\x80\x15a\x02\xC0W`\0\x80\xFD[Pa\x01\xA5a\x12\x14V[4\x80\x15a\x02\xD5W`\0\x80\xFD[Pa\x02\xE9a\x02\xE46`\x04a%\xC6V[a\x12(V[`@Qa\x01\xE8\x91\x90a'\x89V[4\x80\x15a\x03\x02W`\0\x80\xFD[Pa\x02\xE9a\x03\x116`\x04a%\xC6V[a\x12\xE1V[4\x80\x15a\x03\"W`\0\x80\xFD[Pa\x01\xA5a\x13{V[4\x80\x15a\x037W`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC7V[4\x80\x15a\x03bW`\0\x80\xFD[Pa\x01\xA5a\x03q6`\x04a'\xDAV[a\x14\xF2V[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x03\x96a\x03\x916`\x04a(1V[a\x15\xAEV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE8V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x02>a\x03\xDD6`\x04a([V[`j` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04\x06W`\0\x80\xFD[Pa\x02>a\x04\x156`\x04a%\xC6V[`g` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x043W`\0\x80\xFD[Pa\x01\xC7a\x04B6`\x04a%\x91V[\x80Q` \x81\x83\x01\x81\x01\x80Q`f\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x02>a\x04\x906`\x04a(~V[a\x15\xFDV[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x01\xA5a\x04\xB06`\x04a(\xBAV[a\x16DV[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x01\xA5a\x04\xD06`\x04a%\xC6V[a\x17kV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x01\xA5a\x04\xF06`\x04a(\xF0V[a\x18\x05V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x01\xC7a\x05\x106`\x04a%\xE1V[`h` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Finvalid wallet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x06\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`g` R`@\x81 T\x90a\x064\x84\x83a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x06\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Fnonce already used\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x81\x81R`h` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x81\x16\x91\x90\x91\x17\x90\x91U\x87\x16\x83R`g\x90\x91R\x81 \x80T\x91a\x07\x14\x83a)KV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`e` R`@\x81 \x80Ta\x07I\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07u\x90a)\x83V[\x80\x15a\x07\xC2W\x80`\x1F\x10a\x07\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FC\x96\xA2\xE6\xE5\x14>Q\x99\xC0\xE1\xC6\x07\xB9\xDD\xE0^<%\x94r^\xF4;\xC7\xAAs9{b\xF2\x1F\x84`@Qa\x08'\x91\x90a'\x89V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\0`f\x82`@Qa\x08H\x91\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x94\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\n&\x81a\x1A\xB3V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\nB\x91\x83\x91\x90a\x1A\xBBV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0Be\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x0B\xF7\x82a\x1A\xB3V[a\x0C\x03\x82\x82`\x01a\x1A\xBBV[PPV[`@Q3\x90`\0\x90`f\x90a\x0C\x1D\x90\x85\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fusername already exists\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 \x80Ta\x0C\xC0\x90a)\x83V[\x15\x90Pa\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fwallet already takes a username\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 a\r>\x83\x82a*@V[P\x80`f\x83`@Qa\rP\x91\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\r\xB0\x90\x83\x90a)\xD6V[`@Q\x90\x81\x90\x03\x81 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7Fx\xB9h\xEA\xFD\xC5B\xFC`B\xF8\x85Jj\xE0\xAC\x05G\xF8\xBF\x82*\x83\xA9+>\xB8\xD0\xF9x!\x91\x90`\0\x90\xA3PPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`@Q3\x90\x81\x90`f\x90a\x0E\xE3\x90\x88\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid username\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`g` R`@\x90 T\x84\x10a\x0F\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Ftoo large nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[\x82B\x10a\x10\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Fexpired nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0a\x10%\x82\x86a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x10\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Finvalid nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x82\x81R`i` R`@\x90 T\x15a\x10\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fnonce already used for sign-in\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x82\x81R`i` R`@\x81 \x86\x90U[\x84Q\x81`\xFF\x16\x10\x15a\x11\xA3W\x84\x81`\xFF\x16\x81Q\x81\x10a\x11)Wa\x11)a+YV[` \x02` \x01\x01Q` \x01Q`j`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x87\x84`\xFF\x16\x81Q\x81\x10a\x11_Wa\x11_a+YV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 U\x80a\x11\x9B\x81a+\x88V[\x91PPa\x11\x08V[P\x85\x87`@Qa\x11\xB3\x91\x90a)\xD6V[`@Q\x80\x91\x03\x90 \x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFE\xDA\x1F\x8F\xC2\xB3GM_\xDC\x8BA\xA7]\xB0[\x04\x02i\x1E\xBB\x07\xD2h\"[\x8E\xC5v\x95\xA6|\x88\x88`@Qa\x12\x03\x92\x91\x90a+\xA7V[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[a\x12\x1Ca\x1C\x8BV[a\x12&`\0a\x1C\xF2V[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 \x80T``\x91\x90a\x12\\\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\x88\x90a)\x83V[\x80\x15a\x12\xD5W\x80`\x1F\x10a\x12\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`e` R`\0\x90\x81R`@\x90 \x80Ta\x12\xFA\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13&\x90a)\x83V[\x80\x15a\x13sW\x80`\x1F\x10a\x13HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13sV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13VW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xB5WP0;\x15\x80\x15a\x13\xB5WP`\0T`\xFF\x16`\x01\x14[a\x14'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x14\x85W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x14\x8Da\x1DiV[\x80\x15a\nBW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x83\x90R`<\x81 a\x15+\x90\x83a\x1D\xEEV[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[PPPPV[`\0\x80`\0a\x15\xBD\x85\x85a\x1AGV[`\0\x90\x81R`h` \x90\x81R`@\x80\x83 T`i\x90\x92R\x90\x91 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x93P\x91PP[\x92P\x92\x90PV[`\0\x80a\x16\n\x85\x85a\x1AGV[`\0\x90\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T\x91PP\x93\x92PPPV[`\0a\x16P3\x85a\x1AGV[`\0\x81\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x84R\x90\x91R\x90 T\x90\x91P\x82\x11\x15a\x16\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finsufficient token allowance\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x81\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x17\x11\x90\x84\x90a,\x18V[\x90\x91UPP`@Q\x84\x81R\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x903\x90\x7FO\x06\xDD\x7FNpP_\x9A\xBA\xBE\x01\\\x1E\x9E\x19\xD2\x83X\xFF\xDE7\x0F\xED\xF6\x93\xC4\xC2\xB9\xD4S\xA8\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPPV[a\x17sa\x1C\x8BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\nB\x81a\x1C\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x18hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Finvalid wallet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x18\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`g` R`@\x90 T\x81\x10a\x19?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Ftoo large nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0a\x19K\x84\x83a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14a\x19\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FepheAddr is not registered for t`D\x82\x01R\x7Fhe given wallet and nonce\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[`\0\x81\x81R`i` R`@\x90 TB\x10a\x15\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fexpired epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\x000``\x90\x81\x1B\x82\x16` \x84\x01RF`4\x84\x01R\x84\x90\x1B\x16`T\x82\x01R`h\x81\x01\x82\x90R`\0\x90`\x88\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a\nBa\x1C\x8BV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1A\xF3Wa\x1A\xEE\x83a\x1E\x12V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1BxWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1Bu\x91\x81\x01\x90a,+V[`\x01[a\x1B\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1C\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[Pa\x1A\xEE\x83\x83\x83a\x1F\x02V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x12&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x97V[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1D\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x12&a\x1F'V[`\0\x80`\0a\x1D\xFD\x85\x85a\x1F\xADV[\x91P\x91Pa\x1E\n\x81a\x1F\xEFV[P\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1E\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1F\x0B\x83a!TV[`\0\x82Q\x11\x80a\x1F\x18WP\x80[\x15a\x1A\xEEWa\x15\xA8\x83\x83a!\xA1V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1F\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x12&3a\x1C\xF2V[`\0\x80\x82Q`A\x03a\x1F\xE3W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1F\xD7\x87\x82\x85\x85a!\xCDV[\x94P\x94PPPPa\x15\xF6V[P`\0\x90P`\x02a\x15\xF6V[`\0\x81`\x04\x81\x11\x15a \x03Wa \x03a,DV[\x03a \x0BWPV[`\x01\x81`\x04\x81\x11\x15a \x1FWa \x1Fa,DV[\x03a lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\x02\x81`\x04\x81\x11\x15a \x80Wa \x80a,DV[\x03a \xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\x97V[`\x03\x81`\x04\x81\x11\x15a \xE1Wa \xE1a,DV[\x03a\nBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a!]\x81a\x1E\x12V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a!\xC6\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a,t`'\x919a\"\xBCV[\x93\x92PPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\x04WP`\0\x90P`\x03a\"\xB3V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"XW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\"\xACW`\0`\x01\x92P\x92PPa\"\xB3V[\x91P`\0\x90P[\x94P\x94\x92PPPV[```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\"\xE6\x91\x90a)\xD6V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a#!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#&V[``\x91P[P\x91P\x91Pa#7\x86\x83\x83\x87a#AV[\x96\x95PPPPPPV[``\x83\x15a#\xBDW\x82Q`\0\x03a#\xB6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\x97V[P\x81a#\xC7V[a#\xC7\x83\x83a#\xCFV[\x94\x93PPPPV[\x81Q\x15a#\xDFW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x97\x91\x90a'\x89V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\x1DW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$5W`\0\x80\xFD[a$>\x83a#\xF9V[\x91Pa$L` \x84\x01a#\xF9V[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xA7Wa$\xA7a$UV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF4Wa$\xF4a$UV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a%\rW`\0\x80\xFD[\x815` \x83\x01`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a%.Wa%.a$UV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a%a\x81a$\xADV[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a%vW`\0\x80\xFD[\x82\x82` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xA3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBAW`\0\x80\xFD[a#\xC7\x84\x82\x85\x01a$\xFCV[`\0` \x82\x84\x03\x12\x15a%\xD8W`\0\x80\xFD[a!\xC6\x82a#\xF9V[`\0` \x82\x84\x03\x12\x15a%\xF3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\rW`\0\x80\xFD[a&\x16\x83a#\xF9V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&2W`\0\x80\xFD[a&>\x85\x82\x86\x01a$\xFCV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&^W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&uW`\0\x80\xFD[a&\x81\x87\x82\x88\x01a$\xFCV[\x94PP` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xACW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a&\xBDW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xD7Wa&\xD7a$UV[a&\xE6` \x82`\x05\x1B\x01a$\xADV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a'\x08W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a'WW`@\x84\x8B\x03\x12\x15a''W`\0\x80\xFD[a'/a$\x84V[a'8\x85a#\xF9V[\x81R` \x85\x81\x015\x81\x83\x01R\x90\x83R`@\x90\x94\x01\x93\x91\x90\x91\x01\x90a'\x0FV[\x96\x99\x95\x98P\x93\x96PPPPPV[`\0[\x83\x81\x10\x15a'\x80W\x81\x81\x01Q\x83\x82\x01R` \x01a'hV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra'\xA8\x81`@\x85\x01` \x87\x01a'eV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xEFW`\0\x80\xFD[a'\xF8\x84a#\xF9V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1BW`\0\x80\xFD[a('\x86\x82\x87\x01a$\xFCV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a(DW`\0\x80\xFD[a(M\x83a#\xF9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(nW`\0\x80\xFD[\x825\x91Pa$L` \x84\x01a#\xF9V[`\0\x80`\0``\x84\x86\x03\x12\x15a(\x93W`\0\x80\xFD[a(\x9C\x84a#\xF9V[\x92P` \x84\x015\x91Pa(\xB1`@\x85\x01a#\xF9V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a(\xCFW`\0\x80\xFD[\x835\x92Pa(\xDF` \x85\x01a#\xF9V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x05W`\0\x80\xFD[a)\x0E\x84a#\xF9V[\x92Pa(\xDF` \x85\x01a#\xF9V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)|Wa)|a)\x1CV[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\x97W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)\xD0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa)\xE8\x81\x84` \x87\x01a'eV[\x91\x90\x91\x01\x92\x91PPV[`\x1F\x82\x11\x15a\x1A\xEEW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a*\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a*9W`\0\x81U`\x01\x01a*%V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*ZWa*Za$UV[a*n\x81a*h\x84Ta)\x83V[\x84a)\xF2V[` `\x1F\x82\x11`\x01\x81\x14a*\xC0W`\0\x83\x15a*\x8AWP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua*9V[`\0\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15a+\x0EW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a*\xEEV[P\x84\x82\x10\x15a+JW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x81\x03a+\x9EWa+\x9Ea)\x1CV[`\x01\x01\x92\x91PPV[`\0`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15a,\x0CW\x83Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a+\xCCV[P\x90\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x1A\xADWa\x1A\xADa)\x1CV[`\0` \x82\x84\x03\x12\x15a,=W`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 LK\xB63\xB6\xF6<\xAFi\xE6\0\xF2\xD9\xA1\xB8h\xF0\xF7\x7Fg5go/0\xDE&\x03z\x99\xA2\xC3dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static OAUTHCORE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x80W`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xD6W\x80c\xE9\x1D\xED,\x11a\0\x7FW\x80c\xF2\xFD\xE3\x8B\x11a\0YW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB5W\x80c\xF6CM\xA5\x14a\x04\xD5W\x80c\xFB\xFF\xA1k\x14a\x04\xF5W`\0\x80\xFD[\x80c\xE9\x1D\xED,\x14a\x04'W\x80c\xEB\x12\x02\xFF\x14a\x04uW\x80c\xF0\xE3\x84\xA5\x14a\x04\x95W`\0\x80\xFD[\x80c\xB5p:U\x11a\0\xB0W\x80c\xB5p:U\x14a\x03vW\x80c\xCFZ\xE5\x1D\x14a\x03\xC2W\x80c\xDBpF\xA5\x14a\x03\xFAW`\0\x80\xFD[\x80c\x81)\xFC\x1C\x14a\x03\x16W\x80c\x8D\xA5\xCB[\x14a\x03+W\x80c\x97\x16\x04\xC6\x14a\x03VW`\0\x80\xFD[\x80cQ\x9Ccw\x11a\x018W\x80cqP\x18\xA6\x11a\x01\x12W\x80cqP\x18\xA6\x14a\x02\xB4W\x80cr\x18\xFD\xA4\x14a\x02\xC9W\x80c\x80l\xD0j\x14a\x02\xF6W`\0\x80\xFD[\x80cQ\x9Ccw\x14a\x02_W\x80cR\xD1\x90-\x14a\x02\x7FW\x80c]\xCB\x7F\x88\x14a\x02\x94W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01iW\x80c6Y\xCF\xE6\x14a\x01\xF1W\x80c9\xB2QT\x14a\x02\x11W\x80cO\x1E\xF2\x86\x14a\x02LW`\0\x80\xFD[\x80c$a\x9A|\x14a\x01\x85W\x80c6M\xA6\xAF\x14a\x01\xA7W[`\0\x80\xFD[4\x80\x15a\x01\x91W`\0\x80\xFD[Pa\x01\xA5a\x01\xA06`\x04a$\"V[a\x058V[\0[4\x80\x15a\x01\xB3W`\0\x80\xFD[Pa\x01\xC7a\x01\xC26`\x04a%\x91V[a\x086V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFDW`\0\x80\xFD[Pa\x01\xA5a\x02\x0C6`\x04a%\xC6V[a\x08tV[4\x80\x15a\x02\x1DW`\0\x80\xFD[Pa\x02>a\x02,6`\x04a%\xE1V[`i` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01\xE8V[a\x01\xA5a\x02Z6`\x04a%\xFAV[a\nEV[4\x80\x15a\x02kW`\0\x80\xFD[Pa\x01\xA5a\x02z6`\x04a%\x91V[a\x0C\x07V[4\x80\x15a\x02\x8BW`\0\x80\xFD[Pa\x02>a\r\xFCV[4\x80\x15a\x02\xA0W`\0\x80\xFD[Pa\x01\xA5a\x02\xAF6`\x04a&HV[a\x0E\xCEV[4\x80\x15a\x02\xC0W`\0\x80\xFD[Pa\x01\xA5a\x12\x14V[4\x80\x15a\x02\xD5W`\0\x80\xFD[Pa\x02\xE9a\x02\xE46`\x04a%\xC6V[a\x12(V[`@Qa\x01\xE8\x91\x90a'\x89V[4\x80\x15a\x03\x02W`\0\x80\xFD[Pa\x02\xE9a\x03\x116`\x04a%\xC6V[a\x12\xE1V[4\x80\x15a\x03\"W`\0\x80\xFD[Pa\x01\xA5a\x13{V[4\x80\x15a\x037W`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC7V[4\x80\x15a\x03bW`\0\x80\xFD[Pa\x01\xA5a\x03q6`\x04a'\xDAV[a\x14\xF2V[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x03\x96a\x03\x916`\x04a(1V[a\x15\xAEV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE8V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x02>a\x03\xDD6`\x04a([V[`j` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04\x06W`\0\x80\xFD[Pa\x02>a\x04\x156`\x04a%\xC6V[`g` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x043W`\0\x80\xFD[Pa\x01\xC7a\x04B6`\x04a%\x91V[\x80Q` \x81\x83\x01\x81\x01\x80Q`f\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x02>a\x04\x906`\x04a(~V[a\x15\xFDV[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x01\xA5a\x04\xB06`\x04a(\xBAV[a\x16DV[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x01\xA5a\x04\xD06`\x04a%\xC6V[a\x17kV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x01\xA5a\x04\xF06`\x04a(\xF0V[a\x18\x05V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x01\xC7a\x05\x106`\x04a%\xE1V[`h` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Finvalid wallet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x06\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`g` R`@\x81 T\x90a\x064\x84\x83a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x06\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Fnonce already used\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x81\x81R`h` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x81\x16\x91\x90\x91\x17\x90\x91U\x87\x16\x83R`g\x90\x91R\x81 \x80T\x91a\x07\x14\x83a)KV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`e` R`@\x81 \x80Ta\x07I\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07u\x90a)\x83V[\x80\x15a\x07\xC2W\x80`\x1F\x10a\x07\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FC\x96\xA2\xE6\xE5\x14>Q\x99\xC0\xE1\xC6\x07\xB9\xDD\xE0^<%\x94r^\xF4;\xC7\xAAs9{b\xF2\x1F\x84`@Qa\x08'\x91\x90a'\x89V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\0`f\x82`@Qa\x08H\x91\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x94\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\n&\x81a\x1A\xB3V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\nB\x91\x83\x91\x90a\x1A\xBBV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0Be\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x0B\xF7\x82a\x1A\xB3V[a\x0C\x03\x82\x82`\x01a\x1A\xBBV[PPV[`@Q3\x90`\0\x90`f\x90a\x0C\x1D\x90\x85\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fusername already exists\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 \x80Ta\x0C\xC0\x90a)\x83V[\x15\x90Pa\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fwallet already takes a username\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 a\r>\x83\x82a*@V[P\x80`f\x83`@Qa\rP\x91\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\r\xB0\x90\x83\x90a)\xD6V[`@Q\x90\x81\x90\x03\x81 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7Fx\xB9h\xEA\xFD\xC5B\xFC`B\xF8\x85Jj\xE0\xAC\x05G\xF8\xBF\x82*\x83\xA9+>\xB8\xD0\xF9x!\x91\x90`\0\x90\xA3PPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`@Q3\x90\x81\x90`f\x90a\x0E\xE3\x90\x88\x90a)\xD6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid username\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`g` R`@\x90 T\x84\x10a\x0F\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Ftoo large nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[\x82B\x10a\x10\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Fexpired nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0a\x10%\x82\x86a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x10\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Finvalid nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x82\x81R`i` R`@\x90 T\x15a\x10\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fnonce already used for sign-in\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x82\x81R`i` R`@\x81 \x86\x90U[\x84Q\x81`\xFF\x16\x10\x15a\x11\xA3W\x84\x81`\xFF\x16\x81Q\x81\x10a\x11)Wa\x11)a+YV[` \x02` \x01\x01Q` \x01Q`j`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x87\x84`\xFF\x16\x81Q\x81\x10a\x11_Wa\x11_a+YV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 U\x80a\x11\x9B\x81a+\x88V[\x91PPa\x11\x08V[P\x85\x87`@Qa\x11\xB3\x91\x90a)\xD6V[`@Q\x80\x91\x03\x90 \x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFE\xDA\x1F\x8F\xC2\xB3GM_\xDC\x8BA\xA7]\xB0[\x04\x02i\x1E\xBB\x07\xD2h\"[\x8E\xC5v\x95\xA6|\x88\x88`@Qa\x12\x03\x92\x91\x90a+\xA7V[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[a\x12\x1Ca\x1C\x8BV[a\x12&`\0a\x1C\xF2V[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x90 \x80T``\x91\x90a\x12\\\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\x88\x90a)\x83V[\x80\x15a\x12\xD5W\x80`\x1F\x10a\x12\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`e` R`\0\x90\x81R`@\x90 \x80Ta\x12\xFA\x90a)\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13&\x90a)\x83V[\x80\x15a\x13sW\x80`\x1F\x10a\x13HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13sV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13VW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xB5WP0;\x15\x80\x15a\x13\xB5WP`\0T`\xFF\x16`\x01\x14[a\x14'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x14\x85W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x14\x8Da\x1DiV[\x80\x15a\nBW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x83\x90R`<\x81 a\x15+\x90\x83a\x1D\xEEV[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[PPPPV[`\0\x80`\0a\x15\xBD\x85\x85a\x1AGV[`\0\x90\x81R`h` \x90\x81R`@\x80\x83 T`i\x90\x92R\x90\x91 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x93P\x91PP[\x92P\x92\x90PV[`\0\x80a\x16\n\x85\x85a\x1AGV[`\0\x90\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T\x91PP\x93\x92PPPV[`\0a\x16P3\x85a\x1AGV[`\0\x81\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x84R\x90\x91R\x90 T\x90\x91P\x82\x11\x15a\x16\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finsufficient token allowance\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0\x81\x81R`j` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x17\x11\x90\x84\x90a,\x18V[\x90\x91UPP`@Q\x84\x81R\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x903\x90\x7FO\x06\xDD\x7FNpP_\x9A\xBA\xBE\x01\\\x1E\x9E\x19\xD2\x83X\xFF\xDE7\x0F\xED\xF6\x93\xC4\xC2\xB9\xD4S\xA8\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPPV[a\x17sa\x1C\x8BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\nB\x81a\x1C\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x18hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Finvalid wallet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x18\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Finvalid epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`g` R`@\x90 T\x81\x10a\x19?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Ftoo large nonce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\0a\x19K\x84\x83a\x1AGV[`\0\x81\x81R`h` R`@\x90 T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14a\x19\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FepheAddr is not registered for t`D\x82\x01R\x7Fhe given wallet and nonce\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[`\0\x81\x81R`i` R`@\x90 TB\x10a\x15\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fexpired epheAddr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\x000``\x90\x81\x1B\x82\x16` \x84\x01RF`4\x84\x01R\x84\x90\x1B\x16`T\x82\x01R`h\x81\x01\x82\x90R`\0\x90`\x88\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a\nBa\x1C\x8BV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1A\xF3Wa\x1A\xEE\x83a\x1E\x12V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1BxWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1Bu\x91\x81\x01\x90a,+V[`\x01[a\x1B\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1C\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[Pa\x1A\xEE\x83\x83\x83a\x1F\x02V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x12&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x97V[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1D\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x12&a\x1F'V[`\0\x80`\0a\x1D\xFD\x85\x85a\x1F\xADV[\x91P\x91Pa\x1E\n\x81a\x1F\xEFV[P\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1E\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1F\x0B\x83a!TV[`\0\x82Q\x11\x80a\x1F\x18WP\x80[\x15a\x1A\xEEWa\x15\xA8\x83\x83a!\xA1V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1F\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a\x12&3a\x1C\xF2V[`\0\x80\x82Q`A\x03a\x1F\xE3W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1F\xD7\x87\x82\x85\x85a!\xCDV[\x94P\x94PPPPa\x15\xF6V[P`\0\x90P`\x02a\x15\xF6V[`\0\x81`\x04\x81\x11\x15a \x03Wa \x03a,DV[\x03a \x0BWPV[`\x01\x81`\x04\x81\x11\x15a \x1FWa \x1Fa,DV[\x03a lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x97V[`\x02\x81`\x04\x81\x11\x15a \x80Wa \x80a,DV[\x03a \xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\x97V[`\x03\x81`\x04\x81\x11\x15a \xE1Wa \xE1a,DV[\x03a\nBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x97V[a!]\x81a\x1E\x12V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a!\xC6\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a,t`'\x919a\"\xBCV[\x93\x92PPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\x04WP`\0\x90P`\x03a\"\xB3V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"XW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\"\xACW`\0`\x01\x92P\x92PPa\"\xB3V[\x91P`\0\x90P[\x94P\x94\x92PPPV[```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\"\xE6\x91\x90a)\xD6V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a#!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#&V[``\x91P[P\x91P\x91Pa#7\x86\x83\x83\x87a#AV[\x96\x95PPPPPPV[``\x83\x15a#\xBDW\x82Q`\0\x03a#\xB6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\x97V[P\x81a#\xC7V[a#\xC7\x83\x83a#\xCFV[\x94\x93PPPPV[\x81Q\x15a#\xDFW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x97\x91\x90a'\x89V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\x1DW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$5W`\0\x80\xFD[a$>\x83a#\xF9V[\x91Pa$L` \x84\x01a#\xF9V[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xA7Wa$\xA7a$UV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF4Wa$\xF4a$UV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a%\rW`\0\x80\xFD[\x815` \x83\x01`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a%.Wa%.a$UV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a%a\x81a$\xADV[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a%vW`\0\x80\xFD[\x82\x82` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xA3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBAW`\0\x80\xFD[a#\xC7\x84\x82\x85\x01a$\xFCV[`\0` \x82\x84\x03\x12\x15a%\xD8W`\0\x80\xFD[a!\xC6\x82a#\xF9V[`\0` \x82\x84\x03\x12\x15a%\xF3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\rW`\0\x80\xFD[a&\x16\x83a#\xF9V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&2W`\0\x80\xFD[a&>\x85\x82\x86\x01a$\xFCV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&^W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&uW`\0\x80\xFD[a&\x81\x87\x82\x88\x01a$\xFCV[\x94PP` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xACW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a&\xBDW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xD7Wa&\xD7a$UV[a&\xE6` \x82`\x05\x1B\x01a$\xADV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a'\x08W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a'WW`@\x84\x8B\x03\x12\x15a''W`\0\x80\xFD[a'/a$\x84V[a'8\x85a#\xF9V[\x81R` \x85\x81\x015\x81\x83\x01R\x90\x83R`@\x90\x94\x01\x93\x91\x90\x91\x01\x90a'\x0FV[\x96\x99\x95\x98P\x93\x96PPPPPV[`\0[\x83\x81\x10\x15a'\x80W\x81\x81\x01Q\x83\x82\x01R` \x01a'hV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra'\xA8\x81`@\x85\x01` \x87\x01a'eV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xEFW`\0\x80\xFD[a'\xF8\x84a#\xF9V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1BW`\0\x80\xFD[a('\x86\x82\x87\x01a$\xFCV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a(DW`\0\x80\xFD[a(M\x83a#\xF9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a(nW`\0\x80\xFD[\x825\x91Pa$L` \x84\x01a#\xF9V[`\0\x80`\0``\x84\x86\x03\x12\x15a(\x93W`\0\x80\xFD[a(\x9C\x84a#\xF9V[\x92P` \x84\x015\x91Pa(\xB1`@\x85\x01a#\xF9V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a(\xCFW`\0\x80\xFD[\x835\x92Pa(\xDF` \x85\x01a#\xF9V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x05W`\0\x80\xFD[a)\x0E\x84a#\xF9V[\x92Pa(\xDF` \x85\x01a#\xF9V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)|Wa)|a)\x1CV[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\x97W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)\xD0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa)\xE8\x81\x84` \x87\x01a'eV[\x91\x90\x91\x01\x92\x91PPV[`\x1F\x82\x11\x15a\x1A\xEEW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a*\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a*9W`\0\x81U`\x01\x01a*%V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*ZWa*Za$UV[a*n\x81a*h\x84Ta)\x83V[\x84a)\xF2V[` `\x1F\x82\x11`\x01\x81\x14a*\xC0W`\0\x83\x15a*\x8AWP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua*9V[`\0\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15a+\x0EW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a*\xEEV[P\x84\x82\x10\x15a+JW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x81\x03a+\x9EWa+\x9Ea)\x1CV[`\x01\x01\x92\x91PPV[`\0`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15a,\x0CW\x83Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a+\xCCV[P\x90\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x1A\xADWa\x1A\xADa)\x1CV[`\0` \x82\x84\x03\x12\x15a,=W`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 LK\xB63\xB6\xF6<\xAFi\xE6\0\xF2\xD9\xA1\xB8h\xF0\xF7\x7Fg5go/0\xDE&\x03z\x99\xA2\xC3dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static OAUTHCORE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OauthCore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OauthCore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OauthCore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OauthCore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OauthCore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OauthCore))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OauthCore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                OAUTHCORE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                OAUTHCORE_ABI.clone(),
                OAUTHCORE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `epheAddrOfNonceHash` (0xfbffa16b) function
        pub fn ephe_addr_of_nonce_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([251, 255, 161, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expiryOfNonceHash` (0x39b25154) function
        pub fn expiry_of_nonce_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 178, 81, 84], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInfoOfWalletAndNonce` (0xb5703a55) function
        pub fn get_info_of_wallet_and_nonce(
            &self,
            wallet: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([181, 112, 58, 85], (wallet, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAllowancesOfWalletAndNonce` (0xeb1202ff) function
        pub fn get_token_allowances_of_wallet_and_nonce(
            &self,
            wallet: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            token_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 18, 2, 255], (wallet, nonce, token_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUsernameOfWallet` (0x7218fda4) function
        pub fn get_username_of_wallet(
            &self,
            wallet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 24, 253, 164], wallet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWalletOfUsername` (0x364da6af) function
        pub fn get_wallet_of_username(
            &self,
            username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([54, 77, 166, 175], username)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextNonceOfWallet` (0xdb7046a5) function
        pub fn next_nonce_of_wallet(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 112, 70, 165], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reduceTokenAllowance` (0xf0e384a5) function
        pub fn reduce_token_allowance(
            &self,
            nonce: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 227, 132, 165], (nonce, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerEpheAddr` (0x24619a7c) function
        pub fn register_ephe_addr(
            &self,
            wallet: ::ethers::core::types::Address,
            ephe_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 97, 154, 124], (wallet, ephe_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signin` (0x5dcb7f88) function
        pub fn signin(
            &self,
            username: ::std::string::String,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            token_allowances: ::std::vec::Vec<TokenAllowance>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [93, 203, 127, 136],
                    (username, nonce, expiry, token_allowances),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signup` (0x519c6377) function
        pub fn signup(
            &self,
            username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 156, 99, 119], username)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenAllowancesOfNonceHash` (0xcf5ae51d) function
        pub fn token_allowances_of_nonce_hash(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 90, 229, 29], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usernameOfWallet` (0x806cd06a) function
        pub fn username_of_wallet(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([128, 108, 208, 106], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateEpheAddr` (0xf6434da5) function
        pub fn validate_ephe_addr(
            &self,
            wallet: ::ethers::core::types::Address,
            ephe_addr: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 67, 77, 165], (wallet, ephe_addr, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSignature` (0x971604c6) function
        pub fn validate_signature(
            &self,
            ephe_addr: ::ethers::core::types::Address,
            hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 22, 4, 198], (ephe_addr, hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `walletOfUsername` (0xe91ded2c) function
        pub fn wallet_of_username(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([233, 29, 237, 44], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AdminChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BeaconUpgradedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ReducedTokenAllowance` event
        pub fn reduced_token_allowance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ReducedTokenAllowanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RegisteredEpheAddr` event
        pub fn registered_ephe_addr_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisteredEpheAddrFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Signin` event
        pub fn signin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SigninFilter> {
            self.0.event()
        }
        ///Gets the contract's `Signup` event
        pub fn signup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignupFilter> {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OauthCoreEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for OauthCore<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ReducedTokenAllowance",
        abi = "ReducedTokenAllowance(address,address,uint256,uint256)"
    )]
    pub struct ReducedTokenAllowanceFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RegisteredEpheAddr",
        abi = "RegisteredEpheAddr(address,address,uint256,string)"
    )]
    pub struct RegisteredEpheAddrFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ephe_addr: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
        pub username: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Signin",
        abi = "Signin(address,string,uint256,uint256,(address,uint256)[])"
    )]
    pub struct SigninFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub username: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub token_allowances: ::std::vec::Vec<TokenAllowance>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Signup", abi = "Signup(address,string)")]
    pub struct SignupFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub username: ::ethers::core::types::H256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OauthCoreEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ReducedTokenAllowanceFilter(ReducedTokenAllowanceFilter),
        RegisteredEpheAddrFilter(RegisteredEpheAddrFilter),
        SigninFilter(SigninFilter),
        SignupFilter(SignupFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for OauthCoreEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(OauthCoreEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(OauthCoreEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OauthCoreEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OauthCoreEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ReducedTokenAllowanceFilter::decode_log(log) {
                return Ok(OauthCoreEvents::ReducedTokenAllowanceFilter(decoded));
            }
            if let Ok(decoded) = RegisteredEpheAddrFilter::decode_log(log) {
                return Ok(OauthCoreEvents::RegisteredEpheAddrFilter(decoded));
            }
            if let Ok(decoded) = SigninFilter::decode_log(log) {
                return Ok(OauthCoreEvents::SigninFilter(decoded));
            }
            if let Ok(decoded) = SignupFilter::decode_log(log) {
                return Ok(OauthCoreEvents::SignupFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(OauthCoreEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OauthCoreEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeaconUpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReducedTokenAllowanceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredEpheAddrFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SigninFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignupFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for OauthCoreEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for OauthCoreEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for OauthCoreEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for OauthCoreEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ReducedTokenAllowanceFilter> for OauthCoreEvents {
        fn from(value: ReducedTokenAllowanceFilter) -> Self {
            Self::ReducedTokenAllowanceFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredEpheAddrFilter> for OauthCoreEvents {
        fn from(value: RegisteredEpheAddrFilter) -> Self {
            Self::RegisteredEpheAddrFilter(value)
        }
    }
    impl ::core::convert::From<SigninFilter> for OauthCoreEvents {
        fn from(value: SigninFilter) -> Self {
            Self::SigninFilter(value)
        }
    }
    impl ::core::convert::From<SignupFilter> for OauthCoreEvents {
        fn from(value: SignupFilter) -> Self {
            Self::SignupFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for OauthCoreEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `epheAddrOfNonceHash` function with signature `epheAddrOfNonceHash(bytes32)` and selector `0xfbffa16b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "epheAddrOfNonceHash", abi = "epheAddrOfNonceHash(bytes32)")]
    pub struct EpheAddrOfNonceHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `expiryOfNonceHash` function with signature `expiryOfNonceHash(bytes32)` and selector `0x39b25154`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "expiryOfNonceHash", abi = "expiryOfNonceHash(bytes32)")]
    pub struct ExpiryOfNonceHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `getInfoOfWalletAndNonce` function with signature `getInfoOfWalletAndNonce(address,uint256)` and selector `0xb5703a55`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getInfoOfWalletAndNonce",
        abi = "getInfoOfWalletAndNonce(address,uint256)"
    )]
    pub struct GetInfoOfWalletAndNonceCall {
        pub wallet: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTokenAllowancesOfWalletAndNonce` function with signature `getTokenAllowancesOfWalletAndNonce(address,uint256,address)` and selector `0xeb1202ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getTokenAllowancesOfWalletAndNonce",
        abi = "getTokenAllowancesOfWalletAndNonce(address,uint256,address)"
    )]
    pub struct GetTokenAllowancesOfWalletAndNonceCall {
        pub wallet: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub token_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUsernameOfWallet` function with signature `getUsernameOfWallet(address)` and selector `0x7218fda4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getUsernameOfWallet", abi = "getUsernameOfWallet(address)")]
    pub struct GetUsernameOfWalletCall {
        pub wallet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getWalletOfUsername` function with signature `getWalletOfUsername(string)` and selector `0x364da6af`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getWalletOfUsername", abi = "getWalletOfUsername(string)")]
    pub struct GetWalletOfUsernameCall {
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `nextNonceOfWallet` function with signature `nextNonceOfWallet(address)` and selector `0xdb7046a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nextNonceOfWallet", abi = "nextNonceOfWallet(address)")]
    pub struct NextNonceOfWalletCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `reduceTokenAllowance` function with signature `reduceTokenAllowance(uint256,address,uint256)` and selector `0xf0e384a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "reduceTokenAllowance",
        abi = "reduceTokenAllowance(uint256,address,uint256)"
    )]
    pub struct ReduceTokenAllowanceCall {
        pub nonce: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerEpheAddr` function with signature `registerEpheAddr(address,address)` and selector `0x24619a7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registerEpheAddr", abi = "registerEpheAddr(address,address)")]
    pub struct RegisterEpheAddrCall {
        pub wallet: ::ethers::core::types::Address,
        pub ephe_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `signin` function with signature `signin(string,uint256,uint256,(address,uint256)[])` and selector `0x5dcb7f88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signin",
        abi = "signin(string,uint256,uint256,(address,uint256)[])"
    )]
    pub struct SigninCall {
        pub username: ::std::string::String,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub token_allowances: ::std::vec::Vec<TokenAllowance>,
    }
    ///Container type for all input parameters for the `signup` function with signature `signup(string)` and selector `0x519c6377`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "signup", abi = "signup(string)")]
    pub struct SignupCall {
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenAllowancesOfNonceHash` function with signature `tokenAllowancesOfNonceHash(bytes32,address)` and selector `0xcf5ae51d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "tokenAllowancesOfNonceHash",
        abi = "tokenAllowancesOfNonceHash(bytes32,address)"
    )]
    pub struct TokenAllowancesOfNonceHashCall(pub [u8; 32], pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `usernameOfWallet` function with signature `usernameOfWallet(address)` and selector `0x806cd06a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "usernameOfWallet", abi = "usernameOfWallet(address)")]
    pub struct UsernameOfWalletCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `validateEpheAddr` function with signature `validateEpheAddr(address,address,uint256)` and selector `0xf6434da5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "validateEpheAddr",
        abi = "validateEpheAddr(address,address,uint256)"
    )]
    pub struct ValidateEpheAddrCall {
        pub wallet: ::ethers::core::types::Address,
        pub ephe_addr: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `validateSignature` function with signature `validateSignature(address,bytes32,bytes)` and selector `0x971604c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "validateSignature",
        abi = "validateSignature(address,bytes32,bytes)"
    )]
    pub struct ValidateSignatureCall {
        pub ephe_addr: ::ethers::core::types::Address,
        pub hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `walletOfUsername` function with signature `walletOfUsername(string)` and selector `0xe91ded2c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "walletOfUsername", abi = "walletOfUsername(string)")]
    pub struct WalletOfUsernameCall(pub ::std::string::String);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OauthCoreCalls {
        EpheAddrOfNonceHash(EpheAddrOfNonceHashCall),
        ExpiryOfNonceHash(ExpiryOfNonceHashCall),
        GetInfoOfWalletAndNonce(GetInfoOfWalletAndNonceCall),
        GetTokenAllowancesOfWalletAndNonce(GetTokenAllowancesOfWalletAndNonceCall),
        GetUsernameOfWallet(GetUsernameOfWalletCall),
        GetWalletOfUsername(GetWalletOfUsernameCall),
        Initialize(InitializeCall),
        NextNonceOfWallet(NextNonceOfWalletCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        ReduceTokenAllowance(ReduceTokenAllowanceCall),
        RegisterEpheAddr(RegisterEpheAddrCall),
        RenounceOwnership(RenounceOwnershipCall),
        Signin(SigninCall),
        Signup(SignupCall),
        TokenAllowancesOfNonceHash(TokenAllowancesOfNonceHashCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        UsernameOfWallet(UsernameOfWalletCall),
        ValidateEpheAddr(ValidateEpheAddrCall),
        ValidateSignature(ValidateSignatureCall),
        WalletOfUsername(WalletOfUsernameCall),
    }
    impl ::ethers::core::abi::AbiDecode for OauthCoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EpheAddrOfNonceHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EpheAddrOfNonceHash(decoded));
            }
            if let Ok(decoded) =
                <ExpiryOfNonceHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExpiryOfNonceHash(decoded));
            }
            if let Ok(decoded) =
                <GetInfoOfWalletAndNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInfoOfWalletAndNonce(decoded));
            }
            if let Ok(decoded) =
                <GetTokenAllowancesOfWalletAndNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetTokenAllowancesOfWalletAndNonce(decoded));
            }
            if let Ok(decoded) =
                <GetUsernameOfWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUsernameOfWallet(decoded));
            }
            if let Ok(decoded) =
                <GetWalletOfUsernameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetWalletOfUsername(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <NextNonceOfWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextNonceOfWallet(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <ReduceTokenAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReduceTokenAllowance(decoded));
            }
            if let Ok(decoded) =
                <RegisterEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterEpheAddr(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SigninCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Signin(decoded));
            }
            if let Ok(decoded) = <SignupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Signup(decoded));
            }
            if let Ok(decoded) =
                <TokenAllowancesOfNonceHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenAllowancesOfNonceHash(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <UsernameOfWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UsernameOfWallet(decoded));
            }
            if let Ok(decoded) =
                <ValidateEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateEpheAddr(decoded));
            }
            if let Ok(decoded) =
                <ValidateSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSignature(decoded));
            }
            if let Ok(decoded) =
                <WalletOfUsernameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WalletOfUsername(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OauthCoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EpheAddrOfNonceHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpiryOfNonceHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInfoOfWalletAndNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAllowancesOfWalletAndNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUsernameOfWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletOfUsername(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextNonceOfWallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReduceTokenAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterEpheAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Signin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Signup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAllowancesOfNonceHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UsernameOfWallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateEpheAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WalletOfUsername(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OauthCoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EpheAddrOfNonceHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpiryOfNonceHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInfoOfWalletAndNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenAllowancesOfWalletAndNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUsernameOfWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWalletOfUsername(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextNonceOfWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReduceTokenAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signup(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAllowancesOfNonceHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsernameOfWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletOfUsername(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EpheAddrOfNonceHashCall> for OauthCoreCalls {
        fn from(value: EpheAddrOfNonceHashCall) -> Self {
            Self::EpheAddrOfNonceHash(value)
        }
    }
    impl ::core::convert::From<ExpiryOfNonceHashCall> for OauthCoreCalls {
        fn from(value: ExpiryOfNonceHashCall) -> Self {
            Self::ExpiryOfNonceHash(value)
        }
    }
    impl ::core::convert::From<GetInfoOfWalletAndNonceCall> for OauthCoreCalls {
        fn from(value: GetInfoOfWalletAndNonceCall) -> Self {
            Self::GetInfoOfWalletAndNonce(value)
        }
    }
    impl ::core::convert::From<GetTokenAllowancesOfWalletAndNonceCall> for OauthCoreCalls {
        fn from(value: GetTokenAllowancesOfWalletAndNonceCall) -> Self {
            Self::GetTokenAllowancesOfWalletAndNonce(value)
        }
    }
    impl ::core::convert::From<GetUsernameOfWalletCall> for OauthCoreCalls {
        fn from(value: GetUsernameOfWalletCall) -> Self {
            Self::GetUsernameOfWallet(value)
        }
    }
    impl ::core::convert::From<GetWalletOfUsernameCall> for OauthCoreCalls {
        fn from(value: GetWalletOfUsernameCall) -> Self {
            Self::GetWalletOfUsername(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OauthCoreCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NextNonceOfWalletCall> for OauthCoreCalls {
        fn from(value: NextNonceOfWalletCall) -> Self {
            Self::NextNonceOfWallet(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for OauthCoreCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for OauthCoreCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<ReduceTokenAllowanceCall> for OauthCoreCalls {
        fn from(value: ReduceTokenAllowanceCall) -> Self {
            Self::ReduceTokenAllowance(value)
        }
    }
    impl ::core::convert::From<RegisterEpheAddrCall> for OauthCoreCalls {
        fn from(value: RegisterEpheAddrCall) -> Self {
            Self::RegisterEpheAddr(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for OauthCoreCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SigninCall> for OauthCoreCalls {
        fn from(value: SigninCall) -> Self {
            Self::Signin(value)
        }
    }
    impl ::core::convert::From<SignupCall> for OauthCoreCalls {
        fn from(value: SignupCall) -> Self {
            Self::Signup(value)
        }
    }
    impl ::core::convert::From<TokenAllowancesOfNonceHashCall> for OauthCoreCalls {
        fn from(value: TokenAllowancesOfNonceHashCall) -> Self {
            Self::TokenAllowancesOfNonceHash(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for OauthCoreCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for OauthCoreCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for OauthCoreCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<UsernameOfWalletCall> for OauthCoreCalls {
        fn from(value: UsernameOfWalletCall) -> Self {
            Self::UsernameOfWallet(value)
        }
    }
    impl ::core::convert::From<ValidateEpheAddrCall> for OauthCoreCalls {
        fn from(value: ValidateEpheAddrCall) -> Self {
            Self::ValidateEpheAddr(value)
        }
    }
    impl ::core::convert::From<ValidateSignatureCall> for OauthCoreCalls {
        fn from(value: ValidateSignatureCall) -> Self {
            Self::ValidateSignature(value)
        }
    }
    impl ::core::convert::From<WalletOfUsernameCall> for OauthCoreCalls {
        fn from(value: WalletOfUsernameCall) -> Self {
            Self::WalletOfUsername(value)
        }
    }
    ///Container type for all return fields from the `epheAddrOfNonceHash` function with signature `epheAddrOfNonceHash(bytes32)` and selector `0xfbffa16b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EpheAddrOfNonceHashReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `expiryOfNonceHash` function with signature `expiryOfNonceHash(bytes32)` and selector `0x39b25154`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExpiryOfNonceHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getInfoOfWalletAndNonce` function with signature `getInfoOfWalletAndNonce(address,uint256)` and selector `0xb5703a55`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetInfoOfWalletAndNonceReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getTokenAllowancesOfWalletAndNonce` function with signature `getTokenAllowancesOfWalletAndNonce(address,uint256,address)` and selector `0xeb1202ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokenAllowancesOfWalletAndNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUsernameOfWallet` function with signature `getUsernameOfWallet(address)` and selector `0x7218fda4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUsernameOfWalletReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getWalletOfUsername` function with signature `getWalletOfUsername(string)` and selector `0x364da6af`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetWalletOfUsernameReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nextNonceOfWallet` function with signature `nextNonceOfWallet(address)` and selector `0xdb7046a5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NextNonceOfWalletReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `tokenAllowancesOfNonceHash` function with signature `tokenAllowancesOfNonceHash(bytes32,address)` and selector `0xcf5ae51d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenAllowancesOfNonceHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `usernameOfWallet` function with signature `usernameOfWallet(address)` and selector `0x806cd06a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UsernameOfWalletReturn(pub ::std::string::String);
    ///Container type for all return fields from the `walletOfUsername` function with signature `walletOfUsername(string)` and selector `0xe91ded2c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WalletOfUsernameReturn(pub ::ethers::core::types::Address);
    ///`TokenAllowance(address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenAllowance {
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
