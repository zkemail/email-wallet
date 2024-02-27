pub use token_registry::*;
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
pub mod token_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressOfTokenName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressOfTokenName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("chainIdOfName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainIdOfName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getChainIdOfName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainIdOfName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getTokenAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("getTokenNameOfAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenNameOfAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenNameOfAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("setChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setChainId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
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
                    ::std::borrow::ToOwned::to_owned("setTokenAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                    ::std::borrow::ToOwned::to_owned("setTokenAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTokenAddresses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenNames"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addrs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("tokenNameOfAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenNameOfAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("ChainRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChainRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("TokenRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
    pub static TOKENREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[Pb\0\0 b\0\0&V[b\0\0\xE7V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14b\0\0\xE5W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x80Qa)9b\0\x01\x1F`\09`\0\x81\x81a\x07y\x01R\x81\x81a\x08.\x01R\x81\x81a\t\x83\x01R\x81\x81a\n3\x01Ra\x0Bx\x01Ra)9`\0\xF3\xFE`\x80`@R`\x046\x10a\x01_W`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xC0W\x80c\xC3[uG\x11a\0tW\x80c\xE0\xAB\xD9\x1B\x11a\0YW\x80c\xE0\xAB\xD9\x1B\x14a\x03\xFFW\x80c\xE5\xBE\x9D\xEC\x14a\x04\x1FW\x80c\xF2\xFD\xE3\x8B\x14a\x04?W`\0\x80\xFD[\x80c\xC3[uG\x14a\x03\xBFW\x80c\xC4\t\x126\x14a\x03\xDFW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xA5W\x80c\x8D\xA5\xCB[\x14a\x03TW\x80c\xA2\xA7\x86.\x14a\x03\x7FW\x80c\xB3\x99\xDEN\x14a\x03\x9FW`\0\x80\xFD[\x80c\x81)\xFC\x1C\x14a\x03\x1FW\x80c\x8Cz\xF0\x80\x14a\x034W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x11a\x01\x17W\x80cg\xCB\xCB2\x11a\0\xFCW\x80cg\xCB\xCB2\x14a\x02fW\x80ciy\xE2{\x14a\x02\xEAW\x80cqP\x18\xA6\x14a\x03\nW`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02>W\x80cR\xD1\x90-\x14a\x02QW`\0\x80\xFD[\x80c%\x15\xD6`\x11a\x01HW\x80c%\x15\xD6`\x14a\x01\xD1W\x80c0L\xD0@\x14a\x01\xFEW\x80c6Y\xCF\xE6\x14a\x02\x1EW`\0\x80\xFD[\x80c\x17EnV\x14a\x01dW\x80c\x1B\xD5\x13\x0F\x14a\x01\x86W[`\0\x80\xFD[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01\x84a\x01\x7F6`\x04a\"wV[a\x04_V[\0[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\xBEa\x01\xA16`\x04a\"\xC5V[\x80Q` \x81\x83\x01\x81\x01\x80Q`g\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDDW`\0\x80\xFD[Pa\x01\xF1a\x01\xEC6`\x04a\"\xFAV[a\x04vV[`@Qa\x01\xC8\x91\x90a#AV[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x01\xBEa\x02\x196`\x04a\"\xC5V[a\x07:V[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x01\x84a\x0296`\x04a#\x92V[a\x07bV[a\x01\x84a\x02L6`\x04a#\xADV[a\tlV[4\x80\x15a\x02]W`\0\x80\xFD[Pa\x01\xBEa\x0B^V[4\x80\x15a\x02rW`\0\x80\xFD[Pa\x02\xC5a\x02\x816`\x04a$\x0FV[`e` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC8V[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x01\x84a\x03\x056`\x04a$LV[a\x0CJV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x01\x84a\x0EmV[4\x80\x15a\x03+W`\0\x80\xFD[Pa\x01\x84a\x0E\x81V[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x02\xC5a\x03O6`\x04a$\x0FV[a\x10\xD5V[4\x80\x15a\x03`W`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xC5V[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x01\xF1a\x03\x9A6`\x04a\"\xFAV[a\x14\x8DV[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x01\xF1a\x03\xBA6`\x04a#\x92V[a\x152V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x02\xC5a\x03\xDA6`\x04a$\x91V[a\x15>V[4\x80\x15a\x03\xEBW`\0\x80\xFD[Pa\x02\xC5a\x03\xFA6`\x04a\"\xC5V[a\x15\xF7V[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x01\x84a\x04\x1A6`\x04a%\x85V[a\x16\x03V[4\x80\x15a\x04+W`\0\x80\xFD[Pa\x01\x84a\x04:6`\x04a&dV[a\x16\xF2V[4\x80\x15a\x04KW`\0\x80\xFD[Pa\x01\x84a\x04Z6`\x04a#\x92V[a\x196V[a\x04ga\x19\xEAV[a\x04rF\x83\x83a\x16\xF2V[PPV[``a\x04\xB7\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05#WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[a\x05b\x83`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xCEWP`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[a\x06\r\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x06yWP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[`\0\x83\x81R`f` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80Ta\x06\xB3\x90a&\xBBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xDF\x90a&\xBBV[\x80\x15a\x07,W\x80`\x1F\x10a\x07\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0`g\x82`@Qa\x07L\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x08,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xA1\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\tM\x81a\x1AkV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\ti\x91\x83\x91\x90a\x1AsV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\xA6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0BR\x82a\x1AkV[a\x04r\x82\x82`\x01a\x1AsV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[a\x0CRa\x19\xEAV[\x80`\0\x03a\x0C\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fchain id cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7Fmainnet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F{\xEA\xFA\x94\xC8\xBF\xB8\xF1\xC1\xA41\x04\xA3Or\xC5$&\x8A\xAF\xBF\xE8;\xFF\x17HU94\\f\xFF\x03a\r\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fcannot set mainnet chain id\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`g\x82`@Qa\r\x8F\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x14a\x0E\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fchain id already set\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[\x80`g\x83`@Qa\x0E\x18\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x80\x82`@Qa\x0E9\x91\x90a'\x0EV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x86\xA4(l\xD1~\x83w\x05\xC9\x98\x8BbB\xB5\xCD\xD7 \xD82 \x8D\xD8\x85f\x8F\xF0\x0EQM(\xDF\x90`\0\x90\xA3PPV[a\x0Eua\x19\xEAV[a\x0E\x7F`\0a\x1CwV[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\xA1WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E\xBBWP0;\x15\x80\x15a\x0E\xBBWP`\0T`\xFF\x16`\x01\x14[a\x0FGW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0F\xA5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x0F\xADa\x1C\xEEV[`\0`g`@Qa\x0F\xE1\x90\x7Fmainnet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x07\x01\x90V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91U\x7Foptimism\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\n\x90`g\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91U\x7Farbitrum\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xA4\xB1\x90`g\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 U\x80\x15a\tiW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 `\0\x90\x7F\xAA\xAE\xBE\xBA8\x10\xB1\xE6\xB7\x07\x81\xF1K-r\xC1\xCB\x89\xC0\xB2\xB3 \xC4;\xB6\x7F\xF7\x9FV/_\xF4\x03a\x11rW`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x91P[`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\x0F\x8A\x19?\xF4dCD\x86\xC0\xDA\xF7\xDB*\x89X\x846]+\xC8K\xA4zh\xFC\xF8\x9C\x1B\x14\xB5\xB8\x03a\x12bW\x82`\0\x03a\x11\xF6WPs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2a\x074V[\x82`\n\x03a\x12\x19WPsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06a\x074V[\x82a\xA4\xB1\x03a\x12=WPs\x82\xAFID}\x8A\x07\xE3\xBD\x95\xBD\rV\xF3RAR?\xBA\xB1a\x074V[\x82b\xAA6\xA7\x03a\x12bWPs\xFF\xF9\x97g\x82\xD4l\xC0V0\xD1\xF6\xEB\xAB\x18\xB22Mk\x14a\x074V[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\xA5\xE9/>\xFBh&\x15_\x1Fr\x8E\x16*\xF9\xD7\xCD\xA3:WJ\x11S\xB5\x8F\x03\xEA\x01\xCC7\xE5h\x03a\x13RW\x82`\0\x03a\x12\xE6WPsk\x17Tt\xE8\x90\x94\xC4M\xA9\x8B\x95N\xED\xEA\xC4\x95'\x1D\x0Fa\x074V[\x82`\n\x03a\x13\tWPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x074V[\x82a\xA4\xB1\x03a\x13-WPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x074V[\x82b\xAA6\xA7\x03a\x13RWPs\xFF4\xB3\xD4\xAE\xE8\xDD\xCDo\x9A\xFF\xFBo\xE4\x9B\xD3q\xB8\xA3Wa\x074V[`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\xD6\xAC\xA1\xBE\x97)\xC1=gs5\x16\x13!d\x9C\xCC\xAEjY\x15Tw%\x16p\x0F\x98o\x94.\xAA\x03a\x14BW\x82`\0\x03a\x13\xD6WPs\xA0\xB8i\x91\xC6!\x8B6\xC1\xD1\x9DJ.\x9E\xB0\xCE6\x06\xEBHa\x074V[\x82`\n\x03a\x13\xF9WPs\x7F\\vL\xBC\x14\xF9f\x9B\x88\x83|\xA1I\x0C\xCA\x17\xC3\x16\x07a\x074V[\x82a\xA4\xB1\x03a\x14\x1DWPs\xAF\x88\xD0e\xE7|\x8C\xC2#\x93'\xC5\xED\xB3\xA42&\x8EX1a\x074V[\x82b\xAA6\xA7\x03a\x14BWPs\x1C}K\x19l\xB0\xC7\xB0\x1Dt?\xBCa\x16\xA9\x027\x9Cr8a\x074V[`\0\x83\x81R`e` R`@\x90\x81\x90 \x90Qa\x14_\x90\x84\x90a'\x0EV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[`f` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x14\xB1\x90a&\xBBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xDD\x90a&\xBBV[\x80\x15a\x15*W\x80`\x1F\x10a\x14\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x074F\x83a\x04vV[`\0`g\x83`@Qa\x15P\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x03a\x15\xC8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Funknown chain name\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[a\x15\xF0`g\x84`@Qa\x15\xDB\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x83a\x10\xD5V[\x93\x92PPPV[`\0a\x074F\x83a\x10\xD5V[a\x16\x0Ba\x19\xEAV[\x80Q\x82Q\x14a\x16\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FtokenNames and addrs length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[`\0[\x82Q\x81\x10\x15a\x16\xECWa\x16\xE4\x84\x84\x83\x81Q\x81\x10a\x16\xBDWa\x16\xBDa'*V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\x16\xD7Wa\x16\xD7a'*V[` \x02` \x01\x01Qa\x16\xF2V[`\x01\x01a\x16\x9EV[PPPPV[a\x16\xFAa\x19\xEAV[`\0\x83\x81R`e` R`@\x80\x82 \x90Qa\x17\x16\x90\x85\x90a'\x0EV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FToken already registered\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`\0\x83\x81R`f` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 \x80Ta\x17\xDD\x90a&\xBBV[\x15\x90Pa\x18FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FAddress already registered\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[\x80`e`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x83`@Qa\x18h\x91\x90a'\x0EV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U`\0\x86\x81R`f\x83R\x81\x81 \x93\x85\x16\x81R\x92\x90\x91R\x90 a\x18\xD9\x83\x82a'\xA9V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Qa\x18\xFF\x91\x90a'\x0EV[`@Q\x90\x81\x90\x03\x81 \x90\x85\x90\x7Fi$\xCDB\xAC\x86\x94\xED_A\xAA29\xEE\x8E\xD5\x9C\xDB90\x13\xC2\x97u\xF1\xEF\xF5\xF4\xE0\xA6\xBC\xF9\x90`\0\x90\xA4PPPV[a\x19>a\x19\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x19\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\ti\x81a\x1CwV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08#V[a\tia\x19\xEAV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1A\xABWa\x1A\xA6\x83a\x1D\x8DV[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B0WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1B-\x91\x81\x01\x90a(\xC3V[`\x01[a\x1B\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1CkW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[Pa\x1A\xA6\x83\x83\x83a\x1E\x97V[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1D\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0E\x7Fa\x1E\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1E1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1E\xA0\x83a\x1F\\V[`\0\x82Q\x11\x80a\x1E\xADWP\x80[\x15a\x1A\xA6Wa\x16\xEC\x83\x83a\x1F\xA9V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1FSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0E\x7F3a\x1CwV[a\x1Fe\x81a\x1D\x8DV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x15\xF0\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a(\xDD`'\x919```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\x1F\xF3\x91\x90a'\x0EV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a .W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a 3V[``\x91P[P\x91P\x91Pa D\x86\x83\x83\x87a NV[\x96\x95PPPPPPV[``\x83\x15a \xE4W\x82Q`\0\x03a \xDDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a \xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08#V[P\x81a \xEEV[a \xEE\x83\x83a \xF6V[\x94\x93PPPPV[\x81Q\x15a!\x06W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08#\x91\x90a#AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!\xB0Wa!\xB0a!:V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a!\xD2Wa!\xD2a!:V[a\"\x03` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01a!iV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\"\x17W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\"?W`\0\x80\xFD[a\x15\xF0\x83\x835` \x85\x01a!\xB8V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"rW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x8AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA1W`\0\x80\xFD[a\"\xAD\x85\x82\x86\x01a\".V[\x92PPa\"\xBC` \x84\x01a\"NV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\"\xD7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xEEW`\0\x80\xFD[a \xEE\x84\x82\x85\x01a\".V[`\0\x80`@\x83\x85\x03\x12\x15a#\rW`\0\x80\xFD[\x825\x91Pa\"\xBC` \x84\x01a\"NV[`\0[\x83\x81\x10\x15a#8W\x81\x81\x01Q\x83\x82\x01R` \x01a# V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra#`\x81`@\x85\x01` \x87\x01a#\x1DV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xA4W`\0\x80\xFD[a\x15\xF0\x82a\"NV[`\0\x80`@\x83\x85\x03\x12\x15a#\xC0W`\0\x80\xFD[a#\xC9\x83a\"NV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\xF6W`\0\x80\xFD[a$\x05\x85\x825` \x84\x01a!\xB8V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\"W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$@W`\0\x80\xFD[a$\x05\x85\x82\x86\x01a\".V[`\0\x80`@\x83\x85\x03\x12\x15a$_W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$vW`\0\x80\xFD[a$\x82\x85\x82\x86\x01a\".V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a$\xA4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xBCW`\0\x80\xFD[a$\xC8\x86\x83\x87\x01a\".V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a$\xDEW`\0\x80\xFD[Pa$\x05\x85\x82\x86\x01a\".V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x05Wa%\x05a!:V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a% W`\0\x80\xFD[\x815` a%5a%0\x83a$\xEBV[a!iV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a%WW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a%zWa%m\x81a\"NV[\x83R\x91\x83\x01\x91\x83\x01a%\\V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a%\x9AW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xBAW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a%\xCEW`\0\x80\xFD[\x815a%\xDCa%0\x82a$\xEBV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a%\xFBW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a&3W\x805\x85\x81\x11\x15a&\x17W`\0\x80\x81\xFD[a&%\x8D\x89\x83\x8A\x01\x01a\".V[\x84RP\x91\x86\x01\x91\x86\x01a%\xFFV[P\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a&LW`\0\x80\xFD[PPa&Z\x86\x82\x87\x01a%\x0FV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a&yW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x97W`\0\x80\xFD[a&\xA3\x86\x82\x87\x01a\".V[\x92PPa&\xB2`@\x85\x01a\"NV[\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xCFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\x08W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa' \x81\x84` \x87\x01a#\x1DV[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x1A\xA6W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\x82WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\xA1W\x82\x81U`\x01\x01a'\x8EV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xC3Wa'\xC3a!:V[a'\xD7\x81a'\xD1\x84Ta&\xBBV[\x84a'YV[` \x80`\x1F\x83\x11`\x01\x81\x14a(*W`\0\x84\x15a'\xF4WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\xA1V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a(wW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(XV[P\x85\x82\x10\x15a(\xB3W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a(\xD5W`\0\x80\xFD[PQ\x91\x90PV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 &]\x9F\xA6\x16\xFC\xF6\xE9\xED\xA1\xC4\x85\xE4O\xE0\xAFZ`\xE1\xCC\xAE\x0B\x1C*V\xE0\xF5Q\xBA|\xB6xdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static TOKENREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01_W`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xC0W\x80c\xC3[uG\x11a\0tW\x80c\xE0\xAB\xD9\x1B\x11a\0YW\x80c\xE0\xAB\xD9\x1B\x14a\x03\xFFW\x80c\xE5\xBE\x9D\xEC\x14a\x04\x1FW\x80c\xF2\xFD\xE3\x8B\x14a\x04?W`\0\x80\xFD[\x80c\xC3[uG\x14a\x03\xBFW\x80c\xC4\t\x126\x14a\x03\xDFW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xA5W\x80c\x8D\xA5\xCB[\x14a\x03TW\x80c\xA2\xA7\x86.\x14a\x03\x7FW\x80c\xB3\x99\xDEN\x14a\x03\x9FW`\0\x80\xFD[\x80c\x81)\xFC\x1C\x14a\x03\x1FW\x80c\x8Cz\xF0\x80\x14a\x034W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x11a\x01\x17W\x80cg\xCB\xCB2\x11a\0\xFCW\x80cg\xCB\xCB2\x14a\x02fW\x80ciy\xE2{\x14a\x02\xEAW\x80cqP\x18\xA6\x14a\x03\nW`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02>W\x80cR\xD1\x90-\x14a\x02QW`\0\x80\xFD[\x80c%\x15\xD6`\x11a\x01HW\x80c%\x15\xD6`\x14a\x01\xD1W\x80c0L\xD0@\x14a\x01\xFEW\x80c6Y\xCF\xE6\x14a\x02\x1EW`\0\x80\xFD[\x80c\x17EnV\x14a\x01dW\x80c\x1B\xD5\x13\x0F\x14a\x01\x86W[`\0\x80\xFD[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01\x84a\x01\x7F6`\x04a\"wV[a\x04_V[\0[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\xBEa\x01\xA16`\x04a\"\xC5V[\x80Q` \x81\x83\x01\x81\x01\x80Q`g\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDDW`\0\x80\xFD[Pa\x01\xF1a\x01\xEC6`\x04a\"\xFAV[a\x04vV[`@Qa\x01\xC8\x91\x90a#AV[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x01\xBEa\x02\x196`\x04a\"\xC5V[a\x07:V[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x01\x84a\x0296`\x04a#\x92V[a\x07bV[a\x01\x84a\x02L6`\x04a#\xADV[a\tlV[4\x80\x15a\x02]W`\0\x80\xFD[Pa\x01\xBEa\x0B^V[4\x80\x15a\x02rW`\0\x80\xFD[Pa\x02\xC5a\x02\x816`\x04a$\x0FV[`e` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC8V[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x01\x84a\x03\x056`\x04a$LV[a\x0CJV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x01\x84a\x0EmV[4\x80\x15a\x03+W`\0\x80\xFD[Pa\x01\x84a\x0E\x81V[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x02\xC5a\x03O6`\x04a$\x0FV[a\x10\xD5V[4\x80\x15a\x03`W`\0\x80\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xC5V[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x01\xF1a\x03\x9A6`\x04a\"\xFAV[a\x14\x8DV[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x01\xF1a\x03\xBA6`\x04a#\x92V[a\x152V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x02\xC5a\x03\xDA6`\x04a$\x91V[a\x15>V[4\x80\x15a\x03\xEBW`\0\x80\xFD[Pa\x02\xC5a\x03\xFA6`\x04a\"\xC5V[a\x15\xF7V[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x01\x84a\x04\x1A6`\x04a%\x85V[a\x16\x03V[4\x80\x15a\x04+W`\0\x80\xFD[Pa\x01\x84a\x04:6`\x04a&dV[a\x16\xF2V[4\x80\x15a\x04KW`\0\x80\xFD[Pa\x01\x84a\x04Z6`\x04a#\x92V[a\x196V[a\x04ga\x19\xEAV[a\x04rF\x83\x83a\x16\xF2V[PPV[``a\x04\xB7\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05#WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[a\x05b\x83`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xCEWP`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[a\x06\r\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x06yWP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x074V[`\0\x83\x81R`f` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80Ta\x06\xB3\x90a&\xBBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xDF\x90a&\xBBV[\x80\x15a\x07,W\x80`\x1F\x10a\x07\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0`g\x82`@Qa\x07L\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x08,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xA1\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\tM\x81a\x1AkV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\ti\x91\x83\x91\x90a\x1AsV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\xA6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0BR\x82a\x1AkV[a\x04r\x82\x82`\x01a\x1AsV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[a\x0CRa\x19\xEAV[\x80`\0\x03a\x0C\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fchain id cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7Fmainnet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F{\xEA\xFA\x94\xC8\xBF\xB8\xF1\xC1\xA41\x04\xA3Or\xC5$&\x8A\xAF\xBF\xE8;\xFF\x17HU94\\f\xFF\x03a\r\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fcannot set mainnet chain id\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`g\x82`@Qa\r\x8F\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x14a\x0E\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fchain id already set\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[\x80`g\x83`@Qa\x0E\x18\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x80\x82`@Qa\x0E9\x91\x90a'\x0EV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x86\xA4(l\xD1~\x83w\x05\xC9\x98\x8BbB\xB5\xCD\xD7 \xD82 \x8D\xD8\x85f\x8F\xF0\x0EQM(\xDF\x90`\0\x90\xA3PPV[a\x0Eua\x19\xEAV[a\x0E\x7F`\0a\x1CwV[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\xA1WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E\xBBWP0;\x15\x80\x15a\x0E\xBBWP`\0T`\xFF\x16`\x01\x14[a\x0FGW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0F\xA5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x0F\xADa\x1C\xEEV[`\0`g`@Qa\x0F\xE1\x90\x7Fmainnet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x07\x01\x90V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91U\x7Foptimism\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\n\x90`g\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91U\x7Farbitrum\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\xA4\xB1\x90`g\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 U\x80\x15a\tiW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 `\0\x90\x7F\xAA\xAE\xBE\xBA8\x10\xB1\xE6\xB7\x07\x81\xF1K-r\xC1\xCB\x89\xC0\xB2\xB3 \xC4;\xB6\x7F\xF7\x9FV/_\xF4\x03a\x11rW`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x91P[`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FWETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\x0F\x8A\x19?\xF4dCD\x86\xC0\xDA\xF7\xDB*\x89X\x846]+\xC8K\xA4zh\xFC\xF8\x9C\x1B\x14\xB5\xB8\x03a\x12bW\x82`\0\x03a\x11\xF6WPs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2a\x074V[\x82`\n\x03a\x12\x19WPsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06a\x074V[\x82a\xA4\xB1\x03a\x12=WPs\x82\xAFID}\x8A\x07\xE3\xBD\x95\xBD\rV\xF3RAR?\xBA\xB1a\x074V[\x82b\xAA6\xA7\x03a\x12bWPs\xFF\xF9\x97g\x82\xD4l\xC0V0\xD1\xF6\xEB\xAB\x18\xB22Mk\x14a\x074V[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81R\x7FDAI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\xA5\xE9/>\xFBh&\x15_\x1Fr\x8E\x16*\xF9\xD7\xCD\xA3:WJ\x11S\xB5\x8F\x03\xEA\x01\xCC7\xE5h\x03a\x13RW\x82`\0\x03a\x12\xE6WPsk\x17Tt\xE8\x90\x94\xC4M\xA9\x8B\x95N\xED\xEA\xC4\x95'\x1D\x0Fa\x074V[\x82`\n\x03a\x13\tWPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x074V[\x82a\xA4\xB1\x03a\x13-WPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x074V[\x82b\xAA6\xA7\x03a\x13RWPs\xFF4\xB3\xD4\xAE\xE8\xDD\xCDo\x9A\xFF\xFBo\xE4\x9B\xD3q\xB8\xA3Wa\x074V[`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FUSDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x82Q\x90\x83\x01 \x7F\xD6\xAC\xA1\xBE\x97)\xC1=gs5\x16\x13!d\x9C\xCC\xAEjY\x15Tw%\x16p\x0F\x98o\x94.\xAA\x03a\x14BW\x82`\0\x03a\x13\xD6WPs\xA0\xB8i\x91\xC6!\x8B6\xC1\xD1\x9DJ.\x9E\xB0\xCE6\x06\xEBHa\x074V[\x82`\n\x03a\x13\xF9WPs\x7F\\vL\xBC\x14\xF9f\x9B\x88\x83|\xA1I\x0C\xCA\x17\xC3\x16\x07a\x074V[\x82a\xA4\xB1\x03a\x14\x1DWPs\xAF\x88\xD0e\xE7|\x8C\xC2#\x93'\xC5\xED\xB3\xA42&\x8EX1a\x074V[\x82b\xAA6\xA7\x03a\x14BWPs\x1C}K\x19l\xB0\xC7\xB0\x1Dt?\xBCa\x16\xA9\x027\x9Cr8a\x074V[`\0\x83\x81R`e` R`@\x90\x81\x90 \x90Qa\x14_\x90\x84\x90a'\x0EV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[`f` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x14\xB1\x90a&\xBBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xDD\x90a&\xBBV[\x80\x15a\x15*W\x80`\x1F\x10a\x14\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x074F\x83a\x04vV[`\0`g\x83`@Qa\x15P\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x03a\x15\xC8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Funknown chain name\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[a\x15\xF0`g\x84`@Qa\x15\xDB\x91\x90a'\x0EV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x83a\x10\xD5V[\x93\x92PPPV[`\0a\x074F\x83a\x10\xD5V[a\x16\x0Ba\x19\xEAV[\x80Q\x82Q\x14a\x16\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FtokenNames and addrs length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[`\0[\x82Q\x81\x10\x15a\x16\xECWa\x16\xE4\x84\x84\x83\x81Q\x81\x10a\x16\xBDWa\x16\xBDa'*V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\x16\xD7Wa\x16\xD7a'*V[` \x02` \x01\x01Qa\x16\xF2V[`\x01\x01a\x16\x9EV[PPPPV[a\x16\xFAa\x19\xEAV[`\0\x83\x81R`e` R`@\x80\x82 \x90Qa\x17\x16\x90\x85\x90a'\x0EV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FToken already registered\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[`\0\x83\x81R`f` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 \x80Ta\x17\xDD\x90a&\xBBV[\x15\x90Pa\x18FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FAddress already registered\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08#V[\x80`e`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x83`@Qa\x18h\x91\x90a'\x0EV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U`\0\x86\x81R`f\x83R\x81\x81 \x93\x85\x16\x81R\x92\x90\x91R\x90 a\x18\xD9\x83\x82a'\xA9V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Qa\x18\xFF\x91\x90a'\x0EV[`@Q\x90\x81\x90\x03\x81 \x90\x85\x90\x7Fi$\xCDB\xAC\x86\x94\xED_A\xAA29\xEE\x8E\xD5\x9C\xDB90\x13\xC2\x97u\xF1\xEF\xF5\xF4\xE0\xA6\xBC\xF9\x90`\0\x90\xA4PPPV[a\x19>a\x19\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x19\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\ti\x81a\x1CwV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08#V[a\tia\x19\xEAV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1A\xABWa\x1A\xA6\x83a\x1D\x8DV[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B0WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1B-\x91\x81\x01\x90a(\xC3V[`\x01[a\x1B\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1CkW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[Pa\x1A\xA6\x83\x83\x83a\x1E\x97V[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1D\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0E\x7Fa\x1E\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x1E1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1E\xA0\x83a\x1F\\V[`\0\x82Q\x11\x80a\x1E\xADWP\x80[\x15a\x1A\xA6Wa\x16\xEC\x83\x83a\x1F\xA9V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1FSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08#V[a\x0E\x7F3a\x1CwV[a\x1Fe\x81a\x1D\x8DV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x15\xF0\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a(\xDD`'\x919```\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa\x1F\xF3\x91\x90a'\x0EV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a .W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a 3V[``\x91P[P\x91P\x91Pa D\x86\x83\x83\x87a NV[\x96\x95PPPPPPV[``\x83\x15a \xE4W\x82Q`\0\x03a \xDDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a \xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08#V[P\x81a \xEEV[a \xEE\x83\x83a \xF6V[\x94\x93PPPPV[\x81Q\x15a!\x06W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08#\x91\x90a#AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!\xB0Wa!\xB0a!:V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a!\xD2Wa!\xD2a!:V[a\"\x03` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01a!iV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\"\x17W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\"?W`\0\x80\xFD[a\x15\xF0\x83\x835` \x85\x01a!\xB8V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"rW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x8AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA1W`\0\x80\xFD[a\"\xAD\x85\x82\x86\x01a\".V[\x92PPa\"\xBC` \x84\x01a\"NV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\"\xD7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xEEW`\0\x80\xFD[a \xEE\x84\x82\x85\x01a\".V[`\0\x80`@\x83\x85\x03\x12\x15a#\rW`\0\x80\xFD[\x825\x91Pa\"\xBC` \x84\x01a\"NV[`\0[\x83\x81\x10\x15a#8W\x81\x81\x01Q\x83\x82\x01R` \x01a# V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra#`\x81`@\x85\x01` \x87\x01a#\x1DV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xA4W`\0\x80\xFD[a\x15\xF0\x82a\"NV[`\0\x80`@\x83\x85\x03\x12\x15a#\xC0W`\0\x80\xFD[a#\xC9\x83a\"NV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\xF6W`\0\x80\xFD[a$\x05\x85\x825` \x84\x01a!\xB8V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\"W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$@W`\0\x80\xFD[a$\x05\x85\x82\x86\x01a\".V[`\0\x80`@\x83\x85\x03\x12\x15a$_W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$vW`\0\x80\xFD[a$\x82\x85\x82\x86\x01a\".V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a$\xA4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xBCW`\0\x80\xFD[a$\xC8\x86\x83\x87\x01a\".V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a$\xDEW`\0\x80\xFD[Pa$\x05\x85\x82\x86\x01a\".V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x05Wa%\x05a!:V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a% W`\0\x80\xFD[\x815` a%5a%0\x83a$\xEBV[a!iV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a%WW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a%zWa%m\x81a\"NV[\x83R\x91\x83\x01\x91\x83\x01a%\\V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a%\x9AW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xBAW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a%\xCEW`\0\x80\xFD[\x815a%\xDCa%0\x82a$\xEBV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a%\xFBW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a&3W\x805\x85\x81\x11\x15a&\x17W`\0\x80\x81\xFD[a&%\x8D\x89\x83\x8A\x01\x01a\".V[\x84RP\x91\x86\x01\x91\x86\x01a%\xFFV[P\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a&LW`\0\x80\xFD[PPa&Z\x86\x82\x87\x01a%\x0FV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a&yW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x97W`\0\x80\xFD[a&\xA3\x86\x82\x87\x01a\".V[\x92PPa&\xB2`@\x85\x01a\"NV[\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xCFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\x08W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa' \x81\x84` \x87\x01a#\x1DV[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x1A\xA6W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\x82WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\xA1W\x82\x81U`\x01\x01a'\x8EV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xC3Wa'\xC3a!:V[a'\xD7\x81a'\xD1\x84Ta&\xBBV[\x84a'YV[` \x80`\x1F\x83\x11`\x01\x81\x14a(*W`\0\x84\x15a'\xF4WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\xA1V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a(wW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(XV[P\x85\x82\x10\x15a(\xB3W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a(\xD5W`\0\x80\xFD[PQ\x91\x90PV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 &]\x9F\xA6\x16\xFC\xF6\xE9\xED\xA1\xC4\x85\xE4O\xE0\xAFZ`\xE1\xCC\xAE\x0B\x1C*V\xE0\xF5Q\xBA|\xB6xdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static TOKENREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TokenRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TokenRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TokenRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TokenRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TokenRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TokenRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TokenRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TOKENREGISTRY_ABI.clone(),
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
                TOKENREGISTRY_ABI.clone(),
                TOKENREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addressOfTokenName` (0x67cbcb32) function
        pub fn address_of_token_name(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([103, 203, 203, 50], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainIdOfName` (0x1bd5130f) function
        pub fn chain_id_of_name(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 213, 19, 15], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainIdOfName` (0x304cd040) function
        pub fn get_chain_id_of_name(
            &self,
            chain_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 76, 208, 64], chain_name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAddress` (0x8c7af080) function
        pub fn get_token_address_with_chain_id(
            &self,
            chain_id: ::ethers::core::types::U256,
            token_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([140, 122, 240, 128], (chain_id, token_name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAddress` (0xc35b7547) function
        pub fn get_token_address_with_chain_name(
            &self,
            chain_name: ::std::string::String,
            token_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([195, 91, 117, 71], (chain_name, token_name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAddress` (0xc4091236) function
        pub fn get_token_address(
            &self,
            token_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 9, 18, 54], token_name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenNameOfAddress` (0x2515d660) function
        pub fn get_token_name_of_address_with_chain_id(
            &self,
            chain_id: ::ethers::core::types::U256,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([37, 21, 214, 96], (chain_id, addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenNameOfAddress` (0xb399de4e) function
        pub fn get_token_name_of_address(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([179, 153, 222, 78], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChainId` (0x6979e27b) function
        pub fn set_chain_id(
            &self,
            chain_name: ::std::string::String,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 121, 226, 123], (chain_name, chain_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTokenAddress` (0x17456e56) function
        pub fn set_token_address(
            &self,
            token_name: ::std::string::String,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 69, 110, 86], (token_name, addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTokenAddress` (0xe5be9dec) function
        pub fn set_token_address_with_chain_id_and_token_name(
            &self,
            chain_id: ::ethers::core::types::U256,
            token_name: ::std::string::String,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 190, 157, 236], (chain_id, token_name, addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTokenAddresses` (0xe0abd91b) function
        pub fn set_token_addresses(
            &self,
            chain_id: ::ethers::core::types::U256,
            token_names: ::std::vec::Vec<::std::string::String>,
            addrs: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 171, 217, 27], (chain_id, token_names, addrs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenNameOfAddress` (0xa2a7862e) function
        pub fn token_name_of_address(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([162, 167, 134, 46], (p0, p1))
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
        ///Gets the contract's `ChainRegistered` event
        pub fn chain_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChainRegisteredFilter,
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
        ///Gets the contract's `TokenRegistered` event
        pub fn token_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRegisteredFilter,
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
            TokenRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TokenRegistry<M> {
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
    #[ethevent(name = "ChainRegistered", abi = "ChainRegistered(string,uint256)")]
    pub struct ChainRegisteredFilter {
        #[ethevent(indexed)]
        pub chain_name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "TokenRegistered",
        abi = "TokenRegistered(uint256,string,address)"
    )]
    pub struct TokenRegisteredFilter {
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub addr: ::ethers::core::types::Address,
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
    pub enum TokenRegistryEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        ChainRegisteredFilter(ChainRegisteredFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TokenRegisteredFilter(TokenRegisteredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TokenRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = ChainRegisteredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::ChainRegisteredFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TokenRegisteredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::TokenRegisteredFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TokenRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChainRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for TokenRegistryEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for TokenRegistryEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<ChainRegisteredFilter> for TokenRegistryEvents {
        fn from(value: ChainRegisteredFilter) -> Self {
            Self::ChainRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for TokenRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for TokenRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TokenRegisteredFilter> for TokenRegistryEvents {
        fn from(value: TokenRegisteredFilter) -> Self {
            Self::TokenRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for TokenRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addressOfTokenName` function with signature `addressOfTokenName(uint256,string)` and selector `0x67cbcb32`
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
    #[ethcall(name = "addressOfTokenName", abi = "addressOfTokenName(uint256,string)")]
    pub struct AddressOfTokenNameCall(
        pub ::ethers::core::types::U256,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `chainIdOfName` function with signature `chainIdOfName(string)` and selector `0x1bd5130f`
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
    #[ethcall(name = "chainIdOfName", abi = "chainIdOfName(string)")]
    pub struct ChainIdOfNameCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getChainIdOfName` function with signature `getChainIdOfName(string)` and selector `0x304cd040`
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
    #[ethcall(name = "getChainIdOfName", abi = "getChainIdOfName(string)")]
    pub struct GetChainIdOfNameCall {
        pub chain_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `getTokenAddress` function with signature `getTokenAddress(uint256,string)` and selector `0x8c7af080`
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
    #[ethcall(name = "getTokenAddress", abi = "getTokenAddress(uint256,string)")]
    pub struct GetTokenAddressWithChainIdCall {
        pub chain_id: ::ethers::core::types::U256,
        pub token_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `getTokenAddress` function with signature `getTokenAddress(string,string)` and selector `0xc35b7547`
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
    #[ethcall(name = "getTokenAddress", abi = "getTokenAddress(string,string)")]
    pub struct GetTokenAddressWithChainNameCall {
        pub chain_name: ::std::string::String,
        pub token_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `getTokenAddress` function with signature `getTokenAddress(string)` and selector `0xc4091236`
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
    #[ethcall(name = "getTokenAddress", abi = "getTokenAddress(string)")]
    pub struct GetTokenAddressCall {
        pub token_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `getTokenNameOfAddress` function with signature `getTokenNameOfAddress(uint256,address)` and selector `0x2515d660`
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
        name = "getTokenNameOfAddress",
        abi = "getTokenNameOfAddress(uint256,address)"
    )]
    pub struct GetTokenNameOfAddressWithChainIdCall {
        pub chain_id: ::ethers::core::types::U256,
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTokenNameOfAddress` function with signature `getTokenNameOfAddress(address)` and selector `0xb399de4e`
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
    #[ethcall(name = "getTokenNameOfAddress", abi = "getTokenNameOfAddress(address)")]
    pub struct GetTokenNameOfAddressCall {
        pub addr: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
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
    ///Container type for all input parameters for the `setChainId` function with signature `setChainId(string,uint256)` and selector `0x6979e27b`
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
    #[ethcall(name = "setChainId", abi = "setChainId(string,uint256)")]
    pub struct SetChainIdCall {
        pub chain_name: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setTokenAddress` function with signature `setTokenAddress(string,address)` and selector `0x17456e56`
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
    #[ethcall(name = "setTokenAddress", abi = "setTokenAddress(string,address)")]
    pub struct SetTokenAddressCall {
        pub token_name: ::std::string::String,
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTokenAddress` function with signature `setTokenAddress(uint256,string,address)` and selector `0xe5be9dec`
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
    #[ethcall(name = "setTokenAddress", abi = "setTokenAddress(uint256,string,address)")]
    pub struct SetTokenAddressWithChainIdAndTokenNameCall {
        pub chain_id: ::ethers::core::types::U256,
        pub token_name: ::std::string::String,
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTokenAddresses` function with signature `setTokenAddresses(uint256,string[],address[])` and selector `0xe0abd91b`
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
        name = "setTokenAddresses",
        abi = "setTokenAddresses(uint256,string[],address[])"
    )]
    pub struct SetTokenAddressesCall {
        pub chain_id: ::ethers::core::types::U256,
        pub token_names: ::std::vec::Vec<::std::string::String>,
        pub addrs: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `tokenNameOfAddress` function with signature `tokenNameOfAddress(uint256,address)` and selector `0xa2a7862e`
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
    #[ethcall(name = "tokenNameOfAddress", abi = "tokenNameOfAddress(uint256,address)")]
    pub struct TokenNameOfAddressCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TokenRegistryCalls {
        AddressOfTokenName(AddressOfTokenNameCall),
        ChainIdOfName(ChainIdOfNameCall),
        GetChainIdOfName(GetChainIdOfNameCall),
        GetTokenAddressWithChainId(GetTokenAddressWithChainIdCall),
        GetTokenAddressWithChainName(GetTokenAddressWithChainNameCall),
        GetTokenAddress(GetTokenAddressCall),
        GetTokenNameOfAddressWithChainId(GetTokenNameOfAddressWithChainIdCall),
        GetTokenNameOfAddress(GetTokenNameOfAddressCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetChainId(SetChainIdCall),
        SetTokenAddress(SetTokenAddressCall),
        SetTokenAddressWithChainIdAndTokenName(
            SetTokenAddressWithChainIdAndTokenNameCall,
        ),
        SetTokenAddresses(SetTokenAddressesCall),
        TokenNameOfAddress(TokenNameOfAddressCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for TokenRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddressOfTokenNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressOfTokenName(decoded));
            }
            if let Ok(decoded) = <ChainIdOfNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChainIdOfName(decoded));
            }
            if let Ok(decoded) = <GetChainIdOfNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChainIdOfName(decoded));
            }
            if let Ok(decoded) = <GetTokenAddressWithChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenAddressWithChainId(decoded));
            }
            if let Ok(decoded) = <GetTokenAddressWithChainNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenAddressWithChainName(decoded));
            }
            if let Ok(decoded) = <GetTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenAddress(decoded));
            }
            if let Ok(decoded) = <GetTokenNameOfAddressWithChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenNameOfAddressWithChainId(decoded));
            }
            if let Ok(decoded) = <GetTokenNameOfAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenNameOfAddress(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
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
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetChainId(decoded));
            }
            if let Ok(decoded) = <SetTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTokenAddress(decoded));
            }
            if let Ok(decoded) = <SetTokenAddressWithChainIdAndTokenNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTokenAddressWithChainIdAndTokenName(decoded));
            }
            if let Ok(decoded) = <SetTokenAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTokenAddresses(decoded));
            }
            if let Ok(decoded) = <TokenNameOfAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenNameOfAddress(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TokenRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressOfTokenName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainIdOfName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainIdOfName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAddressWithChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAddressWithChainName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenNameOfAddressWithChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenNameOfAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTokenAddressWithChainIdAndTokenName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTokenAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenNameOfAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TokenRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressOfTokenName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChainIdOfName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainIdOfName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenAddressWithChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenAddressWithChainName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenNameOfAddressWithChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenNameOfAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTokenAddressWithChainIdAndTokenName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTokenAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenNameOfAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressOfTokenNameCall> for TokenRegistryCalls {
        fn from(value: AddressOfTokenNameCall) -> Self {
            Self::AddressOfTokenName(value)
        }
    }
    impl ::core::convert::From<ChainIdOfNameCall> for TokenRegistryCalls {
        fn from(value: ChainIdOfNameCall) -> Self {
            Self::ChainIdOfName(value)
        }
    }
    impl ::core::convert::From<GetChainIdOfNameCall> for TokenRegistryCalls {
        fn from(value: GetChainIdOfNameCall) -> Self {
            Self::GetChainIdOfName(value)
        }
    }
    impl ::core::convert::From<GetTokenAddressWithChainIdCall> for TokenRegistryCalls {
        fn from(value: GetTokenAddressWithChainIdCall) -> Self {
            Self::GetTokenAddressWithChainId(value)
        }
    }
    impl ::core::convert::From<GetTokenAddressWithChainNameCall> for TokenRegistryCalls {
        fn from(value: GetTokenAddressWithChainNameCall) -> Self {
            Self::GetTokenAddressWithChainName(value)
        }
    }
    impl ::core::convert::From<GetTokenAddressCall> for TokenRegistryCalls {
        fn from(value: GetTokenAddressCall) -> Self {
            Self::GetTokenAddress(value)
        }
    }
    impl ::core::convert::From<GetTokenNameOfAddressWithChainIdCall>
    for TokenRegistryCalls {
        fn from(value: GetTokenNameOfAddressWithChainIdCall) -> Self {
            Self::GetTokenNameOfAddressWithChainId(value)
        }
    }
    impl ::core::convert::From<GetTokenNameOfAddressCall> for TokenRegistryCalls {
        fn from(value: GetTokenNameOfAddressCall) -> Self {
            Self::GetTokenNameOfAddress(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for TokenRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TokenRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for TokenRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for TokenRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetChainIdCall> for TokenRegistryCalls {
        fn from(value: SetChainIdCall) -> Self {
            Self::SetChainId(value)
        }
    }
    impl ::core::convert::From<SetTokenAddressCall> for TokenRegistryCalls {
        fn from(value: SetTokenAddressCall) -> Self {
            Self::SetTokenAddress(value)
        }
    }
    impl ::core::convert::From<SetTokenAddressWithChainIdAndTokenNameCall>
    for TokenRegistryCalls {
        fn from(value: SetTokenAddressWithChainIdAndTokenNameCall) -> Self {
            Self::SetTokenAddressWithChainIdAndTokenName(value)
        }
    }
    impl ::core::convert::From<SetTokenAddressesCall> for TokenRegistryCalls {
        fn from(value: SetTokenAddressesCall) -> Self {
            Self::SetTokenAddresses(value)
        }
    }
    impl ::core::convert::From<TokenNameOfAddressCall> for TokenRegistryCalls {
        fn from(value: TokenNameOfAddressCall) -> Self {
            Self::TokenNameOfAddress(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for TokenRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for TokenRegistryCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for TokenRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    ///Container type for all return fields from the `addressOfTokenName` function with signature `addressOfTokenName(uint256,string)` and selector `0x67cbcb32`
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
    pub struct AddressOfTokenNameReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `chainIdOfName` function with signature `chainIdOfName(string)` and selector `0x1bd5130f`
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
    pub struct ChainIdOfNameReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChainIdOfName` function with signature `getChainIdOfName(string)` and selector `0x304cd040`
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
    pub struct GetChainIdOfNameReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTokenAddress` function with signature `getTokenAddress(uint256,string)` and selector `0x8c7af080`
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
    pub struct GetTokenAddressWithChainIdReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenAddress` function with signature `getTokenAddress(string,string)` and selector `0xc35b7547`
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
    pub struct GetTokenAddressWithChainNameReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenAddress` function with signature `getTokenAddress(string)` and selector `0xc4091236`
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
    pub struct GetTokenAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenNameOfAddress` function with signature `getTokenNameOfAddress(uint256,address)` and selector `0x2515d660`
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
    pub struct GetTokenNameOfAddressWithChainIdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getTokenNameOfAddress` function with signature `getTokenNameOfAddress(address)` and selector `0xb399de4e`
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
    pub struct GetTokenNameOfAddressReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `tokenNameOfAddress` function with signature `tokenNameOfAddress(uint256,address)` and selector `0xa2a7862e`
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
    pub struct TokenNameOfAddressReturn(pub ::std::string::String);
}
