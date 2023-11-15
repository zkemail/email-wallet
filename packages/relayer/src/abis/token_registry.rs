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
            ]),
            events: ::core::convert::From::from([
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\xA2V[`\0`\x03`@Qa\08\x90f\x1BXZ[\x9B\x99]`\xCA\x1B\x81R`\x07\x01\x90V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91Ugoptimism`\xC0\x1B\x81R`\n\x90`\x03\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91Ugarbitrum`\xC0\x1B\x81Ra\xA4\xB1\x90`\x03\x90`\x08\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ua\0\xF2V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x13\xCC\x80a\x01\x01`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xD0W`\x005`\xE0\x1C\x80c\x17EnV\x14a\0\xD5W\x80c\x1B\xD5\x13\x0F\x14a\0\xEAW\x80c%\x15\xD6`\x14a\x01(W\x80c0L\xD0@\x14a\x01HW\x80cg\xCB\xCB2\x14a\x01[W\x80ciy\xE2{\x14a\x01\xB8W\x80cqP\x18\xA6\x14a\x01\xCBW\x80c\x8Cz\xF0\x80\x14a\x01\xD3W\x80c\x8D\xA5\xCB[\x14a\x01\xE6W\x80c\xA2\xA7\x86.\x14a\x01\xEEW\x80c\xB3\x99\xDEN\x14a\x02\x01W\x80c\xC3[uG\x14a\x02\x14W\x80c\xC4\t\x126\x14a\x02'W\x80c\xE0\xAB\xD9\x1B\x14a\x02:W\x80c\xE5\xBE\x9D\xEC\x14a\x02MW\x80c\xF2\xFD\xE3\x8B\x14a\x02`W[`\0\x80\xFD[a\0\xE8a\0\xE36`\x04a\x0ELV[a\x02sV[\0[a\x01\x15a\0\xF86`\x04a\x0E\x99V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x03\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x0166`\x04a\x0E\xD5V[a\x02\x8AV[`@Qa\x01\x1F\x91\x90a\x0F\x1CV[a\x01\x15a\x01V6`\x04a\x0E\x99V[a\x04[V[a\x01\xA0a\x01i6`\x04a\x0FOV[`\x01` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1FV[a\0\xE8a\x01\xC66`\x04a\x0F\x95V[a\x04\x83V[a\0\xE8a\x06\x16V[a\x01\xA0a\x01\xE16`\x04a\x0FOV[a\x06*V[a\x01\xA0a\x08eV[a\x01;a\x01\xFC6`\x04a\x0E\xD5V[a\x08tV[a\x01;a\x02\x0F6`\x04a\x0F\xD9V[a\t\x19V[a\x01\xA0a\x02\"6`\x04a\x0F\xF4V[a\t%V[a\x01\xA0a\x0256`\x04a\x0E\x99V[a\t\xB9V[a\0\xE8a\x02H6`\x04a\x10\xE6V[a\t\xC5V[a\0\xE8a\x02[6`\x04a\x11\xC4V[a\n\x81V[a\0\xE8a\x02n6`\x04a\x0F\xD9V[a\x0C=V[a\x02{a\x0C\xB6V[a\x02\x86F\x83\x83a\n\x81V[PPV[``a\x02\xB2\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\xEBWP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81Rc\n\xE8\xAA\x89`\xE3\x1B` \x82\x01Ra\x04UV[a\x03\x10\x83`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bDAI`\xE8\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03HWP`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbDAI`\xE8\x1B` \x82\x01Ra\x04UV[a\x03n\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cUSDC`\xE0\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03\xA7WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81RcUSDC`\xE0\x1B` \x82\x01Ra\x04UV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80Ta\x03\xD4\x90a\x12\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\0\x90a\x12\x1AV[\x80\x15a\x04MW\x80`\x1F\x10a\x04\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04MV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x040W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0`\x03\x82`@Qa\x04m\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[a\x04\x8Ba\x0C\xB6V[\x80`\0\x03a\x04\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x066\x86\x16\x96\xE2\x06\x96B\x066\x16\xE6\xE6\xF7B\x06&R\x03`d\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\0\x82`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x1BXZ[\x9B\x99]`\xCA\x1B\x81RPa\r\x15V[\x15a\x05KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x18\xD8[\x9B\x9B\xDD\x08\x1C\xD9]\x08\x1BXZ[\x9B\x99]\x08\x18\xDA\x18Z[\x88\x1AY`*\x1B`D\x82\x01R`d\x01a\x04\xCEV[`\x03\x82`@Qa\x05[\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x14a\x05\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x18\xDA\x18Z[\x88\x1AY\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]`b\x1B`D\x82\x01R`d\x01a\x04\xCEV[\x80`\x03\x83`@Qa\x05\xC1\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x80\x82`@Qa\x05\xE2\x91\x90a\x12TV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x86\xA4(l\xD1~\x83w\x05\xC9\x98\x8BbB\xB5\xCD\xD7 \xD82 \x8D\xD8\x85f\x8F\xF0\x0EQM(\xDF\x90`\0\x90\xA3PPV[a\x06\x1Ea\x0C\xB6V[a\x06(`\0a\r+V[V[`\0a\x06Q\x82`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x08\xAA\x89`\xEB\x1B\x81RPa\r\x15V[\x15a\x06vW`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RP\x91P[a\x06\x9C\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RPa\r\x15V[\x15a\x06\xFEW\x82`\0\x03a\x06\xC4WPs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2a\x04UV[\x82`\n\x03a\x06\xDAWP`\x06`!`\x99\x1B\x01a\x04UV[\x82a\xA4\xB1\x03a\x06\xFEWPs\x82\xAFID}\x8A\x07\xE3\xBD\x95\xBD\rV\xF3RAR?\xBA\xB1a\x04UV[a\x07#\x82`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bDAI`\xE8\x1B\x81RPa\r\x15V[\x15a\x07\x92W\x82`\0\x03a\x07KWPsk\x17Tt\xE8\x90\x94\xC4M\xA9\x8B\x95N\xED\xEA\xC4\x95'\x1D\x0Fa\x04UV[\x82`\n\x03a\x07nWPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x04UV[\x82a\xA4\xB1\x03a\x07\x92WPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x04UV[a\x07\xB8\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cUSDC`\xE0\x1B\x81RPa\r\x15V[\x15a\x08'W\x82`\0\x03a\x07\xE0WPs\xA0\xB8i\x91\xC6!\x8B6\xC1\xD1\x9DJ.\x9E\xB0\xCE6\x06\xEBHa\x04UV[\x82`\n\x03a\x08\x03WPs\x7F\\vL\xBC\x14\xF9f\x9B\x88\x83|\xA1I\x0C\xCA\x17\xC3\x16\x07a\x04UV[\x82a\xA4\xB1\x03a\x08'WPs\xAF\x88\xD0e\xE7|\x8C\xC2#\x93'\xC5\xED\xB3\xA42&\x8EX1a\x04UV[`\0\x83\x81R`\x01` R`@\x90\x81\x90 \x90Qa\x08D\x90\x84\x90a\x12TV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x08\x98\x90a\x12\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC4\x90a\x12\x1AV[\x80\x15a\t\x11W\x80`\x1F\x10a\x08\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x04UF\x83a\x02\x8AV[`\0`\x03\x83`@Qa\t7\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x03a\t\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqunknown chain name`p\x1B`D\x82\x01R`d\x01a\x04\xCEV[a\t\xB2`\x03\x84`@Qa\t\x9D\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x83a\x06*V[\x93\x92PPPV[`\0a\x04UF\x83a\x06*V[a\t\xCDa\x0C\xB6V[\x80Q\x82Q\x14a\n*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FtokenNames and addrs length mism`D\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`d\x82\x01R`\x84\x01a\x04\xCEV[`\0[\x82Q\x81\x10\x15a\n{Wa\ns\x84\x84\x83\x81Q\x81\x10a\nLWa\nLa\x12pV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\nfWa\nfa\x12pV[` \x02` \x01\x01Qa\n\x81V[`\x01\x01a\n-V[PPPPV[a\n\x89a\x0C\xB6V[`\0\x83\x81R`\x01` R`@\x80\x82 \x90Qa\n\xA5\x90\x85\x90a\x12TV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x15\x1B\xDA\xD9[\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`B\x1B`D\x82\x01R`d\x01a\x04\xCEV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80Ta\x0B3\x90a\x12\x1AV[\x15\x90Pa\x0B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x10Y\x19\x1C\x99\\\xDC\xC8\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01a\x04\xCEV[\x80`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x83`@Qa\x0B\xA1\x91\x90a\x12TV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U`\0\x86\x81R`\x02\x83R\x81\x81 \x93\x85\x16\x81R\x92\x90\x91R\x90 a\x0B\xED\x83\x82a\x12\xD7V[P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x0C\x06\x91\x90a\x12TV[`@Q\x90\x81\x90\x03\x81 \x90\x85\x90\x7Fi$\xCDB\xAC\x86\x94\xED_A\xAA29\xEE\x8E\xD5\x9C\xDB90\x13\xC2\x97u\xF1\xEF\xF5\xF4\xE0\xA6\xBC\xF9\x90`\0\x90\xA4PPPV[a\x0CEa\x0C\xB6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04\xCEV[a\x0C\xB3\x81a\r+V[PV[3a\x0C\xBFa\x08eV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\xCEV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\r\xB9Wa\r\xB9a\r{V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\r\xD2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xEBWa\r\xEBa\r{V[a\r\xFE`\x1F\x82\x01`\x1F\x19\x16` \x01a\r\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\x13W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EGW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E_W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EuW`\0\x80\xFD[a\x0E\x81\x85\x82\x86\x01a\r\xC1V[\x92PPa\x0E\x90` \x84\x01a\x0E0V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xABW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC1W`\0\x80\xFD[a\x0E\xCD\x84\x82\x85\x01a\r\xC1V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xE8W`\0\x80\xFD[\x825\x91Pa\x0E\x90` \x84\x01a\x0E0V[`\0[\x83\x81\x10\x15a\x0F\x13W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E\xFBV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0F;\x81`@\x85\x01` \x87\x01a\x0E\xF8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FbW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\x7FW`\0\x80\xFD[a\x0F\x8B\x85\x82\x86\x01a\r\xC1V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xA8W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xBEW`\0\x80\xFD[a\x0F\xCA\x85\x82\x86\x01a\r\xC1V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xEBW`\0\x80\xFD[a\t\xB2\x82a\x0E0V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x07W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\x1EW`\0\x80\xFD[a\x10*\x86\x83\x87\x01a\r\xC1V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x10@W`\0\x80\xFD[Pa\x0F\x8B\x85\x82\x86\x01a\r\xC1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x10fWa\x10fa\r{V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\x81W`\0\x80\xFD[\x815` a\x10\x96a\x10\x91\x83a\x10MV[a\r\x91V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x10\xB8W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x10\xDBWa\x10\xCE\x81a\x0E0V[\x83R\x91\x83\x01\x91\x83\x01a\x10\xBDV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xFBW`\0\x80\xFD[\x835\x92P` \x80\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x1AW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11.W`\0\x80\xFD[\x815a\x11<a\x10\x91\x82a\x10MV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x11[W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x11\x93W\x805\x85\x81\x11\x15a\x11wW`\0\x80\x81\xFD[a\x11\x85\x8D\x89\x83\x8A\x01\x01a\r\xC1V[\x84RP\x91\x86\x01\x91\x86\x01a\x11_V[P\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x11\xACW`\0\x80\xFD[PPa\x11\xBA\x86\x82\x87\x01a\x10pV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xD9W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xF6W`\0\x80\xFD[a\x12\x02\x86\x82\x87\x01a\r\xC1V[\x92PPa\x12\x11`@\x85\x01a\x0E0V[\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12.W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x12NWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa\x12f\x81\x84` \x87\x01a\x0E\xF8V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x12\xD2W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x12\xAFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\xCEW\x82\x81U`\x01\x01a\x12\xBBV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF0Wa\x12\xF0a\r{V[a\x13\x04\x81a\x12\xFE\x84Ta\x12\x1AV[\x84a\x12\x86V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x139W`\0\x84\x15a\x13!WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x12\xCEV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x13hW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x13IV[P\x85\x82\x10\x15a\x13\x86W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x1AP\xCC\xC3\x11\xC6\x0F\x15\x83\x07\x17\xE6d\x1A-\xF5z\xE3E}\xB3\xA6{Ia{\xFC\\\xF2\xAF\xFAodsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static TOKENREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xD0W`\x005`\xE0\x1C\x80c\x17EnV\x14a\0\xD5W\x80c\x1B\xD5\x13\x0F\x14a\0\xEAW\x80c%\x15\xD6`\x14a\x01(W\x80c0L\xD0@\x14a\x01HW\x80cg\xCB\xCB2\x14a\x01[W\x80ciy\xE2{\x14a\x01\xB8W\x80cqP\x18\xA6\x14a\x01\xCBW\x80c\x8Cz\xF0\x80\x14a\x01\xD3W\x80c\x8D\xA5\xCB[\x14a\x01\xE6W\x80c\xA2\xA7\x86.\x14a\x01\xEEW\x80c\xB3\x99\xDEN\x14a\x02\x01W\x80c\xC3[uG\x14a\x02\x14W\x80c\xC4\t\x126\x14a\x02'W\x80c\xE0\xAB\xD9\x1B\x14a\x02:W\x80c\xE5\xBE\x9D\xEC\x14a\x02MW\x80c\xF2\xFD\xE3\x8B\x14a\x02`W[`\0\x80\xFD[a\0\xE8a\0\xE36`\x04a\x0ELV[a\x02sV[\0[a\x01\x15a\0\xF86`\x04a\x0E\x99V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x03\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x0166`\x04a\x0E\xD5V[a\x02\x8AV[`@Qa\x01\x1F\x91\x90a\x0F\x1CV[a\x01\x15a\x01V6`\x04a\x0E\x99V[a\x04[V[a\x01\xA0a\x01i6`\x04a\x0FOV[`\x01` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1FV[a\0\xE8a\x01\xC66`\x04a\x0F\x95V[a\x04\x83V[a\0\xE8a\x06\x16V[a\x01\xA0a\x01\xE16`\x04a\x0FOV[a\x06*V[a\x01\xA0a\x08eV[a\x01;a\x01\xFC6`\x04a\x0E\xD5V[a\x08tV[a\x01;a\x02\x0F6`\x04a\x0F\xD9V[a\t\x19V[a\x01\xA0a\x02\"6`\x04a\x0F\xF4V[a\t%V[a\x01\xA0a\x0256`\x04a\x0E\x99V[a\t\xB9V[a\0\xE8a\x02H6`\x04a\x10\xE6V[a\t\xC5V[a\0\xE8a\x02[6`\x04a\x11\xC4V[a\n\x81V[a\0\xE8a\x02n6`\x04a\x0F\xD9V[a\x0C=V[a\x02{a\x0C\xB6V[a\x02\x86F\x83\x83a\n\x81V[PPV[``a\x02\xB2\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\xEBWP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81Rc\n\xE8\xAA\x89`\xE3\x1B` \x82\x01Ra\x04UV[a\x03\x10\x83`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bDAI`\xE8\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03HWP`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbDAI`\xE8\x1B` \x82\x01Ra\x04UV[a\x03n\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cUSDC`\xE0\x1B\x81RPa\x06*V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03\xA7WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81RcUSDC`\xE0\x1B` \x82\x01Ra\x04UV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80Ta\x03\xD4\x90a\x12\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\0\x90a\x12\x1AV[\x80\x15a\x04MW\x80`\x1F\x10a\x04\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04MV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x040W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0`\x03\x82`@Qa\x04m\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[a\x04\x8Ba\x0C\xB6V[\x80`\0\x03a\x04\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x066\x86\x16\x96\xE2\x06\x96B\x066\x16\xE6\xE6\xF7B\x06&R\x03`d\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\0\x82`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x1BXZ[\x9B\x99]`\xCA\x1B\x81RPa\r\x15V[\x15a\x05KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x18\xD8[\x9B\x9B\xDD\x08\x1C\xD9]\x08\x1BXZ[\x9B\x99]\x08\x18\xDA\x18Z[\x88\x1AY`*\x1B`D\x82\x01R`d\x01a\x04\xCEV[`\x03\x82`@Qa\x05[\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x14a\x05\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x18\xDA\x18Z[\x88\x1AY\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]`b\x1B`D\x82\x01R`d\x01a\x04\xCEV[\x80`\x03\x83`@Qa\x05\xC1\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x80\x82`@Qa\x05\xE2\x91\x90a\x12TV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x86\xA4(l\xD1~\x83w\x05\xC9\x98\x8BbB\xB5\xCD\xD7 \xD82 \x8D\xD8\x85f\x8F\xF0\x0EQM(\xDF\x90`\0\x90\xA3PPV[a\x06\x1Ea\x0C\xB6V[a\x06(`\0a\r+V[V[`\0a\x06Q\x82`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x08\xAA\x89`\xEB\x1B\x81RPa\r\x15V[\x15a\x06vW`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RP\x91P[a\x06\x9C\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\n\xE8\xAA\x89`\xE3\x1B\x81RPa\r\x15V[\x15a\x06\xFEW\x82`\0\x03a\x06\xC4WPs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2a\x04UV[\x82`\n\x03a\x06\xDAWP`\x06`!`\x99\x1B\x01a\x04UV[\x82a\xA4\xB1\x03a\x06\xFEWPs\x82\xAFID}\x8A\x07\xE3\xBD\x95\xBD\rV\xF3RAR?\xBA\xB1a\x04UV[a\x07#\x82`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bDAI`\xE8\x1B\x81RPa\r\x15V[\x15a\x07\x92W\x82`\0\x03a\x07KWPsk\x17Tt\xE8\x90\x94\xC4M\xA9\x8B\x95N\xED\xEA\xC4\x95'\x1D\x0Fa\x04UV[\x82`\n\x03a\x07nWPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x04UV[\x82a\xA4\xB1\x03a\x07\x92WPs\xDA\x10\0\x9C\xBD]\x07\xDD\x0C\xEC\xC6aa\xFC\x93\xD7\xC9\0\r\xA1a\x04UV[a\x07\xB8\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cUSDC`\xE0\x1B\x81RPa\r\x15V[\x15a\x08'W\x82`\0\x03a\x07\xE0WPs\xA0\xB8i\x91\xC6!\x8B6\xC1\xD1\x9DJ.\x9E\xB0\xCE6\x06\xEBHa\x04UV[\x82`\n\x03a\x08\x03WPs\x7F\\vL\xBC\x14\xF9f\x9B\x88\x83|\xA1I\x0C\xCA\x17\xC3\x16\x07a\x04UV[\x82a\xA4\xB1\x03a\x08'WPs\xAF\x88\xD0e\xE7|\x8C\xC2#\x93'\xC5\xED\xB3\xA42&\x8EX1a\x04UV[`\0\x83\x81R`\x01` R`@\x90\x81\x90 \x90Qa\x08D\x90\x84\x90a\x12TV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x08\x98\x90a\x12\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC4\x90a\x12\x1AV[\x80\x15a\t\x11W\x80`\x1F\x10a\x08\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x04UF\x83a\x02\x8AV[`\0`\x03\x83`@Qa\t7\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`\0\x03a\t\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqunknown chain name`p\x1B`D\x82\x01R`d\x01a\x04\xCEV[a\t\xB2`\x03\x84`@Qa\t\x9D\x91\x90a\x12TV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x83a\x06*V[\x93\x92PPPV[`\0a\x04UF\x83a\x06*V[a\t\xCDa\x0C\xB6V[\x80Q\x82Q\x14a\n*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FtokenNames and addrs length mism`D\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`d\x82\x01R`\x84\x01a\x04\xCEV[`\0[\x82Q\x81\x10\x15a\n{Wa\ns\x84\x84\x83\x81Q\x81\x10a\nLWa\nLa\x12pV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\nfWa\nfa\x12pV[` \x02` \x01\x01Qa\n\x81V[`\x01\x01a\n-V[PPPPV[a\n\x89a\x0C\xB6V[`\0\x83\x81R`\x01` R`@\x80\x82 \x90Qa\n\xA5\x90\x85\x90a\x12TV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x15\x1B\xDA\xD9[\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`B\x1B`D\x82\x01R`d\x01a\x04\xCEV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80Ta\x0B3\x90a\x12\x1AV[\x15\x90Pa\x0B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x10Y\x19\x1C\x99\\\xDC\xC8\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01a\x04\xCEV[\x80`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x83`@Qa\x0B\xA1\x91\x90a\x12TV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U`\0\x86\x81R`\x02\x83R\x81\x81 \x93\x85\x16\x81R\x92\x90\x91R\x90 a\x0B\xED\x83\x82a\x12\xD7V[P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x0C\x06\x91\x90a\x12TV[`@Q\x90\x81\x90\x03\x81 \x90\x85\x90\x7Fi$\xCDB\xAC\x86\x94\xED_A\xAA29\xEE\x8E\xD5\x9C\xDB90\x13\xC2\x97u\xF1\xEF\xF5\xF4\xE0\xA6\xBC\xF9\x90`\0\x90\xA4PPPV[a\x0CEa\x0C\xB6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04\xCEV[a\x0C\xB3\x81a\r+V[PV[3a\x0C\xBFa\x08eV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\xCEV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\r\xB9Wa\r\xB9a\r{V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\r\xD2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xEBWa\r\xEBa\r{V[a\r\xFE`\x1F\x82\x01`\x1F\x19\x16` \x01a\r\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\x13W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EGW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E_W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EuW`\0\x80\xFD[a\x0E\x81\x85\x82\x86\x01a\r\xC1V[\x92PPa\x0E\x90` \x84\x01a\x0E0V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xABW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC1W`\0\x80\xFD[a\x0E\xCD\x84\x82\x85\x01a\r\xC1V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xE8W`\0\x80\xFD[\x825\x91Pa\x0E\x90` \x84\x01a\x0E0V[`\0[\x83\x81\x10\x15a\x0F\x13W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E\xFBV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0F;\x81`@\x85\x01` \x87\x01a\x0E\xF8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FbW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\x7FW`\0\x80\xFD[a\x0F\x8B\x85\x82\x86\x01a\r\xC1V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xA8W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xBEW`\0\x80\xFD[a\x0F\xCA\x85\x82\x86\x01a\r\xC1V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xEBW`\0\x80\xFD[a\t\xB2\x82a\x0E0V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x07W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\x1EW`\0\x80\xFD[a\x10*\x86\x83\x87\x01a\r\xC1V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x10@W`\0\x80\xFD[Pa\x0F\x8B\x85\x82\x86\x01a\r\xC1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x10fWa\x10fa\r{V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\x81W`\0\x80\xFD[\x815` a\x10\x96a\x10\x91\x83a\x10MV[a\r\x91V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x10\xB8W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x10\xDBWa\x10\xCE\x81a\x0E0V[\x83R\x91\x83\x01\x91\x83\x01a\x10\xBDV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xFBW`\0\x80\xFD[\x835\x92P` \x80\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x1AW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11.W`\0\x80\xFD[\x815a\x11<a\x10\x91\x82a\x10MV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x11[W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x11\x93W\x805\x85\x81\x11\x15a\x11wW`\0\x80\x81\xFD[a\x11\x85\x8D\x89\x83\x8A\x01\x01a\r\xC1V[\x84RP\x91\x86\x01\x91\x86\x01a\x11_V[P\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x11\xACW`\0\x80\xFD[PPa\x11\xBA\x86\x82\x87\x01a\x10pV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xD9W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xF6W`\0\x80\xFD[a\x12\x02\x86\x82\x87\x01a\r\xC1V[\x92PPa\x12\x11`@\x85\x01a\x0E0V[\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12.W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x12NWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qa\x12f\x81\x84` \x87\x01a\x0E\xF8V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x12\xD2W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x12\xAFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\xCEW\x82\x81U`\x01\x01a\x12\xBBV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF0Wa\x12\xF0a\r{V[a\x13\x04\x81a\x12\xFE\x84Ta\x12\x1AV[\x84a\x12\x86V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x139W`\0\x84\x15a\x13!WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x12\xCEV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x13hW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x13IV[P\x85\x82\x10\x15a\x13\x86W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x1AP\xCC\xC3\x11\xC6\x0F\x15\x83\x07\x17\xE6d\x1A-\xF5z\xE3E}\xB3\xA6{Ia{\xFC\\\xF2\xAF\xFAodsolcC\0\x08\x17\x003";
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TokenRegistryEvents {
        ChainRegisteredFilter(ChainRegisteredFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TokenRegisteredFilter(TokenRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for TokenRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChainRegisteredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::ChainRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TokenRegisteredFilter::decode_log(log) {
                return Ok(TokenRegistryEvents::TokenRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TokenRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChainRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ChainRegisteredFilter> for TokenRegistryEvents {
        fn from(value: ChainRegisteredFilter) -> Self {
            Self::ChainRegisteredFilter(value)
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
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetChainId(SetChainIdCall),
        SetTokenAddress(SetTokenAddressCall),
        SetTokenAddressWithChainIdAndTokenName(
            SetTokenAddressWithChainIdAndTokenNameCall,
        ),
        SetTokenAddresses(SetTokenAddressesCall),
        TokenNameOfAddress(TokenNameOfAddressCall),
        TransferOwnership(TransferOwnershipCall),
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
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
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
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<OwnerCall> for TokenRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
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
