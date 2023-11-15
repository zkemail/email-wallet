pub use email_wallet_core::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod email_wallet_core {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_relayerHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_accountHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_unclaimsHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_extensionHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_priceOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_wethContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_maxFeePerGas"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_emailValidityDuration"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_unclaimedFundClaimGas"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_unclaimedStateClaimGas",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accountHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accountHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract AccountHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositTokenAsExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositTokenAsExtension",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emailNullifiers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emailNullifiers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emailValidityDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "emailValidityDuration",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeAsExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeAsExtension"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("extensionHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("extensionHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ExtensionHandler",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("handleEmailOp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("handleEmailOp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailOp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EmailOp"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("err"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalFeeInETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registeredUnclaimId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultExtensions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxFeePerGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFeePerGas"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPriceOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "registerUnclaimedStateAsExtension",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerUnclaimedStateAsExtension",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensionAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relayerHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relayerHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract RelayerHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestTokenAsExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestTokenAsExtension",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract TokenRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unclaimedFundClaimGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "unclaimedFundClaimGas",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unclaimedStateClaimGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "unclaimedStateClaimGas",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unclaimsHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unclaimsHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract UnclaimsHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateEmailOp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateEmailOp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailOp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EmailOp"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVerifier"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wethContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wethContract"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EmailOpHandled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EmailOpHandled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registeredUnclaimId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("emailNullifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("emailAddrPointer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "recipientEmailAddrCommit",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipientETHAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("err"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EMAILWALLETCORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct EmailWalletCore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EmailWalletCore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EmailWalletCore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EmailWalletCore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EmailWalletCore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EmailWalletCore))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EmailWalletCore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EMAILWALLETCORE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accountHandler` (0x33ddfb9a) function
        pub fn account_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([51, 221, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositTokenAsExtension` (0x75af3217) function
        pub fn deposit_token_as_extension(
            &self,
            token_addr: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 175, 50, 23], (token_addr, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emailNullifiers` (0x17ace6b5) function
        pub fn email_nullifiers(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 172, 230, 181], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emailValidityDuration` (0xb90d36f9) function
        pub fn email_validity_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 13, 54, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeAsExtension` (0x21d71c8f) function
        pub fn execute_as_extension(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 215, 28, 143], (target, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extensionHandler` (0x9f6d1d83) function
        pub fn extension_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 109, 29, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleEmailOp` (0x1377df44) function
        pub fn handle_email_op(
            &self,
            email_op: EmailOp,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([19, 119, 223, 68], (email_op,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x6f2de70e) function
        pub fn initialize(
            &self,
            default_extensions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 45, 231, 14], default_extensions)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxFeePerGas` (0x2728bf2c) function
        pub fn max_fee_per_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 40, 191, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceOracle` (0x2630c12f) function
        pub fn price_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 48, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerUnclaimedStateAsExtension` (0xc9af3f1c) function
        pub fn register_unclaimed_state_as_extension(
            &self,
            extension_addr: ::ethers::core::types::Address,
            state: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 175, 63, 28], (extension_addr, state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerHandler` (0x5e5f2610) function
        pub fn relayer_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 95, 38, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestTokenAsExtension` (0x65a345ac) function
        pub fn request_token_as_extension(
            &self,
            token_addr: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 163, 69, 172], (token_addr, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenRegistry` (0x9d23c4c7) function
        pub fn token_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 35, 196, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unclaimedFundClaimGas` (0x66221734) function
        pub fn unclaimed_fund_claim_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 34, 23, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unclaimedStateClaimGas` (0xa87fedae) function
        pub fn unclaimed_state_claim_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 127, 237, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unclaimsHandler` (0xd5e5c1c8) function
        pub fn unclaims_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([213, 229, 193, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateEmailOp` (0xa3964475) function
        pub fn validate_email_op(
            &self,
            email_op: EmailOp,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 150, 68, 117], (email_op,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifier` (0x2b7ac3f3) function
        pub fn verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wethContract` (0x4780eac1) function
        pub fn weth_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([71, 128, 234, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EmailOpHandled` event
        pub fn email_op_handled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EmailOpHandledFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EmailOpHandledFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EmailWalletCore<M> {
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
        Hash
    )]
    #[ethevent(
        name = "EmailOpHandled",
        abi = "EmailOpHandled(bool,uint256,bytes32,bytes32,bytes32,address,bytes)"
    )]
    pub struct EmailOpHandledFilter {
        #[ethevent(indexed)]
        pub success: bool,
        #[ethevent(indexed)]
        pub registered_unclaim_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub email_nullifier: [u8; 32],
        pub email_addr_pointer: [u8; 32],
        pub recipient_email_addr_commit: [u8; 32],
        pub recipient_eth_addr: ::ethers::core::types::Address,
        pub err: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `accountHandler` function with signature `accountHandler()` and selector `0x33ddfb9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accountHandler", abi = "accountHandler()")]
    pub struct AccountHandlerCall;
    ///Container type for all input parameters for the `depositTokenAsExtension` function with signature `depositTokenAsExtension(address,uint256)` and selector `0x75af3217`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "depositTokenAsExtension",
        abi = "depositTokenAsExtension(address,uint256)"
    )]
    pub struct DepositTokenAsExtensionCall {
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emailNullifiers` function with signature `emailNullifiers(bytes32)` and selector `0x17ace6b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "emailNullifiers", abi = "emailNullifiers(bytes32)")]
    pub struct EmailNullifiersCall(pub [u8; 32]);
    ///Container type for all input parameters for the `emailValidityDuration` function with signature `emailValidityDuration()` and selector `0xb90d36f9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "emailValidityDuration", abi = "emailValidityDuration()")]
    pub struct EmailValidityDurationCall;
    ///Container type for all input parameters for the `executeAsExtension` function with signature `executeAsExtension(address,bytes)` and selector `0x21d71c8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeAsExtension", abi = "executeAsExtension(address,bytes)")]
    pub struct ExecuteAsExtensionCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `extensionHandler` function with signature `extensionHandler()` and selector `0x9f6d1d83`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "extensionHandler", abi = "extensionHandler()")]
    pub struct ExtensionHandlerCall;
    ///Container type for all input parameters for the `handleEmailOp` function with signature `handleEmailOp((bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes))` and selector `0x1377df44`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "handleEmailOp",
        abi = "handleEmailOp((bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes))"
    )]
    pub struct HandleEmailOpCall {
        pub email_op: EmailOp,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(bytes[])` and selector `0x6f2de70e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(bytes[])")]
    pub struct InitializeCall {
        pub default_extensions: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `maxFeePerGas` function with signature `maxFeePerGas()` and selector `0x2728bf2c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxFeePerGas", abi = "maxFeePerGas()")]
    pub struct MaxFeePerGasCall;
    ///Container type for all input parameters for the `priceOracle` function with signature `priceOracle()` and selector `0x2630c12f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "priceOracle", abi = "priceOracle()")]
    pub struct PriceOracleCall;
    ///Container type for all input parameters for the `registerUnclaimedStateAsExtension` function with signature `registerUnclaimedStateAsExtension(address,bytes)` and selector `0xc9af3f1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "registerUnclaimedStateAsExtension",
        abi = "registerUnclaimedStateAsExtension(address,bytes)"
    )]
    pub struct RegisterUnclaimedStateAsExtensionCall {
        pub extension_addr: ::ethers::core::types::Address,
        pub state: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `relayerHandler` function with signature `relayerHandler()` and selector `0x5e5f2610`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "relayerHandler", abi = "relayerHandler()")]
    pub struct RelayerHandlerCall;
    ///Container type for all input parameters for the `requestTokenAsExtension` function with signature `requestTokenAsExtension(address,uint256)` and selector `0x65a345ac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "requestTokenAsExtension",
        abi = "requestTokenAsExtension(address,uint256)"
    )]
    pub struct RequestTokenAsExtensionCall {
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenRegistry` function with signature `tokenRegistry()` and selector `0x9d23c4c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenRegistry", abi = "tokenRegistry()")]
    pub struct TokenRegistryCall;
    ///Container type for all input parameters for the `unclaimedFundClaimGas` function with signature `unclaimedFundClaimGas()` and selector `0x66221734`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unclaimedFundClaimGas", abi = "unclaimedFundClaimGas()")]
    pub struct UnclaimedFundClaimGasCall;
    ///Container type for all input parameters for the `unclaimedStateClaimGas` function with signature `unclaimedStateClaimGas()` and selector `0xa87fedae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unclaimedStateClaimGas", abi = "unclaimedStateClaimGas()")]
    pub struct UnclaimedStateClaimGasCall;
    ///Container type for all input parameters for the `unclaimsHandler` function with signature `unclaimsHandler()` and selector `0xd5e5c1c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unclaimsHandler", abi = "unclaimsHandler()")]
    pub struct UnclaimsHandlerCall;
    ///Container type for all input parameters for the `validateEmailOp` function with signature `validateEmailOp((bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes))` and selector `0xa3964475`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "validateEmailOp",
        abi = "validateEmailOp((bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes))"
    )]
    pub struct ValidateEmailOpCall {
        pub email_op: EmailOp,
    }
    ///Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    ///Container type for all input parameters for the `wethContract` function with signature `wethContract()` and selector `0x4780eac1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "wethContract", abi = "wethContract()")]
    pub struct WethContractCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum EmailWalletCoreCalls {
        AccountHandler(AccountHandlerCall),
        DepositTokenAsExtension(DepositTokenAsExtensionCall),
        EmailNullifiers(EmailNullifiersCall),
        EmailValidityDuration(EmailValidityDurationCall),
        ExecuteAsExtension(ExecuteAsExtensionCall),
        ExtensionHandler(ExtensionHandlerCall),
        HandleEmailOp(HandleEmailOpCall),
        Initialize(InitializeCall),
        MaxFeePerGas(MaxFeePerGasCall),
        PriceOracle(PriceOracleCall),
        RegisterUnclaimedStateAsExtension(RegisterUnclaimedStateAsExtensionCall),
        RelayerHandler(RelayerHandlerCall),
        RequestTokenAsExtension(RequestTokenAsExtensionCall),
        TokenRegistry(TokenRegistryCall),
        UnclaimedFundClaimGas(UnclaimedFundClaimGasCall),
        UnclaimedStateClaimGas(UnclaimedStateClaimGasCall),
        UnclaimsHandler(UnclaimsHandlerCall),
        ValidateEmailOp(ValidateEmailOpCall),
        Verifier(VerifierCall),
        WethContract(WethContractCall),
    }
    impl ::ethers::core::abi::AbiDecode for EmailWalletCoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountHandler(decoded));
            }
            if let Ok(decoded) = <DepositTokenAsExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositTokenAsExtension(decoded));
            }
            if let Ok(decoded) = <EmailNullifiersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmailNullifiers(decoded));
            }
            if let Ok(decoded) = <EmailValidityDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmailValidityDuration(decoded));
            }
            if let Ok(decoded) = <ExecuteAsExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteAsExtension(decoded));
            }
            if let Ok(decoded) = <ExtensionHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExtensionHandler(decoded));
            }
            if let Ok(decoded) = <HandleEmailOpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HandleEmailOp(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MaxFeePerGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxFeePerGas(decoded));
            }
            if let Ok(decoded) = <PriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PriceOracle(decoded));
            }
            if let Ok(decoded) = <RegisterUnclaimedStateAsExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterUnclaimedStateAsExtension(decoded));
            }
            if let Ok(decoded) = <RelayerHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayerHandler(decoded));
            }
            if let Ok(decoded) = <RequestTokenAsExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestTokenAsExtension(decoded));
            }
            if let Ok(decoded) = <TokenRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenRegistry(decoded));
            }
            if let Ok(decoded) = <UnclaimedFundClaimGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnclaimedFundClaimGas(decoded));
            }
            if let Ok(decoded) = <UnclaimedStateClaimGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnclaimedStateClaimGas(decoded));
            }
            if let Ok(decoded) = <UnclaimsHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnclaimsHandler(decoded));
            }
            if let Ok(decoded) = <ValidateEmailOpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateEmailOp(decoded));
            }
            if let Ok(decoded) = <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verifier(decoded));
            }
            if let Ok(decoded) = <WethContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WethContract(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EmailWalletCoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositTokenAsExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailNullifiers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailValidityDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteAsExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtensionHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HandleEmailOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFeePerGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterUnclaimedStateAsExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayerHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestTokenAsExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnclaimedFundClaimGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnclaimedStateClaimGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnclaimsHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateEmailOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WethContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EmailWalletCoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositTokenAsExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmailNullifiers(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmailValidityDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteAsExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExtensionHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleEmailOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFeePerGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterUnclaimedStateAsExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayerHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestTokenAsExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnclaimedFundClaimGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnclaimedStateClaimGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnclaimsHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateEmailOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::WethContract(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountHandlerCall> for EmailWalletCoreCalls {
        fn from(value: AccountHandlerCall) -> Self {
            Self::AccountHandler(value)
        }
    }
    impl ::core::convert::From<DepositTokenAsExtensionCall> for EmailWalletCoreCalls {
        fn from(value: DepositTokenAsExtensionCall) -> Self {
            Self::DepositTokenAsExtension(value)
        }
    }
    impl ::core::convert::From<EmailNullifiersCall> for EmailWalletCoreCalls {
        fn from(value: EmailNullifiersCall) -> Self {
            Self::EmailNullifiers(value)
        }
    }
    impl ::core::convert::From<EmailValidityDurationCall> for EmailWalletCoreCalls {
        fn from(value: EmailValidityDurationCall) -> Self {
            Self::EmailValidityDuration(value)
        }
    }
    impl ::core::convert::From<ExecuteAsExtensionCall> for EmailWalletCoreCalls {
        fn from(value: ExecuteAsExtensionCall) -> Self {
            Self::ExecuteAsExtension(value)
        }
    }
    impl ::core::convert::From<ExtensionHandlerCall> for EmailWalletCoreCalls {
        fn from(value: ExtensionHandlerCall) -> Self {
            Self::ExtensionHandler(value)
        }
    }
    impl ::core::convert::From<HandleEmailOpCall> for EmailWalletCoreCalls {
        fn from(value: HandleEmailOpCall) -> Self {
            Self::HandleEmailOp(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EmailWalletCoreCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MaxFeePerGasCall> for EmailWalletCoreCalls {
        fn from(value: MaxFeePerGasCall) -> Self {
            Self::MaxFeePerGas(value)
        }
    }
    impl ::core::convert::From<PriceOracleCall> for EmailWalletCoreCalls {
        fn from(value: PriceOracleCall) -> Self {
            Self::PriceOracle(value)
        }
    }
    impl ::core::convert::From<RegisterUnclaimedStateAsExtensionCall>
    for EmailWalletCoreCalls {
        fn from(value: RegisterUnclaimedStateAsExtensionCall) -> Self {
            Self::RegisterUnclaimedStateAsExtension(value)
        }
    }
    impl ::core::convert::From<RelayerHandlerCall> for EmailWalletCoreCalls {
        fn from(value: RelayerHandlerCall) -> Self {
            Self::RelayerHandler(value)
        }
    }
    impl ::core::convert::From<RequestTokenAsExtensionCall> for EmailWalletCoreCalls {
        fn from(value: RequestTokenAsExtensionCall) -> Self {
            Self::RequestTokenAsExtension(value)
        }
    }
    impl ::core::convert::From<TokenRegistryCall> for EmailWalletCoreCalls {
        fn from(value: TokenRegistryCall) -> Self {
            Self::TokenRegistry(value)
        }
    }
    impl ::core::convert::From<UnclaimedFundClaimGasCall> for EmailWalletCoreCalls {
        fn from(value: UnclaimedFundClaimGasCall) -> Self {
            Self::UnclaimedFundClaimGas(value)
        }
    }
    impl ::core::convert::From<UnclaimedStateClaimGasCall> for EmailWalletCoreCalls {
        fn from(value: UnclaimedStateClaimGasCall) -> Self {
            Self::UnclaimedStateClaimGas(value)
        }
    }
    impl ::core::convert::From<UnclaimsHandlerCall> for EmailWalletCoreCalls {
        fn from(value: UnclaimsHandlerCall) -> Self {
            Self::UnclaimsHandler(value)
        }
    }
    impl ::core::convert::From<ValidateEmailOpCall> for EmailWalletCoreCalls {
        fn from(value: ValidateEmailOpCall) -> Self {
            Self::ValidateEmailOp(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for EmailWalletCoreCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
        }
    }
    impl ::core::convert::From<WethContractCall> for EmailWalletCoreCalls {
        fn from(value: WethContractCall) -> Self {
            Self::WethContract(value)
        }
    }
    ///Container type for all return fields from the `accountHandler` function with signature `accountHandler()` and selector `0x33ddfb9a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccountHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `emailNullifiers` function with signature `emailNullifiers(bytes32)` and selector `0x17ace6b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailNullifiersReturn(pub bool);
    ///Container type for all return fields from the `emailValidityDuration` function with signature `emailValidityDuration()` and selector `0xb90d36f9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailValidityDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `extensionHandler` function with signature `extensionHandler()` and selector `0x9f6d1d83`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExtensionHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `handleEmailOp` function with signature `handleEmailOp((bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes))` and selector `0x1377df44`
    #[derive(Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
    pub struct HandleEmailOpReturn {
        pub success: bool,
        pub err: ::ethers::core::types::Bytes,
        pub total_fee_in_eth: ::ethers::core::types::U256,
        pub registered_unclaim_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `maxFeePerGas` function with signature `maxFeePerGas()` and selector `0x2728bf2c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxFeePerGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `priceOracle` function with signature `priceOracle()` and selector `0x2630c12f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PriceOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `relayerHandler` function with signature `relayerHandler()` and selector `0x5e5f2610`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RelayerHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenRegistry` function with signature `tokenRegistry()` and selector `0x9d23c4c7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `unclaimedFundClaimGas` function with signature `unclaimedFundClaimGas()` and selector `0x66221734`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UnclaimedFundClaimGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `unclaimedStateClaimGas` function with signature `unclaimedStateClaimGas()` and selector `0xa87fedae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UnclaimedStateClaimGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `unclaimsHandler` function with signature `unclaimsHandler()` and selector `0xd5e5c1c8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UnclaimsHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `wethContract` function with signature `wethContract()` and selector `0x4780eac1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WethContractReturn(pub ::ethers::core::types::Address);
    ///`EmailOp(bytes32,bool,bytes32,uint256,address,string,bytes32,string,bytes32,uint256,string,string,uint256,bytes,string,address,address,(string,uint256),(uint8,bytes[]),bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailOp {
        pub email_addr_pointer: [u8; 32],
        pub has_email_recipient: bool,
        pub recipient_email_addr_commit: [u8; 32],
        pub num_recipient_email_addr_bytes: ::ethers::core::types::U256,
        pub recipient_eth_addr: ::ethers::core::types::Address,
        pub command: ::std::string::String,
        pub email_nullifier: [u8; 32],
        pub email_domain: ::std::string::String,
        pub dkim_public_key_hash: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub masked_subject: ::std::string::String,
        pub fee_token_name: ::std::string::String,
        pub fee_per_gas: ::ethers::core::types::U256,
        pub execute_call_data: ::ethers::core::types::Bytes,
        pub extension_name: ::std::string::String,
        pub new_wallet_owner: ::ethers::core::types::Address,
        pub new_dkim_registry: ::ethers::core::types::Address,
        pub wallet_params: WalletParams,
        pub extension_params: ExtensionParams,
        pub email_proof: ::ethers::core::types::Bytes,
    }
    ///`ExtensionParams(uint8,bytes[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExtensionParams {
        pub subject_template_index: u8,
        pub subject_params: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///`WalletParams(string,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WalletParams {
        pub token_name: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
}
