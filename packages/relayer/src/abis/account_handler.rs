pub use account_handler::*;
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
pub mod account_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accountSaltOfPSIPoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "accountSaltOfPSIPoint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EmailProof"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultDkimRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultDkimRegistry",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDKIMRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dkimRegistryOfAccountSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dkimRegistryOfAccountSalt",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getWalletOfSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getWalletOfSalt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_relayerHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_defaultDkimRegistry",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_walletImplementation",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_emailValidityDuration",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("isAccountSaltDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isAccountSaltDeployed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountSalt"),
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
                    ::std::borrow::ToOwned::to_owned("isDKIMPublicKeyHashValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerPSIPoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerPSIPoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("psiPoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("updateDKIMRegistryOfAccountSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateDKIMRegistryOfAccountSalt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dkimRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("walletImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "walletImplementation",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("AccountCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
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
                                    name: ::std::borrow::ToOwned::to_owned("walletAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beacon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ACCOUNTHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[Pa\0\x1Da\0\"V[a\0\xE1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\0\xDFW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x80Qa+\x7Fa\x01\x18`\09`\0\x81\x81a\x04\xCD\x01R\x81\x81a\x05h\x01R\x81\x81a\t\xB7\x01R\x81\x81a\nM\x01Ra\x0B|\x01Ra+\x7F`\0\xF3\xFE`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80c\x81\x17\xAB\xC1\x11a\0\xCBW\x80c\xB9\r6\xF9\x11a\0\x7FW\x80c\xD8BHN\x11a\0YW\x80c\xD8BHN\x14a\x04VW\x80c\xF2\xFD\xE3\x8B\x14a\x04vW\x80c\xF7\x01>\xF6\x14a\x04\x96W`\0\x80\xFD[\x80c\xB9\r6\xF9\x14a\x03\xF3W\x80c\xD33\xEB\xBD\x14a\x04\tW\x80c\xD3C\xD5\xCA\x14a\x04)W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xB0W\x80c\x8D\xA5\xCB[\x14a\x03eW\x80c\xAB\x17\x95\xE2\x14a\x03\x90W\x80c\xB7\x12CT\x14a\x03\xB0W`\0\x80\xFD[\x80c\x81\x17\xAB\xC1\x14a\x03\0W\x80c\x87\xAEF\xCC\x14a\x03-W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x11a\x01\"W\x80cVd\xC7\x8E\x11a\x01\x07W\x80cVd\xC7\x8E\x14a\x02\x9EW\x80c^_&\x10\x14a\x02\xBEW\x80cqP\x18\xA6\x14a\x02\xEBW`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02hW\x80cR\xD1\x90-\x14a\x02{W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01SW\x80c6Y\xCF\xE6\x14a\x02\x06W\x80c7\x19\xD4\xDE\x14a\x02(W\x80cC\xC3\xBB!\x14a\x02HW`\0\x80\xFD[\x80c\x17\xAC\xE6\xB5\x14a\x01oW\x80c+z\xC3\xF3\x14a\x01\xB4W[`\0\x80\xFD[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01\x9Fa\x01\x8A6`\x04a\x1E8V[`l` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xC0W`\0\x80\xFD[P`hTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xABV[4\x80\x15a\x02\x12W`\0\x80\xFD[Pa\x02&a\x02!6`\x04a\x1EzV[a\x04\xB6V[\0[4\x80\x15a\x024W`\0\x80\xFD[Pa\x02&a\x02C6`\x04a\x1E\x95V[a\x06\x8CV[4\x80\x15a\x02TW`\0\x80\xFD[Pa\x02&a\x02c6`\x04a\x1F\xA7V[a\x06\xE7V[a\x02&a\x02v6`\x04a LV[a\t\xA0V[4\x80\x15a\x02\x87W`\0\x80\xFD[Pa\x02\x90a\x0BbV[`@Q\x90\x81R` \x01a\x01\xABV[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x01\xE1a\x02\xB96`\x04a\x1E8V[a\x0C4V[4\x80\x15a\x02\xCAW`\0\x80\xFD[P`gTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xF7W`\0\x80\xFD[Pa\x02&a\r\x1BV[4\x80\x15a\x03\x0CW`\0\x80\xFD[P`fTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x039W`\0\x80\xFD[Pa\x02\x90a\x03H6`\x04a \x9AV[\x80Q` \x81\x83\x01\x81\x01\x80Q`j\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[4\x80\x15a\x03qW`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xE1V[4\x80\x15a\x03\x9CW`\0\x80\xFD[Pa\x01\x9Fa\x03\xAB6`\x04a\x1E8V[a\r/V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x01\xE1a\x03\xCB6`\x04a\x1E8V[`k` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x03\xFFW`\0\x80\xFD[Pa\x02\x90`mT\x81V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x01\xE1a\x04$6`\x04a \xCFV[a\rYV[4\x80\x15a\x045W`\0\x80\xFD[P`eTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04bW`\0\x80\xFD[Pa\x01\x9Fa\x04q6`\x04a!\x0BV[a\x12@V[4\x80\x15a\x04\x82W`\0\x80\xFD[Pa\x02&a\x04\x916`\x04a\x1EzV[a\x13#V[4\x80\x15a\x04\xA2W`\0\x80\xFD[Pa\x02&a\x04\xB16`\x04a!pV[a\x13\xBDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x05fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xDB\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x06m\x81a\x15\xB8V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x06\x89\x91\x83\x91\x90a\x16\x1FV[PV[a\x06\x94a\x17\xD1V[`\0\x91\x82R`k` R`@\x90\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82a\x074W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Finvalid wallet salt\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`\0\x80\x1B`j\x85`@Qa\x07H\x91\x90a!\xF0V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x07\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPSI point exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`gT`@Q\x7FS\0\xF8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cS\0\xF8A\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08;\x91\x90\x81\x01\x90a\"QV[P\x90P\x80Q`\0\x03a\x08\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not a relayer\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`hT`@Q\x7F\x02E\xC9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x02E\xC9]\x90a\x08\xEB\x90\x87\x90\x89\x90\x88\x90\x88\x90`\x04\x01a#\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t,\x91\x90a#LV[a\txW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid PSI point proof\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[\x83`j\x86`@Qa\t\x89\x91\x90a!\xF0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 UPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\nKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\xC0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x0BR\x82a\x15\xB8V[a\x0B^\x82\x82`\x01a\x16\x1FV[PPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`\0a\r\x15\x82`@Q\x80` \x01a\x0CJ\x90a\x1E+V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`fT`\x04`\x1F\x90\x93\x01\x90\x91\x16\x91\x82R`$\x82\x01`@\x90\x81R` \x83\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x81)\xFC\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Qa\x0C\xDC\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x91\x01a#iV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\xFA\x92\x91` \x01a#\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x188V[\x92\x91PPV[a\r#a\x17\xD1V[a\r-`\0a\x18LV[V[`\0a\r\x15a\r=\x83a\x0C4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0`\xA0\x82\x015a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Finvalid wallet salt\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\r\xBCa\r=\x83`\xA0\x015a\x0C4V[\x15a\x0E\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fwallet already deployed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[``\x82\x015`\0\x90\x81R`l` R`@\x90 T`\xFF\x16\x15a\x0EmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Femail already nullified\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\x0E\xBD`\xA0\x83\x015a\x0E\x7F\x84\x80a#\xC7V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP` \x85\x015a\x12@V[a\x0F\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid DKIM public key hash\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\x0F\x19`\xE0\x83\x01`\xC0\x84\x01a$>V[\x15\x15`\x01\x14a\x0FjW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fthe invitation code must exist\0\0`D\x82\x01R`d\x01a\x05]V[`gT`@Q\x7FS\0\xF8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cS\0\xF8A\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\x01\x91\x90\x81\x01\x90a\"QV[P\x90P\x80Q`\0\x03a\x10UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not a relayer\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`@\x83\x015\x15a\x10\xBFWB`mT\x84`@\x015a\x10r\x91\x90a$[V[\x11a\x10\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Femail expired\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`hT`@Q\x7F\x10^i\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x10^i\xD5\x90a\x11\x15\x90\x86\x90`\x04\x01a$\xF9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x112W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11V\x91\x90a#LV[a\x11\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid account creation proof\0\0`D\x82\x01R`d\x01a\x05]V[``\x83\x015`\0\x90\x81R`l` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x11\xEB`\xA0\x84\x015a\x18\xC3V[\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83``\x015\x84`\xA0\x015\x7FB\x98\x9C\xCB\xF3\xAE\xEA\x87\xFE\x06\xF6+d\n\x85\x16\x8B(\xB8\x8E\xB7\t\x9B\xCDh\xE9\xFB\xD3\xA5u\x92x`@Q`@Q\x80\x91\x03\x90\xA4P\x91\x90PV[`\0\x83\x81R`k` R`@\x81 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x12\x85WP`eTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c\xE7\xA7\x97z\x90a\x12\xD9\x90\x87\x90\x87\x90`\x04\x01a%\xE0V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x1A\x91\x90a#LV[\x95\x94PPPPPV[a\x13+a\x17\xD1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x13\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x06\x89\x81a\x18LV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xDDWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xF7WP0;\x15\x80\x15a\x13\xF7WP`\0T`\xFF\x16`\x01\x14[a\x14iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x14\xC7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x14\xCFa\x1A1V[3`i\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x17\x90\x91U`m\x84\x90U`e\x80T\x82\x16\x88\x84\x16\x17\x90U`f\x80T\x82\x16\x86\x84\x16\x17\x90U`g\x80T\x82\x16\x89\x84\x16\x17\x90U`h\x80T\x90\x91\x16\x91\x86\x16\x91\x90\x91\x17\x90U\x80\x15a\x15\xB0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`iTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fcaller is not a deployer\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x16WWa\x16R\x83a\x1A\xB6V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x16\xBEWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x16\xBB\x91\x81\x01\x90a&\x02V[`\x01[a\x170W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x17\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[Pa\x16R\x83\x83\x83a\x1B\xA6V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\r-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05]V[`\0a\x18E\x83\x830a\x1B\xD1V[\x93\x92PPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`fT`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x81)\xFC\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q`\0\x92\x84\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x19E\x90a\x1E+V[a\x19P\x92\x91\x90a#iV[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x19pW=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xFD\xE3\x8Ba\x19\xAE`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x14W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A(W=`\0\x80>=`\0\xFD[PPPP\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\r-a\x1B\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1B\xAF\x83a\x1C\x81V[`\0\x82Q\x11\x80a\x1B\xBCWP\x80[\x15a\x16RWa\x1B\xCB\x83\x83a\x1C\xCEV[PPPPV[`\0`@Q\x83`@\x82\x01R\x84` \x82\x01R\x82\x81R`\x0B\x81\x01\x90P`\xFF\x81S`U\x90 \x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1CxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\r-3a\x18LV[a\x1C\x8A\x81a\x1A\xB6V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x18E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a+#`'\x919```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\x1D\x18\x91\x90a!\xF0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1DSW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1DXV[``\x91P[P\x91P\x91Pa\x1Di\x86\x83\x83\x87a\x1DsV[\x96\x95PPPPPPV[``\x83\x15a\x1D\xEFW\x82Q`\0\x03a\x1D\xE8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x1D\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05]V[P\x81a\x1D\xF9V[a\x1D\xF9\x83\x83a\x1E\x01V[\x94\x93PPPPV[\x81Q\x15a\x1E\x11W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05]\x91\x90a&\x1BV[a\x04\xF4\x80a&/\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1EJW`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EuW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1E\x8CW`\0\x80\xFD[a\x18E\x82a\x1EQV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xA8W`\0\x80\xFD[\x825\x91Pa\x1E\xB8` \x84\x01a\x1EQV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\x19Wa\x1F\x19a\x1E\xC1V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F;Wa\x1F;a\x1E\xC1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a\x1F\\a\x1FW\x84a\x1F!V[a\x1E\xF0V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x1FpW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1F\x98W`\0\x80\xFD[a\x18E\x83\x835` \x85\x01a\x1FIV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1F\xBDW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xD4W`\0\x80\xFD[a\x1F\xE0\x87\x82\x88\x01a\x1F\x87V[\x94PP` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x04W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a \x15W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a ,W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a >W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15a _W`\0\x80\xFD[a h\x83a\x1EQV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x84W`\0\x80\xFD[a \x90\x85\x82\x86\x01a\x1F\x87V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a \xACW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xC3W`\0\x80\xFD[a\x1D\xF9\x84\x82\x85\x01a\x1F\x87V[`\0` \x82\x84\x03\x12\x15a \xE1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xF8W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x18EW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a! W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!>W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a!OW`\0\x80\xFD[a!^\x86\x825` \x84\x01a\x1FIV[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a!\x88W`\0\x80\xFD[a!\x91\x86a\x1EQV[\x94Pa!\x9F` \x87\x01a\x1EQV[\x93Pa!\xAD`@\x87\x01a\x1EQV[\x92Pa!\xBB``\x87\x01a\x1EQV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0[\x83\x81\x10\x15a!\xE7W\x81\x81\x01Q\x83\x82\x01R` \x01a!\xCFV[PP`\0\x91\x01RV[`\0\x82Qa\"\x02\x81\x84` \x87\x01a!\xCCV[\x91\x90\x91\x01\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\"\x1DW`\0\x80\xFD[\x81Qa\"+a\x1FW\x82a\x1F!V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"@W`\0\x80\xFD[a\x1D\xF9\x82` \x83\x01` \x87\x01a!\xCCV[`\0\x80`@\x83\x85\x03\x12\x15a\"dW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"{W`\0\x80\xFD[a\"\x87\x85\x82\x86\x01a\"\x0CV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA4W`\0\x80\xFD[a \x90\x85\x82\x86\x01a\"\x0CV[`\0\x81Q\x80\x84Ra\"\xC8\x81` \x86\x01` \x86\x01a!\xCCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x84\x81R``` \x82\x01R`\0a# ``\x83\x01\x86a\"\xB0V[\x82\x81\x03`@\x84\x01Ra#3\x81\x85\x87a\"\xDCV[\x97\x96PPPPPPPV[\x80\x15\x15\x81\x14a\x06\x89W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#^W`\0\x80\xFD[\x81Qa\x18E\x81a#>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1D\xF9`@\x83\x01\x84a\"\xB0V[`\0\x83Qa#\xAA\x81\x84` \x88\x01a!\xCCV[\x83Q\x90\x83\x01\x90a#\xBE\x81\x83` \x88\x01a!\xCCV[\x01\x94\x93PPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#\xFCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$\x17W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$,W`\0\x80\xFD[\x92P\x92\x90PV[\x805a\x1Eu\x81a#>V[`\0` \x82\x84\x03\x12\x15a$PW`\0\x80\xFD[\x815a\x18E\x81a#>V[\x80\x82\x01\x80\x82\x11\x15a\r\x15W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a$\xCAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xEAW`\0\x80\xFD[\x806\x03\x82\x13\x15a$,W`\0\x80\xFD[` \x81R`\0a%\t\x83\x84a$\x95V[a\x01@` \x85\x01Ra% a\x01`\x85\x01\x82\x84a\"\xDCV[` \x86\x015`@\x86\x81\x01\x91\x90\x91R\x86\x015``\x80\x87\x01\x91\x90\x91R\x86\x015`\x80\x80\x87\x01\x91\x90\x91R\x90\x92Pa%V\x91P\x85\x01\x85a$\x95V[`\x1F\x19\x85\x84\x03\x01`\xA0\x86\x01Ra%m\x83\x82\x84a\"\xDCV[\x92PPP`\0`\xA0\x85\x015\x90P\x80`\xC0\x85\x01RPa%\x8D`\xC0\x85\x01a$3V[\x80\x15\x15`\xE0\x85\x01RPa%\xA2`\xE0\x85\x01a$3V[\x80\x15\x15a\x01\0\x85\x01RPa\x01\0\x84\x015a\x01 \x84\x81\x01\x91\x90\x91Ra%\xC8\x90\x85\x01\x85a$\x95V[`\x1F\x19\x85\x84\x03\x01a\x01@\x86\x01Ra\x1Di\x83\x82\x84a\"\xDCV[`@\x81R`\0a%\xF3`@\x83\x01\x85a\"\xB0V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a&\x14W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x18E` \x83\x01\x84a\"\xB0V\xFE`\x80`@R`@Qa\x04\xF48\x03\x80a\x04\xF4\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x02\xDEV[a\0.\x82\x82`\0a\x005V[PPa\x04\x01V[a\0>\x83a\0aV[`\0\x82Q\x11\x80a\0KWP\x80[\x15a\0\\Wa\0Z\x83\x83a\0\xA1V[P[PPPV[a\0j\x81a\0\xCDV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\0\xC6\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x04\xCD`'\x919a\x01\x80V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x01?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01\x9D\x91\x90a\x03\xB2V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xD8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xDDV[``\x91P[P\x90\x92P\x90Pa\x01\xEF\x86\x83\x83\x87a\x01\xF9V[\x96\x95PPPPPPV[``\x83\x15a\x02hW\x82Q`\0\x03a\x02aW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x02aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x016V[P\x81a\x02rV[a\x02r\x83\x83a\x02zV[\x94\x93PPPPV[\x81Q\x15a\x02\x8AW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x016\x91\x90a\x03\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x02\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xBDV[PP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xF1W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x08W`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03$W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x035W`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03NWa\x03Na\x02\xA4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03|Wa\x03|a\x02\xA4V[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x03\x94W`\0\x80\xFD[a\x03\xA5\x82` \x83\x01` \x86\x01a\x02\xBAV[\x80\x93PPPP\x92P\x92\x90PV[`\0\x82Qa\x03\xC4\x81\x84` \x87\x01a\x02\xBAV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x03\xED\x81`@\x85\x01` \x87\x01a\x02\xBAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\xBE\x80a\x04\x0F`\09`\0\xF3\xFE`\x80`@R6`\x10W`\x0E`\x13V[\0[`\x0E[`\x1F`\x1B`!V[`eV[V[`\0``\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15`\x83W=`\0\xF3[=`\0\xFD\xFE\xA2dipfsX\"\x12 \xC5\xDF\xB1\xAA[-\xA5\xC2\xDA\xA5\xA7WD2;\xAA\xF1[\xFF\xDE\x19\x16\x86\xD7\xC7\x88(\xAB\x8C\xCA\x1A\xD9dsolcC\0\x08\x1A\x003Address: low-level delegate call failedAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x19\x83Q-\xAC\xC8\xF5\xEB\xEF\x8D\xF2\xAFx9\xD7\x02h\x15\xEE\x9C\xBA\xD4\r\xCCY\\\xAFnI\x11'\xAEdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ACCOUNTHANDLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80c\x81\x17\xAB\xC1\x11a\0\xCBW\x80c\xB9\r6\xF9\x11a\0\x7FW\x80c\xD8BHN\x11a\0YW\x80c\xD8BHN\x14a\x04VW\x80c\xF2\xFD\xE3\x8B\x14a\x04vW\x80c\xF7\x01>\xF6\x14a\x04\x96W`\0\x80\xFD[\x80c\xB9\r6\xF9\x14a\x03\xF3W\x80c\xD33\xEB\xBD\x14a\x04\tW\x80c\xD3C\xD5\xCA\x14a\x04)W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xB0W\x80c\x8D\xA5\xCB[\x14a\x03eW\x80c\xAB\x17\x95\xE2\x14a\x03\x90W\x80c\xB7\x12CT\x14a\x03\xB0W`\0\x80\xFD[\x80c\x81\x17\xAB\xC1\x14a\x03\0W\x80c\x87\xAEF\xCC\x14a\x03-W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x11a\x01\"W\x80cVd\xC7\x8E\x11a\x01\x07W\x80cVd\xC7\x8E\x14a\x02\x9EW\x80c^_&\x10\x14a\x02\xBEW\x80cqP\x18\xA6\x14a\x02\xEBW`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02hW\x80cR\xD1\x90-\x14a\x02{W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01SW\x80c6Y\xCF\xE6\x14a\x02\x06W\x80c7\x19\xD4\xDE\x14a\x02(W\x80cC\xC3\xBB!\x14a\x02HW`\0\x80\xFD[\x80c\x17\xAC\xE6\xB5\x14a\x01oW\x80c+z\xC3\xF3\x14a\x01\xB4W[`\0\x80\xFD[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01\x9Fa\x01\x8A6`\x04a\x1E8V[`l` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xC0W`\0\x80\xFD[P`hTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xABV[4\x80\x15a\x02\x12W`\0\x80\xFD[Pa\x02&a\x02!6`\x04a\x1EzV[a\x04\xB6V[\0[4\x80\x15a\x024W`\0\x80\xFD[Pa\x02&a\x02C6`\x04a\x1E\x95V[a\x06\x8CV[4\x80\x15a\x02TW`\0\x80\xFD[Pa\x02&a\x02c6`\x04a\x1F\xA7V[a\x06\xE7V[a\x02&a\x02v6`\x04a LV[a\t\xA0V[4\x80\x15a\x02\x87W`\0\x80\xFD[Pa\x02\x90a\x0BbV[`@Q\x90\x81R` \x01a\x01\xABV[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x01\xE1a\x02\xB96`\x04a\x1E8V[a\x0C4V[4\x80\x15a\x02\xCAW`\0\x80\xFD[P`gTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xF7W`\0\x80\xFD[Pa\x02&a\r\x1BV[4\x80\x15a\x03\x0CW`\0\x80\xFD[P`fTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x039W`\0\x80\xFD[Pa\x02\x90a\x03H6`\x04a \x9AV[\x80Q` \x81\x83\x01\x81\x01\x80Q`j\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[4\x80\x15a\x03qW`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xE1V[4\x80\x15a\x03\x9CW`\0\x80\xFD[Pa\x01\x9Fa\x03\xAB6`\x04a\x1E8V[a\r/V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x01\xE1a\x03\xCB6`\x04a\x1E8V[`k` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x03\xFFW`\0\x80\xFD[Pa\x02\x90`mT\x81V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x01\xE1a\x04$6`\x04a \xCFV[a\rYV[4\x80\x15a\x045W`\0\x80\xFD[P`eTa\x01\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04bW`\0\x80\xFD[Pa\x01\x9Fa\x04q6`\x04a!\x0BV[a\x12@V[4\x80\x15a\x04\x82W`\0\x80\xFD[Pa\x02&a\x04\x916`\x04a\x1EzV[a\x13#V[4\x80\x15a\x04\xA2W`\0\x80\xFD[Pa\x02&a\x04\xB16`\x04a!pV[a\x13\xBDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x05fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xDB\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x06m\x81a\x15\xB8V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x06\x89\x91\x83\x91\x90a\x16\x1FV[PV[a\x06\x94a\x17\xD1V[`\0\x91\x82R`k` R`@\x90\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82a\x074W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Finvalid wallet salt\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`\0\x80\x1B`j\x85`@Qa\x07H\x91\x90a!\xF0V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x07\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPSI point exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`gT`@Q\x7FS\0\xF8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cS\0\xF8A\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08;\x91\x90\x81\x01\x90a\"QV[P\x90P\x80Q`\0\x03a\x08\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not a relayer\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`hT`@Q\x7F\x02E\xC9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x02E\xC9]\x90a\x08\xEB\x90\x87\x90\x89\x90\x88\x90\x88\x90`\x04\x01a#\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t,\x91\x90a#LV[a\txW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid PSI point proof\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[\x83`j\x86`@Qa\t\x89\x91\x90a!\xF0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 UPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\nKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\xC0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x0BR\x82a\x15\xB8V[a\x0B^\x82\x82`\x01a\x16\x1FV[PPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`\0a\r\x15\x82`@Q\x80` \x01a\x0CJ\x90a\x1E+V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`fT`\x04`\x1F\x90\x93\x01\x90\x91\x16\x91\x82R`$\x82\x01`@\x90\x81R` \x83\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x81)\xFC\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Qa\x0C\xDC\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x91\x01a#iV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\xFA\x92\x91` \x01a#\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x188V[\x92\x91PPV[a\r#a\x17\xD1V[a\r-`\0a\x18LV[V[`\0a\r\x15a\r=\x83a\x0C4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0`\xA0\x82\x015a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Finvalid wallet salt\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\r\xBCa\r=\x83`\xA0\x015a\x0C4V[\x15a\x0E\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fwallet already deployed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[``\x82\x015`\0\x90\x81R`l` R`@\x90 T`\xFF\x16\x15a\x0EmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Femail already nullified\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\x0E\xBD`\xA0\x83\x015a\x0E\x7F\x84\x80a#\xC7V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP` \x85\x015a\x12@V[a\x0F\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Finvalid DKIM public key hash\0\0\0\0`D\x82\x01R`d\x01a\x05]V[a\x0F\x19`\xE0\x83\x01`\xC0\x84\x01a$>V[\x15\x15`\x01\x14a\x0FjW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fthe invitation code must exist\0\0`D\x82\x01R`d\x01a\x05]V[`gT`@Q\x7FS\0\xF8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cS\0\xF8A\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\x01\x91\x90\x81\x01\x90a\"QV[P\x90P\x80Q`\0\x03a\x10UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not a relayer\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`@\x83\x015\x15a\x10\xBFWB`mT\x84`@\x015a\x10r\x91\x90a$[V[\x11a\x10\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Femail expired\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[`hT`@Q\x7F\x10^i\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x10^i\xD5\x90a\x11\x15\x90\x86\x90`\x04\x01a$\xF9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x112W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11V\x91\x90a#LV[a\x11\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid account creation proof\0\0`D\x82\x01R`d\x01a\x05]V[``\x83\x015`\0\x90\x81R`l` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x11\xEB`\xA0\x84\x015a\x18\xC3V[\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83``\x015\x84`\xA0\x015\x7FB\x98\x9C\xCB\xF3\xAE\xEA\x87\xFE\x06\xF6+d\n\x85\x16\x8B(\xB8\x8E\xB7\t\x9B\xCDh\xE9\xFB\xD3\xA5u\x92x`@Q`@Q\x80\x91\x03\x90\xA4P\x91\x90PV[`\0\x83\x81R`k` R`@\x81 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x12\x85WP`eTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c\xE7\xA7\x97z\x90a\x12\xD9\x90\x87\x90\x87\x90`\x04\x01a%\xE0V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x1A\x91\x90a#LV[\x95\x94PPPPPV[a\x13+a\x17\xD1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x13\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\x06\x89\x81a\x18LV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xDDWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xF7WP0;\x15\x80\x15a\x13\xF7WP`\0T`\xFF\x16`\x01\x14[a\x14iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x14\xC7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x14\xCFa\x1A1V[3`i\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x17\x90\x91U`m\x84\x90U`e\x80T\x82\x16\x88\x84\x16\x17\x90U`f\x80T\x82\x16\x86\x84\x16\x17\x90U`g\x80T\x82\x16\x89\x84\x16\x17\x90U`h\x80T\x90\x91\x16\x91\x86\x16\x91\x90\x91\x17\x90U\x80\x15a\x15\xB0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`iTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fcaller is not a deployer\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05]V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x16WWa\x16R\x83a\x1A\xB6V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x16\xBEWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x16\xBB\x91\x81\x01\x90a&\x02V[`\x01[a\x170W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x17\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[Pa\x16R\x83\x83\x83a\x1B\xA6V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\r-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05]V[`\0a\x18E\x83\x830a\x1B\xD1V[\x93\x92PPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`fT`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x81)\xFC\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q`\0\x92\x84\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x19E\x90a\x1E+V[a\x19P\x92\x91\x90a#iV[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x19pW=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xFD\xE3\x8Ba\x19\xAE`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x14W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A(W=`\0\x80>=`\0\xFD[PPPP\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\r-a\x1B\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1B\xAF\x83a\x1C\x81V[`\0\x82Q\x11\x80a\x1B\xBCWP\x80[\x15a\x16RWa\x1B\xCB\x83\x83a\x1C\xCEV[PPPPV[`\0`@Q\x83`@\x82\x01R\x84` \x82\x01R\x82\x81R`\x0B\x81\x01\x90P`\xFF\x81S`U\x90 \x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1CxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05]V[a\r-3a\x18LV[a\x1C\x8A\x81a\x1A\xB6V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x18E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a+#`'\x919```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\x1D\x18\x91\x90a!\xF0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1DSW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1DXV[``\x91P[P\x91P\x91Pa\x1Di\x86\x83\x83\x87a\x1DsV[\x96\x95PPPPPPV[``\x83\x15a\x1D\xEFW\x82Q`\0\x03a\x1D\xE8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x1D\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05]V[P\x81a\x1D\xF9V[a\x1D\xF9\x83\x83a\x1E\x01V[\x94\x93PPPPV[\x81Q\x15a\x1E\x11W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05]\x91\x90a&\x1BV[a\x04\xF4\x80a&/\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1EJW`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EuW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1E\x8CW`\0\x80\xFD[a\x18E\x82a\x1EQV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xA8W`\0\x80\xFD[\x825\x91Pa\x1E\xB8` \x84\x01a\x1EQV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\x19Wa\x1F\x19a\x1E\xC1V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F;Wa\x1F;a\x1E\xC1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a\x1F\\a\x1FW\x84a\x1F!V[a\x1E\xF0V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x1FpW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1F\x98W`\0\x80\xFD[a\x18E\x83\x835` \x85\x01a\x1FIV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1F\xBDW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xD4W`\0\x80\xFD[a\x1F\xE0\x87\x82\x88\x01a\x1F\x87V[\x94PP` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x04W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a \x15W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a ,W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a >W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15a _W`\0\x80\xFD[a h\x83a\x1EQV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x84W`\0\x80\xFD[a \x90\x85\x82\x86\x01a\x1F\x87V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a \xACW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xC3W`\0\x80\xFD[a\x1D\xF9\x84\x82\x85\x01a\x1F\x87V[`\0` \x82\x84\x03\x12\x15a \xE1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xF8W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x18EW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a! W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!>W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a!OW`\0\x80\xFD[a!^\x86\x825` \x84\x01a\x1FIV[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a!\x88W`\0\x80\xFD[a!\x91\x86a\x1EQV[\x94Pa!\x9F` \x87\x01a\x1EQV[\x93Pa!\xAD`@\x87\x01a\x1EQV[\x92Pa!\xBB``\x87\x01a\x1EQV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0[\x83\x81\x10\x15a!\xE7W\x81\x81\x01Q\x83\x82\x01R` \x01a!\xCFV[PP`\0\x91\x01RV[`\0\x82Qa\"\x02\x81\x84` \x87\x01a!\xCCV[\x91\x90\x91\x01\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\"\x1DW`\0\x80\xFD[\x81Qa\"+a\x1FW\x82a\x1F!V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"@W`\0\x80\xFD[a\x1D\xF9\x82` \x83\x01` \x87\x01a!\xCCV[`\0\x80`@\x83\x85\x03\x12\x15a\"dW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"{W`\0\x80\xFD[a\"\x87\x85\x82\x86\x01a\"\x0CV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA4W`\0\x80\xFD[a \x90\x85\x82\x86\x01a\"\x0CV[`\0\x81Q\x80\x84Ra\"\xC8\x81` \x86\x01` \x86\x01a!\xCCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x84\x81R``` \x82\x01R`\0a# ``\x83\x01\x86a\"\xB0V[\x82\x81\x03`@\x84\x01Ra#3\x81\x85\x87a\"\xDCV[\x97\x96PPPPPPPV[\x80\x15\x15\x81\x14a\x06\x89W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#^W`\0\x80\xFD[\x81Qa\x18E\x81a#>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1D\xF9`@\x83\x01\x84a\"\xB0V[`\0\x83Qa#\xAA\x81\x84` \x88\x01a!\xCCV[\x83Q\x90\x83\x01\x90a#\xBE\x81\x83` \x88\x01a!\xCCV[\x01\x94\x93PPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#\xFCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$\x17W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$,W`\0\x80\xFD[\x92P\x92\x90PV[\x805a\x1Eu\x81a#>V[`\0` \x82\x84\x03\x12\x15a$PW`\0\x80\xFD[\x815a\x18E\x81a#>V[\x80\x82\x01\x80\x82\x11\x15a\r\x15W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a$\xCAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xEAW`\0\x80\xFD[\x806\x03\x82\x13\x15a$,W`\0\x80\xFD[` \x81R`\0a%\t\x83\x84a$\x95V[a\x01@` \x85\x01Ra% a\x01`\x85\x01\x82\x84a\"\xDCV[` \x86\x015`@\x86\x81\x01\x91\x90\x91R\x86\x015``\x80\x87\x01\x91\x90\x91R\x86\x015`\x80\x80\x87\x01\x91\x90\x91R\x90\x92Pa%V\x91P\x85\x01\x85a$\x95V[`\x1F\x19\x85\x84\x03\x01`\xA0\x86\x01Ra%m\x83\x82\x84a\"\xDCV[\x92PPP`\0`\xA0\x85\x015\x90P\x80`\xC0\x85\x01RPa%\x8D`\xC0\x85\x01a$3V[\x80\x15\x15`\xE0\x85\x01RPa%\xA2`\xE0\x85\x01a$3V[\x80\x15\x15a\x01\0\x85\x01RPa\x01\0\x84\x015a\x01 \x84\x81\x01\x91\x90\x91Ra%\xC8\x90\x85\x01\x85a$\x95V[`\x1F\x19\x85\x84\x03\x01a\x01@\x86\x01Ra\x1Di\x83\x82\x84a\"\xDCV[`@\x81R`\0a%\xF3`@\x83\x01\x85a\"\xB0V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a&\x14W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x18E` \x83\x01\x84a\"\xB0V\xFE`\x80`@R`@Qa\x04\xF48\x03\x80a\x04\xF4\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x02\xDEV[a\0.\x82\x82`\0a\x005V[PPa\x04\x01V[a\0>\x83a\0aV[`\0\x82Q\x11\x80a\0KWP\x80[\x15a\0\\Wa\0Z\x83\x83a\0\xA1V[P[PPPV[a\0j\x81a\0\xCDV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\0\xC6\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x04\xCD`'\x919a\x01\x80V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x01?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01\x9D\x91\x90a\x03\xB2V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xD8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xDDV[``\x91P[P\x90\x92P\x90Pa\x01\xEF\x86\x83\x83\x87a\x01\xF9V[\x96\x95PPPPPPV[``\x83\x15a\x02hW\x82Q`\0\x03a\x02aW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x02aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x016V[P\x81a\x02rV[a\x02r\x83\x83a\x02zV[\x94\x93PPPPV[\x81Q\x15a\x02\x8AW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x016\x91\x90a\x03\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x02\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xBDV[PP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xF1W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x08W`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03$W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x035W`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03NWa\x03Na\x02\xA4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03|Wa\x03|a\x02\xA4V[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x03\x94W`\0\x80\xFD[a\x03\xA5\x82` \x83\x01` \x86\x01a\x02\xBAV[\x80\x93PPPP\x92P\x92\x90PV[`\0\x82Qa\x03\xC4\x81\x84` \x87\x01a\x02\xBAV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x03\xED\x81`@\x85\x01` \x87\x01a\x02\xBAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\xBE\x80a\x04\x0F`\09`\0\xF3\xFE`\x80`@R6`\x10W`\x0E`\x13V[\0[`\x0E[`\x1F`\x1B`!V[`eV[V[`\0``\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15`\x83W=`\0\xF3[=`\0\xFD\xFE\xA2dipfsX\"\x12 \xC5\xDF\xB1\xAA[-\xA5\xC2\xDA\xA5\xA7WD2;\xAA\xF1[\xFF\xDE\x19\x16\x86\xD7\xC7\x88(\xAB\x8C\xCA\x1A\xD9dsolcC\0\x08\x1A\x003Address: low-level delegate call failedAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x19\x83Q-\xAC\xC8\xF5\xEB\xEF\x8D\xF2\xAFx9\xD7\x02h\x15\xEE\x9C\xBA\xD4\r\xCCY\\\xAFnI\x11'\xAEdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ACCOUNTHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AccountHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AccountHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AccountHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AccountHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AccountHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AccountHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AccountHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACCOUNTHANDLER_ABI.clone(),
                    client,
                ),
            )
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
                ACCOUNTHANDLER_ABI.clone(),
                ACCOUNTHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `accountSaltOfPSIPoint` (0x87ae46cc) function
        pub fn account_salt_of_psi_point(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([135, 174, 70, 204], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAccount` (0xd333ebbd) function
        pub fn create_account(
            &self,
            email_proof: EmailProof,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 51, 235, 189], (email_proof,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultDkimRegistry` (0xd343d5ca) function
        pub fn default_dkim_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 67, 213, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dkimRegistryOfAccountSalt` (0xb7124354) function
        pub fn dkim_registry_of_account_salt(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([183, 18, 67, 84], p0)
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
        ///Calls the contract's `getWalletOfSalt` (0x5664c78e) function
        pub fn get_wallet_of_salt(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 100, 199, 142], salt)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf7013ef6) function
        pub fn initialize(
            &self,
            relayer_handler: ::ethers::core::types::Address,
            default_dkim_registry: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
            wallet_implementation: ::ethers::core::types::Address,
            email_validity_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [247, 1, 62, 246],
                    (
                        relayer_handler,
                        default_dkim_registry,
                        verifier,
                        wallet_implementation,
                        email_validity_duration,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAccountSaltDeployed` (0xab1795e2) function
        pub fn is_account_salt_deployed(
            &self,
            account_salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([171, 23, 149, 226], account_salt)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0xd842484e) function
        pub fn is_dkim_public_key_hash_valid(
            &self,
            account_salt: [u8; 32],
            email_domain: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [216, 66, 72, 78],
                    (account_salt, email_domain, public_key_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPSIPoint` (0x43c3bb21) function
        pub fn register_psi_point(
            &self,
            psi_point: ::ethers::core::types::Bytes,
            account_salt: [u8; 32],
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 195, 187, 33], (psi_point, account_salt, proof))
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `updateDKIMRegistryOfAccountSalt` (0x3719d4de) function
        pub fn update_dkim_registry_of_account_salt(
            &self,
            account_salt: [u8; 32],
            dkim_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 25, 212, 222], (account_salt, dkim_registry))
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
        ///Calls the contract's `walletImplementation` (0x8117abc1) function
        pub fn wallet_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([129, 23, 171, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountCreated` event
        pub fn account_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountHandlerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AccountHandler<M> {
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
    #[ethevent(name = "AccountCreated", abi = "AccountCreated(bytes32,bytes32,address)")]
    pub struct AccountCreatedFilter {
        #[ethevent(indexed)]
        pub account_salt: [u8; 32],
        #[ethevent(indexed)]
        pub email_nullifier: [u8; 32],
        #[ethevent(indexed)]
        pub wallet_addr: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccountHandlerEvents {
        AccountCreatedFilter(AccountCreatedFilter),
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AccountHandlerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccountCreatedFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::AccountCreatedFilter(decoded));
            }
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(AccountHandlerEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AccountHandlerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountCreatedFilter> for AccountHandlerEvents {
        fn from(value: AccountCreatedFilter) -> Self {
            Self::AccountCreatedFilter(value)
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for AccountHandlerEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for AccountHandlerEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for AccountHandlerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AccountHandlerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for AccountHandlerEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `accountSaltOfPSIPoint` function with signature `accountSaltOfPSIPoint(bytes)` and selector `0x87ae46cc`
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
    #[ethcall(name = "accountSaltOfPSIPoint", abi = "accountSaltOfPSIPoint(bytes)")]
    pub struct AccountSaltOfPSIPointCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `createAccount` function with signature `createAccount((string,bytes32,uint256,bytes32,string,bytes32,bool,bool,bytes32,bytes))` and selector `0xd333ebbd`
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
        name = "createAccount",
        abi = "createAccount((string,bytes32,uint256,bytes32,string,bytes32,bool,bool,bytes32,bytes))"
    )]
    pub struct CreateAccountCall {
        pub email_proof: EmailProof,
    }
    ///Container type for all input parameters for the `defaultDkimRegistry` function with signature `defaultDkimRegistry()` and selector `0xd343d5ca`
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
    #[ethcall(name = "defaultDkimRegistry", abi = "defaultDkimRegistry()")]
    pub struct DefaultDkimRegistryCall;
    ///Container type for all input parameters for the `dkimRegistryOfAccountSalt` function with signature `dkimRegistryOfAccountSalt(bytes32)` and selector `0xb7124354`
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
        name = "dkimRegistryOfAccountSalt",
        abi = "dkimRegistryOfAccountSalt(bytes32)"
    )]
    pub struct DkimRegistryOfAccountSaltCall(pub [u8; 32]);
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
    ///Container type for all input parameters for the `getWalletOfSalt` function with signature `getWalletOfSalt(bytes32)` and selector `0x5664c78e`
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
    #[ethcall(name = "getWalletOfSalt", abi = "getWalletOfSalt(bytes32)")]
    pub struct GetWalletOfSaltCall {
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,uint256)` and selector `0xf7013ef6`
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
        name = "initialize",
        abi = "initialize(address,address,address,address,uint256)"
    )]
    pub struct InitializeCall {
        pub relayer_handler: ::ethers::core::types::Address,
        pub default_dkim_registry: ::ethers::core::types::Address,
        pub verifier: ::ethers::core::types::Address,
        pub wallet_implementation: ::ethers::core::types::Address,
        pub email_validity_duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isAccountSaltDeployed` function with signature `isAccountSaltDeployed(bytes32)` and selector `0xab1795e2`
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
    #[ethcall(name = "isAccountSaltDeployed", abi = "isAccountSaltDeployed(bytes32)")]
    pub struct IsAccountSaltDeployedCall {
        pub account_salt: [u8; 32],
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(bytes32,string,bytes32)` and selector `0xd842484e`
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
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(bytes32,string,bytes32)"
    )]
    pub struct IsDKIMPublicKeyHashValidCall {
        pub account_salt: [u8; 32],
        pub email_domain: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `registerPSIPoint` function with signature `registerPSIPoint(bytes,bytes32,bytes)` and selector `0x43c3bb21`
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
    #[ethcall(name = "registerPSIPoint", abi = "registerPSIPoint(bytes,bytes32,bytes)")]
    pub struct RegisterPSIPointCall {
        pub psi_point: ::ethers::core::types::Bytes,
        pub account_salt: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateDKIMRegistryOfAccountSalt` function with signature `updateDKIMRegistryOfAccountSalt(bytes32,address)` and selector `0x3719d4de`
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
        name = "updateDKIMRegistryOfAccountSalt",
        abi = "updateDKIMRegistryOfAccountSalt(bytes32,address)"
    )]
    pub struct UpdateDKIMRegistryOfAccountSaltCall {
        pub account_salt: [u8; 32],
        pub dkim_registry: ::ethers::core::types::Address,
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `walletImplementation` function with signature `walletImplementation()` and selector `0x8117abc1`
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
    #[ethcall(name = "walletImplementation", abi = "walletImplementation()")]
    pub struct WalletImplementationCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccountHandlerCalls {
        AccountSaltOfPSIPoint(AccountSaltOfPSIPointCall),
        CreateAccount(CreateAccountCall),
        DefaultDkimRegistry(DefaultDkimRegistryCall),
        DkimRegistryOfAccountSalt(DkimRegistryOfAccountSaltCall),
        EmailNullifiers(EmailNullifiersCall),
        EmailValidityDuration(EmailValidityDurationCall),
        GetWalletOfSalt(GetWalletOfSaltCall),
        Initialize(InitializeCall),
        IsAccountSaltDeployed(IsAccountSaltDeployedCall),
        IsDKIMPublicKeyHashValid(IsDKIMPublicKeyHashValidCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RegisterPSIPoint(RegisterPSIPointCall),
        RelayerHandler(RelayerHandlerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateDKIMRegistryOfAccountSalt(UpdateDKIMRegistryOfAccountSaltCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Verifier(VerifierCall),
        WalletImplementation(WalletImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for AccountHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountSaltOfPSIPointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountSaltOfPSIPoint(decoded));
            }
            if let Ok(decoded) = <CreateAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAccount(decoded));
            }
            if let Ok(decoded) = <DefaultDkimRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultDkimRegistry(decoded));
            }
            if let Ok(decoded) = <DkimRegistryOfAccountSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DkimRegistryOfAccountSalt(decoded));
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
            if let Ok(decoded) = <GetWalletOfSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWalletOfSalt(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsAccountSaltDeployedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAccountSaltDeployed(decoded));
            }
            if let Ok(decoded) = <IsDKIMPublicKeyHashValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDKIMPublicKeyHashValid(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RegisterPSIPointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterPSIPoint(decoded));
            }
            if let Ok(decoded) = <RelayerHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayerHandler(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateDKIMRegistryOfAccountSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDKIMRegistryOfAccountSalt(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verifier(decoded));
            }
            if let Ok(decoded) = <WalletImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WalletImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccountHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountSaltOfPSIPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultDkimRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DkimRegistryOfAccountSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailNullifiers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailValidityDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletOfSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAccountSaltDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterPSIPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayerHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDKIMRegistryOfAccountSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WalletImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AccountHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountSaltOfPSIPoint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultDkimRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimRegistryOfAccountSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmailNullifiers(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmailValidityDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetWalletOfSalt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAccountSaltDeployed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterPSIPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDKIMRegistryOfAccountSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccountSaltOfPSIPointCall> for AccountHandlerCalls {
        fn from(value: AccountSaltOfPSIPointCall) -> Self {
            Self::AccountSaltOfPSIPoint(value)
        }
    }
    impl ::core::convert::From<CreateAccountCall> for AccountHandlerCalls {
        fn from(value: CreateAccountCall) -> Self {
            Self::CreateAccount(value)
        }
    }
    impl ::core::convert::From<DefaultDkimRegistryCall> for AccountHandlerCalls {
        fn from(value: DefaultDkimRegistryCall) -> Self {
            Self::DefaultDkimRegistry(value)
        }
    }
    impl ::core::convert::From<DkimRegistryOfAccountSaltCall> for AccountHandlerCalls {
        fn from(value: DkimRegistryOfAccountSaltCall) -> Self {
            Self::DkimRegistryOfAccountSalt(value)
        }
    }
    impl ::core::convert::From<EmailNullifiersCall> for AccountHandlerCalls {
        fn from(value: EmailNullifiersCall) -> Self {
            Self::EmailNullifiers(value)
        }
    }
    impl ::core::convert::From<EmailValidityDurationCall> for AccountHandlerCalls {
        fn from(value: EmailValidityDurationCall) -> Self {
            Self::EmailValidityDuration(value)
        }
    }
    impl ::core::convert::From<GetWalletOfSaltCall> for AccountHandlerCalls {
        fn from(value: GetWalletOfSaltCall) -> Self {
            Self::GetWalletOfSalt(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for AccountHandlerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsAccountSaltDeployedCall> for AccountHandlerCalls {
        fn from(value: IsAccountSaltDeployedCall) -> Self {
            Self::IsAccountSaltDeployed(value)
        }
    }
    impl ::core::convert::From<IsDKIMPublicKeyHashValidCall> for AccountHandlerCalls {
        fn from(value: IsDKIMPublicKeyHashValidCall) -> Self {
            Self::IsDKIMPublicKeyHashValid(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AccountHandlerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for AccountHandlerCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RegisterPSIPointCall> for AccountHandlerCalls {
        fn from(value: RegisterPSIPointCall) -> Self {
            Self::RegisterPSIPoint(value)
        }
    }
    impl ::core::convert::From<RelayerHandlerCall> for AccountHandlerCalls {
        fn from(value: RelayerHandlerCall) -> Self {
            Self::RelayerHandler(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AccountHandlerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AccountHandlerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateDKIMRegistryOfAccountSaltCall>
    for AccountHandlerCalls {
        fn from(value: UpdateDKIMRegistryOfAccountSaltCall) -> Self {
            Self::UpdateDKIMRegistryOfAccountSalt(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for AccountHandlerCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for AccountHandlerCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for AccountHandlerCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
        }
    }
    impl ::core::convert::From<WalletImplementationCall> for AccountHandlerCalls {
        fn from(value: WalletImplementationCall) -> Self {
            Self::WalletImplementation(value)
        }
    }
    ///Container type for all return fields from the `accountSaltOfPSIPoint` function with signature `accountSaltOfPSIPoint(bytes)` and selector `0x87ae46cc`
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
    pub struct AccountSaltOfPSIPointReturn(pub [u8; 32]);
    ///Container type for all return fields from the `createAccount` function with signature `createAccount((string,bytes32,uint256,bytes32,string,bytes32,bool,bool,bytes32,bytes))` and selector `0xd333ebbd`
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
    pub struct CreateAccountReturn {
        pub wallet: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `defaultDkimRegistry` function with signature `defaultDkimRegistry()` and selector `0xd343d5ca`
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
    pub struct DefaultDkimRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dkimRegistryOfAccountSalt` function with signature `dkimRegistryOfAccountSalt(bytes32)` and selector `0xb7124354`
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
    pub struct DkimRegistryOfAccountSaltReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getWalletOfSalt` function with signature `getWalletOfSalt(bytes32)` and selector `0x5664c78e`
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
    pub struct GetWalletOfSaltReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isAccountSaltDeployed` function with signature `isAccountSaltDeployed(bytes32)` and selector `0xab1795e2`
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
    pub struct IsAccountSaltDeployedReturn(pub bool);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(bytes32,string,bytes32)` and selector `0xd842484e`
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
    pub struct IsDKIMPublicKeyHashValidReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `walletImplementation` function with signature `walletImplementation()` and selector `0x8117abc1`
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
    pub struct WalletImplementationReturn(pub ::ethers::core::types::Address);
    ///`EmailProof(string,bytes32,uint256,bytes32,string,bytes32,bool,bool,bytes32,bytes)`
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
    pub struct EmailProof {
        pub email_domain: ::std::string::String,
        pub dkim_public_key_hash: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub email_nullifier: [u8; 32],
        pub masked_subject: ::std::string::String,
        pub account_salt: [u8; 32],
        pub is_code_exist: bool,
        pub has_email_recipient: bool,
        pub recipient_email_addr_commit: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
}
