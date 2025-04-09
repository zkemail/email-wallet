pub use extension_handler::*;
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
pub mod extension_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressOfExtensionName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addressOfExtensionName",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("defaultExtensionOfCommand"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultExtensionOfCommand",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getExtensionForCommand"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getExtensionForCommand",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("command"),
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
                    ::std::borrow::ToOwned::to_owned("getSubjectTemplatesOfExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubjectTemplatesOfExtension",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensionAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[][]"),
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
                    ::std::borrow::ToOwned::to_owned("maxGasOfExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxGasOfExtension"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("publishExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("publishExtension"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subjectTemplates"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxExecutionGas"),
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
                    ::std::borrow::ToOwned::to_owned("setDefaultExtensions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDefaultExtensions",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("setExtensionForCommand"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setExtensionForCommand",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("command"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensionAddr"),
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
                    ::std::borrow::ToOwned::to_owned("subjectTemplatesOfExtension"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "subjectTemplatesOfExtension",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                (
                    ::std::borrow::ToOwned::to_owned("userExtensionOfCommand"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userExtensionOfCommand",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("ExtensionPublished"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExtensionPublished"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extensionAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subjectTemplates"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maxExecutionGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
    pub static EXTENSIONHANDLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x13W__\xFD[Pa\0\x1Ca\0!V[a\0\xDDV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\0\xDBW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x80Qa/\xB9a\x01\x11_9_\x81\x81a\x04\x0B\x01R\x81\x81a\x04\xA6\x01R\x81\x81a\x05\xE0\x01R\x81\x81a\x06v\x01Ra\x07\xA4\x01Ra/\xB9_\xF3\xFE`\x80`@R`\x046\x10a\x01\tW_5`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xA1W\x80c\xBC'\xAE\xB3\x11a\0qW\x80c\xF1\xFB\xBB\xA1\x11a\0WW\x80c\xF1\xFB\xBB\xA1\x14a\x03\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xB6W\x80c\xF8\x84c\x0B\x14a\x03\xD5W__\xFD[\x80c\xBC'\xAE\xB3\x14a\x03YW\x80c\xE0\x86\xA8\"\x14a\x03xW__\xFD[\x80c\x81)\xFC\x1C\x14a\x02\xA2W\x80c\x8D\xA5\xCB[\x14a\x02\xB6W\x80c\x93\xECIq\x14a\x02\xE0W\x80c\x93\xF8xB\x14a\x03\x0CW__\xFD[\x80cO\x1E\xF2\x86\x11a\0\xDCW\x80cO\x1E\xF2\x86\x14a\x02;W\x80cR\xD1\x90-\x14a\x02NW\x80cm\xFB\xE3^\x14a\x02bW\x80cqP\x18\xA6\x14a\x02\x8EW__\xFD[\x80c\r\xB7\x9B\x93\x14a\x01\rW\x80c/\x1D}\xDC\x14a\x01\x84W\x80c6Y\xCF\xE6\x14a\x01\xBDW\x80c:\x03s\x8A\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01\x18W__\xFD[Pa\x01Za\x01'6`\x04a&\x85V[\x80Q` \x81\x83\x01\x81\x01\x80Q`g\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x8FW__\xFD[Pa\x01\xAFa\x01\x9E6`\x04a&\xE8V[`i` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01{V[4\x80\x15a\x01\xC8W__\xFD[Pa\x01\xDCa\x01\xD76`\x04a&\xE8V[a\x03\xF4V[\0[4\x80\x15a\x01\xE9W__\xFD[Pa\x01Za\x01\xF86`\x04a'\x03V[`j` \x90\x81R_\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xDCa\x02I6`\x04a'PV[a\x05\xC9V[4\x80\x15a\x02YW__\xFD[Pa\x01\xAFa\x07\x8BV[4\x80\x15a\x02mW__\xFD[Pa\x02\x81a\x02|6`\x04a&\xE8V[a\x08\\V[`@Qa\x01{\x91\x90a(\xCEV[4\x80\x15a\x02\x99W__\xFD[Pa\x01\xDCa\t\x97V[4\x80\x15a\x02\xADW__\xFD[Pa\x01\xDCa\t\xAAV[4\x80\x15a\x02\xC1W__\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01ZV[4\x80\x15a\x02\xEBW__\xFD[Pa\x02\xFFa\x02\xFA6`\x04a(\xE0V[a\x0BIV[`@Qa\x01{\x91\x90a)\x12V[4\x80\x15a\x03\x17W__\xFD[Pa\x01Za\x03&6`\x04a&\x85V[\x80Q` \x81\x83\x01\x81\x01\x80Q`f\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x03dW__\xFD[Pa\x01\xDCa\x03s6`\x04a*RV[a\x0C\x10V[4\x80\x15a\x03\x83W__\xFD[Pa\x01Za\x03\x926`\x04a'\x03V[a\x19FV[4\x80\x15a\x03\xA2W__\xFD[Pa\x01\xDCa\x03\xB16`\x04a+\0V[a\x19\xF3V[4\x80\x15a\x03\xC1W__\xFD[Pa\x01\xDCa\x03\xD06`\x04a&\xE8V[a\x1A\x8AV[4\x80\x15a\x03\xE0W__\xFD[Pa\x01\xDCa\x03\xEF6`\x04a+`V[a\x1B$V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x04\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\x19\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x05\xAB\x81a\x1DDV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92Ra\x05\xC6\x91\x83\x91\x90a\x1D\xB0V[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x06tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\xE9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x07{\x82a\x1DDV[a\x07\x87\x82\x82`\x01a\x1D\xB0V[PPV[_0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x087W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16_\x90\x81R`h` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\t\x8CW\x83\x82\x90_R` _ \x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\tyW\x83\x82\x90_R` _ \x01\x80Ta\x08\xEE\x90a+\xD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x1A\x90a+\xD1V[\x80\x15a\teW\x80`\x1F\x10a\t<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\teV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tHW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\xD1V[PPPP\x81R` \x01\x90`\x01\x01\x90a\x08\xA0V[PPPP\x90P\x91\x90PV[a\t\x9Fa\x1F\x80V[a\t\xA8_a\x1F\xE7V[V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xC8WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xE1WP0;\x15\x80\x15a\t\xE1WP_T`\xFF\x16`\x01\x14[a\nSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\n\xAFW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\n\xB7a ]V[`e\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x163a\x01\0\x02\x17\x90U\x80\x15a\x05\xC6W_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`h` R\x82_R`@_ \x82\x81T\x81\x10a\x0BbW_\x80\xFD[\x90_R` _ \x01\x81\x81T\x81\x10a\x0BwW_\x80\xFD[\x90_R` _ \x01_\x92P\x92PPP\x80Ta\x0B\x91\x90a+\xD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xBD\x90a+\xD1V[\x80\x15a\x0C\x08W\x80`\x1F\x10a\x0B\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x08V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`f\x86\x86`@Qa\x0C9\x92\x91\x90a,\"V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fextension name already used\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid extension address\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[\x83a\r\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Finvalid extension name\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[_\x81\x11a\r\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FmaxExecutionGas must be larger t`D\x82\x01R\x7Fhan zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_\x82Q\x11a\x0EGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FsubjectTemplates array cannot be`D\x82\x01R\x7F empty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`i` R`@\x90 T\x15a\x0E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fextension already published\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[``_[\x83Q\x81\x10\x15a\x11\x88W_\x84\x82\x81Q\x81\x10a\x0E\xD9Wa\x0E\xD9a,1V[` \x02` \x01\x01QQ\x11a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FsubjectTemplate cannot be empty\0`D\x82\x01R`d\x01a\x04\x9BV[\x80_\x03a\x0FpW\x83\x81\x81Q\x81\x10a\x0FHWa\x0FHa,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x0FaWa\x0Faa,1V[` \x02` \x01\x01Q\x91Pa\x10/V[a\x0F\xBD\x82\x85\x83\x81Q\x81\x10a\x0F\x86Wa\x0F\x86a,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x0F\x9FWa\x0F\x9Fa,1V[` \x02` \x01\x01Q\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a\x10/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FsubjectTemplates must have same `D\x82\x01R\x7Fcommand\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_`\x01[\x85\x83\x81Q\x81\x10a\x10EWa\x10Ea,1V[` \x02` \x01\x01QQ\x81\x10\x15a\x11\x07Wa\x10\xEC\x86\x84\x81Q\x81\x10a\x10jWa\x10ja,1V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x10\x83Wa\x10\x83a,1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81R\x7F{recipient}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x80Q\x91\x01 \x7F\x8A\xAF2\xFC\xBD\x17C\x99\xCB6i\x03n\x8D\xA3\xC3W\x80\x02\xBC\xCE\x06JJ\x8F)\xF7\xB5gK\xBC\xAF\x14\x90V[\x15a\x10\xFFW\x81a\x10\xFB\x81a,^V[\x92PP[`\x01\x01a\x103V[P`\x01\x81\x11\x15a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Frecipient template can only be u`D\x82\x01R\x7Fsed once\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[P`\x01\x01a\x0E\xBDV[P_[\x81Q\x81\x10\x15a\x12CW\x81\x81\x81Q\x81\x10a\x11\xA6Wa\x11\xA6a,1V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\x12;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fcommand should be one word\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[`\x01\x01a\x11\x8BV[P`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FSend\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xA8\x86\xDD\x9B\xF7J#R\x1A{\x7F\xF5\xD4\x19\xD5\xA1\xC9\xE5>K\x8C\x03\0\x12\x96\x7F\xAB\xD9\xBEw\x82\x19\x14\x15\x80\x15a\x13\x08WP`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7FExecute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F(\xD6h\xBE\xB2\x86\xA0\x83\x9CR\x82V\x18\xAB\xB59\xCB\xE2t\x8571\xDB@\x04\xCA{\xCF\x10I\xA4\xCA\x14\x15[\x80\x15a\x13nWP`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7FInstall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Ff\x97\xC6:t\xFC\xC5\xC4d\xF3\xDC\xB6\xB3\xA4\xE4\x9C\xB1\xD1\0=\xA9\x90H\xD4Lm+\x12\xFE\0\xFF:\x14\x15[\x80\x15a\x13\xD4WP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FUninstall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xCF\x0F\xB4\xAFo\xA7\x12\x03\xBF\x8C\xA4\x9A\x15\xD70\xB9\xE9\xA08ZR\x96\x1Eu\xDE\xD3\x07\x18\xD6\x9A\x94N\x14\x15[\x80\x15a\x14:WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FExit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F$\xF7?K\t\x82\xCE\xA5@\x8Ev\x08\x8D\x18 \xDC\xFC\xE3\xFD?\xB6^\x92=\xCB\x98y\xC1\xE7\xE9\x11A\x14\x15[\x80\x15a\x14\xA0WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FDKIM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fc\xCA\xED\x16\x123\x96\xDD\xFB\xCC\x1E\x89:\xD0\xC1\x8E\xC4j%H\xDB\xEBQ\x18u#h5d9\xF0\xAF\x14\x15[a\x15\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcommand cannot be a reserved nam`D\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7F{tokenAmount}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x81_\xF0\x03\xEC<\xD9\x03\xB6\xB6o\xF0\xF9[T/\xD3\xA9=-\x90i<\r\x0Ck\xEA\xA3<\xCA\xAF+\x14\x15\x80\x15a\x15\xD6WP`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81R\x7F{amount}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fo\x83g\xB1\xD9\xD8\xA7\xB8-\xD8\xC3\xFE\xA7Sb\xC0aF\xBC\xA0q\x11\x9C\xF5\x9A<\xB5\x85\x01\x07k\xF0\x14\x15[\x80\x15a\x16<WP`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81R\x7F{string}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xB0\xDD\x9C]\xFDk\x13H\x08\x959\xC8\xCD\x81F\xA5\x9F\x1F\xD2?\xF2\xDE\x9C`R\xE5M\xA8\xD2\xA6\xC0\xFB\x14\x15[\x80\x15a\x16\xA2WP`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81R\x7F{uint}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fo^\xA6\xF4\x05\xF6a\xD5\x06k\x9E\x0F\xF0z%\xFD.\r `W\xA7\xFC-\xFE\xF3?\xF6Z\xD2*#\x14\x15[\x80\x15a\x17\x08WP`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F{int}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x96\x11\\R(\x17\x05\0M\xB3\xCA\x7F`A\x12\xB6\xBCv\xAEP^\xD2h\xB2\xDB\xEE)\xFE\xB8\xE7\x89\x9D\x14\x15[\x80\x15a\x17nWP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7F{address}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x9A=\xE9\xFC\xA4o$ \x94\x7F-\x1DGD\xC5\xFE\xBE\xDE\xBCw\xB9\xA3\xFC\xBF\xCF\x16\xED\x02\xCB\xE4\x8Fs\x14\x15[\x80\x15a\x17\xD4WP`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81R\x7F{recipient}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x8A\xAF2\xFC\xBD\x17C\x99\xCB6i\x03n\x8D\xA3\xC3W\x80\x02\xBC\xCE\x06JJ\x8F)\xF7\xB5gK\xBC\xAF\x14\x15[a\x18EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcommand cannot be a template mat`D\x82\x01R\x7Fcher\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x83`f\x87\x87`@Qa\x18X\x92\x91\x90a,\"V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U\x91\x86\x16_\x90\x81R`h\x82R\x91\x90\x91 \x84Qa\x18\xC5\x92\x86\x01\x90a$0V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x81\x81R`i` R`@\x90\x81\x90 \x84\x90UQa\x18\xFD\x90\x88\x90\x88\x90a,\"V[`@Q\x80\x91\x03\x90 \x7F\xEF\x0E\x97\x1FC\xD94~\xAD|\xB6g\x91\xFD\xCC\xE2\xD7\xA5\x17\x1E\xE1\xF6R\xFE\xC7'\x82=\x1E-N\xE8\x85\x85`@Qa\x196\x92\x91\x90a,\xBAV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`j` R`@\x80\x82 \x90Q\x82\x91\x82\x91a\x19|\x90\x86\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x15a\x19\xB2W\x80\x91Pa\x19\xEBV[`g\x84`@Qa\x19\xC2\x91\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P[P\x93\x92PPPV[a\x19\xFBa\x1F\x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`j` R`@\x90\x81\x90 \x90Q\x82\x91\x90a\x1A1\x90\x85\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x1A\x92a\x1F\x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x05\xC6\x81a\x1F\xE7V[a\x1B,a\x1F\x80V[`eT`\xFF\x16\x15a\x1B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fdefault extensions already set\0\0`D\x82\x01R`d\x01a\x04\x9BV[_[\x81\x81\x10\x15a\x1D\x14W____\x86\x86\x86\x81\x81\x10a\x1B\x9FWa\x1B\x9Fa,1V[\x90P` \x02\x81\x01\x90a\x1B\xB1\x91\x90a,\xFDV[\x81\x01\x90a\x1B\xBE\x91\x90a-eV[\x93P\x93P\x93P\x93P\x82`f\x85`@Qa\x1B\xD7\x91\x90a,\xF2V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U\x91\x85\x16_\x90\x81R`h\x82R\x91\x90\x91 \x83Qa\x1CD\x92\x85\x01\x90a$0V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`i` R`@\x81 \x82\x90U\x82Q\x84\x91`g\x91\x85\x91\x90a\x1C\x82Wa\x1C\x82a,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x1C\x9BWa\x1C\x9Ba,1V[` \x02` \x01\x01Q`@Qa\x1C\xB0\x91\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90UPPP`\x01\x91\x90\x91\x01\x90Pa\x1B\x81V[PP`e\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UPV[`eTa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fcaller is not a deployer\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1D\xE8Wa\x1D\xE3\x83a \xE1V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1EmWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1Ej\x91\x81\x01\x90a-\xE3V[`\x01[a\x1E\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1FtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[Pa\x1D\xE3\x83\x83\x83a!\xD1V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\x9BV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a \xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\t\xA8a!\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a!kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a!\xDA\x83a\"\x80V[_\x82Q\x11\x80a!\xE6WP\x80[\x15a\x1D\xE3Wa!\xF5\x83\x83a\"\xCCV[PPPPV[_Ta\x01\0\x90\x04`\xFF\x16a\"wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\t\xA83a\x1F\xE7V[a\"\x89\x81a \xE1V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\"\xF1\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a/]`'\x919a\"\xF8V[\x93\x92PPPV[``__\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa#!\x91\x90a,\xF2V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a#YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a#^V[``\x91P[P\x91P\x91Pa#o\x86\x83\x83\x87a#yV[\x96\x95PPPPPPV[``\x83\x15a#\xF4W\x82Q_\x03a#\xEDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[P\x81a#\xFEV[a#\xFE\x83\x83a$\x06V[\x94\x93PPPPV[\x81Q\x15a$\x16W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x9B\x91\x90a)\x12V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a${W\x91` \x02\x82\x01[\x82\x81\x11\x15a${W\x82Q\x80Qa$k\x91\x84\x91` \x90\x91\x01\x90a$\x8BV[P\x91` \x01\x91\x90`\x01\x01\x90a$NV[Pa$\x87\x92\x91Pa$\xDBV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a$\xCFW\x91` \x02\x82\x01[\x82\x81\x11\x15a$\xCFW\x82Q\x82\x90a$\xBF\x90\x82a.EV[P\x91` \x01\x91\x90`\x01\x01\x90a$\xA9V[Pa$\x87\x92\x91Pa$\xF7V[\x80\x82\x11\x15a$\x87W_a$\xEE\x82\x82a%\x13V[P`\x01\x01a$\xDBV[\x80\x82\x11\x15a$\x87W_a%\n\x82\x82a%.V[P`\x01\x01a$\xF7V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x05\xC6\x91\x90a$\xF7V[P\x80Ta%:\x90a+\xD1V[_\x82U\x80`\x1F\x10a%IWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x05\xC6\x91\x90[\x80\x82\x11\x15a$\x87W_\x81U`\x01\x01a%aV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xE8Wa%\xE8a%tV[`@R\x91\x90PV[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a&\nWa&\na%tV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a&=\x81a%\xA1V[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a&QW__\xFD[\x82\x82` \x83\x017_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&vW__\xFD[a\"\xF1\x83\x835` \x85\x01a%\xF0V[_` \x82\x84\x03\x12\x15a&\x95W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xABW__\xFD[a#\xFE\x84\x82\x85\x01a&gV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xC6W__\xFD[\x805a&\xE3\x81a&\xB7V[\x91\x90PV[_` \x82\x84\x03\x12\x15a&\xF8W__\xFD[\x815a\"\xF1\x81a&\xB7V[__`@\x83\x85\x03\x12\x15a'\x14W__\xFD[\x825a'\x1F\x81a&\xB7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a':W__\xFD[a'F\x85\x82\x86\x01a&gV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'aW__\xFD[\x825a'l\x81a&\xB7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a'\x97W__\xFD[a'F\x85\x825` \x84\x01a%\xF0V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a(\xC2W\x84\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x88R\x81Q\x80Q\x80\x85R` \x91\x82\x01\x91\x80\x86\x01\x91\x90`\x05\x82\x90\x1B\x87\x01\x01_[\x82\x81\x10\x15a(\xA8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x88\x83\x03\x01\x84Ra(\x93\x82\x86Qa'\xA6V[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01a(YV[P` \x9B\x8C\x01\x9B\x90\x96P\x94\x90\x94\x01\x93PPP`\x01\x01a(\x0EV[P\x90\x96\x95PPPPPPV[` \x81R_a\"\xF1` \x83\x01\x84a'\xF2V[___``\x84\x86\x03\x12\x15a(\xF2W__\xFD[\x835a(\xFD\x81a&\xB7V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x81R_a\"\xF1` \x83\x01\x84a'\xA6V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)=Wa)=a%tV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a)VW__\xFD[\x815a)ia)d\x82a)$V[a%\xA1V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a)\x8AW__\xFD[` \x85\x01[\x83\x81\x10\x15a*HW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xADW__\xFD[\x86\x01`?\x81\x01\x88\x13a)\xBDW__\xFD[` \x81\x015a)\xCEa)d\x82a)$V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8A\x83\x11\x15a)\xF1W__\xFD[`@\x84\x01[\x83\x81\x10\x15a*2W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x14W__\xFD[a*#\x8D`@\x88\x84\x01\x01a&gV[\x84RP` \x92\x83\x01\x92\x01a)\xF6V[P\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa)\x8FV[P\x95\x94PPPPPV[_____`\x80\x86\x88\x03\x12\x15a*fW__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*|W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a*\x8CW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xA2W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a*\xB3W__\xFD[` \x91\x82\x01\x96P\x94Pa*\xC7\x90\x87\x01a&\xD8V[\x92P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE2W__\xFD[a*\xEE\x88\x82\x89\x01a)GV[\x95\x98\x94\x97P\x92\x95``\x015\x93\x92PPPV[___``\x84\x86\x03\x12\x15a+\x12W__\xFD[\x835a+\x1D\x81a&\xB7V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+8W__\xFD[a+D\x86\x82\x87\x01a&gV[\x92PP`@\x84\x015a+U\x81a&\xB7V[\x80\x91PP\x92P\x92P\x92V[__` \x83\x85\x03\x12\x15a+qW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+\x97W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xADW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a+\xC1W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a+\xE5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,\x1CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a,\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[`@\x81R_a,\xCC`@\x83\x01\x85a'\xF2V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\"\xF1\x82\x84a,\xDBV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a-0W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-JW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a-^W__\xFD[\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a-xW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\x8EW__\xFD[a-\x9A\x87\x82\x88\x01a&gV[\x94PP` \x85\x015a-\xAB\x81a&\xB7V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xC6W__\xFD[a-\xD2\x87\x82\x88\x01a)GV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[_` \x82\x84\x03\x12\x15a-\xF3W__\xFD[PQ\x91\x90PV[`\x1F\x82\x11\x15a\x1D\xE3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\x1FWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a.>W_\x81U`\x01\x01a.+V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a._Wa._a%tV[a.s\x81a.m\x84Ta+\xD1V[\x84a-\xFAV[` `\x1F\x82\x11`\x01\x81\x14a.\xC4W_\x83\x15a.\x8EWP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua.>V[_\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15a/\x11W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a.\xF1V[P\x84\x82\x10\x15a/MW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xD6\x8F\xFD\x18\x0Fa\x10\xEE\xD9{:w \x9B\x98\xA4Z)\xB4\x93\x04u/\x91\xD6,\x8B\x1Di\x95\xDE\xFDdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static EXTENSIONHANDLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\tW_5`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xA1W\x80c\xBC'\xAE\xB3\x11a\0qW\x80c\xF1\xFB\xBB\xA1\x11a\0WW\x80c\xF1\xFB\xBB\xA1\x14a\x03\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xB6W\x80c\xF8\x84c\x0B\x14a\x03\xD5W__\xFD[\x80c\xBC'\xAE\xB3\x14a\x03YW\x80c\xE0\x86\xA8\"\x14a\x03xW__\xFD[\x80c\x81)\xFC\x1C\x14a\x02\xA2W\x80c\x8D\xA5\xCB[\x14a\x02\xB6W\x80c\x93\xECIq\x14a\x02\xE0W\x80c\x93\xF8xB\x14a\x03\x0CW__\xFD[\x80cO\x1E\xF2\x86\x11a\0\xDCW\x80cO\x1E\xF2\x86\x14a\x02;W\x80cR\xD1\x90-\x14a\x02NW\x80cm\xFB\xE3^\x14a\x02bW\x80cqP\x18\xA6\x14a\x02\x8EW__\xFD[\x80c\r\xB7\x9B\x93\x14a\x01\rW\x80c/\x1D}\xDC\x14a\x01\x84W\x80c6Y\xCF\xE6\x14a\x01\xBDW\x80c:\x03s\x8A\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01\x18W__\xFD[Pa\x01Za\x01'6`\x04a&\x85V[\x80Q` \x81\x83\x01\x81\x01\x80Q`g\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x8FW__\xFD[Pa\x01\xAFa\x01\x9E6`\x04a&\xE8V[`i` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01{V[4\x80\x15a\x01\xC8W__\xFD[Pa\x01\xDCa\x01\xD76`\x04a&\xE8V[a\x03\xF4V[\0[4\x80\x15a\x01\xE9W__\xFD[Pa\x01Za\x01\xF86`\x04a'\x03V[`j` \x90\x81R_\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xDCa\x02I6`\x04a'PV[a\x05\xC9V[4\x80\x15a\x02YW__\xFD[Pa\x01\xAFa\x07\x8BV[4\x80\x15a\x02mW__\xFD[Pa\x02\x81a\x02|6`\x04a&\xE8V[a\x08\\V[`@Qa\x01{\x91\x90a(\xCEV[4\x80\x15a\x02\x99W__\xFD[Pa\x01\xDCa\t\x97V[4\x80\x15a\x02\xADW__\xFD[Pa\x01\xDCa\t\xAAV[4\x80\x15a\x02\xC1W__\xFD[P`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01ZV[4\x80\x15a\x02\xEBW__\xFD[Pa\x02\xFFa\x02\xFA6`\x04a(\xE0V[a\x0BIV[`@Qa\x01{\x91\x90a)\x12V[4\x80\x15a\x03\x17W__\xFD[Pa\x01Za\x03&6`\x04a&\x85V[\x80Q` \x81\x83\x01\x81\x01\x80Q`f\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x03dW__\xFD[Pa\x01\xDCa\x03s6`\x04a*RV[a\x0C\x10V[4\x80\x15a\x03\x83W__\xFD[Pa\x01Za\x03\x926`\x04a'\x03V[a\x19FV[4\x80\x15a\x03\xA2W__\xFD[Pa\x01\xDCa\x03\xB16`\x04a+\0V[a\x19\xF3V[4\x80\x15a\x03\xC1W__\xFD[Pa\x01\xDCa\x03\xD06`\x04a&\xE8V[a\x1A\x8AV[4\x80\x15a\x03\xE0W__\xFD[Pa\x01\xDCa\x03\xEF6`\x04a+`V[a\x1B$V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x04\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\x19\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x05\xAB\x81a\x1DDV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92Ra\x05\xC6\x91\x83\x91\x90a\x1D\xB0V[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x06tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\xE9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x07{\x82a\x1DDV[a\x07\x87\x82\x82`\x01a\x1D\xB0V[PPV[_0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x087W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16_\x90\x81R`h` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\t\x8CW\x83\x82\x90_R` _ \x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\tyW\x83\x82\x90_R` _ \x01\x80Ta\x08\xEE\x90a+\xD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x1A\x90a+\xD1V[\x80\x15a\teW\x80`\x1F\x10a\t<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\teV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tHW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\xD1V[PPPP\x81R` \x01\x90`\x01\x01\x90a\x08\xA0V[PPPP\x90P\x91\x90PV[a\t\x9Fa\x1F\x80V[a\t\xA8_a\x1F\xE7V[V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xC8WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xE1WP0;\x15\x80\x15a\t\xE1WP_T`\xFF\x16`\x01\x14[a\nSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\n\xAFW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\n\xB7a ]V[`e\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x163a\x01\0\x02\x17\x90U\x80\x15a\x05\xC6W_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`h` R\x82_R`@_ \x82\x81T\x81\x10a\x0BbW_\x80\xFD[\x90_R` _ \x01\x81\x81T\x81\x10a\x0BwW_\x80\xFD[\x90_R` _ \x01_\x92P\x92PPP\x80Ta\x0B\x91\x90a+\xD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xBD\x90a+\xD1V[\x80\x15a\x0C\x08W\x80`\x1F\x10a\x0B\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x08V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`f\x86\x86`@Qa\x0C9\x92\x91\x90a,\"V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fextension name already used\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid extension address\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[\x83a\r\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Finvalid extension name\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[_\x81\x11a\r\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FmaxExecutionGas must be larger t`D\x82\x01R\x7Fhan zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_\x82Q\x11a\x0EGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FsubjectTemplates array cannot be`D\x82\x01R\x7F empty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`i` R`@\x90 T\x15a\x0E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fextension already published\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[``_[\x83Q\x81\x10\x15a\x11\x88W_\x84\x82\x81Q\x81\x10a\x0E\xD9Wa\x0E\xD9a,1V[` \x02` \x01\x01QQ\x11a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FsubjectTemplate cannot be empty\0`D\x82\x01R`d\x01a\x04\x9BV[\x80_\x03a\x0FpW\x83\x81\x81Q\x81\x10a\x0FHWa\x0FHa,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x0FaWa\x0Faa,1V[` \x02` \x01\x01Q\x91Pa\x10/V[a\x0F\xBD\x82\x85\x83\x81Q\x81\x10a\x0F\x86Wa\x0F\x86a,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x0F\x9FWa\x0F\x9Fa,1V[` \x02` \x01\x01Q\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a\x10/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FsubjectTemplates must have same `D\x82\x01R\x7Fcommand\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[_`\x01[\x85\x83\x81Q\x81\x10a\x10EWa\x10Ea,1V[` \x02` \x01\x01QQ\x81\x10\x15a\x11\x07Wa\x10\xEC\x86\x84\x81Q\x81\x10a\x10jWa\x10ja,1V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x10\x83Wa\x10\x83a,1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81R\x7F{recipient}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x80Q\x91\x01 \x7F\x8A\xAF2\xFC\xBD\x17C\x99\xCB6i\x03n\x8D\xA3\xC3W\x80\x02\xBC\xCE\x06JJ\x8F)\xF7\xB5gK\xBC\xAF\x14\x90V[\x15a\x10\xFFW\x81a\x10\xFB\x81a,^V[\x92PP[`\x01\x01a\x103V[P`\x01\x81\x11\x15a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Frecipient template can only be u`D\x82\x01R\x7Fsed once\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[P`\x01\x01a\x0E\xBDV[P_[\x81Q\x81\x10\x15a\x12CW\x81\x81\x81Q\x81\x10a\x11\xA6Wa\x11\xA6a,1V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\x12;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fcommand should be one word\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[`\x01\x01a\x11\x8BV[P`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FSend\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xA8\x86\xDD\x9B\xF7J#R\x1A{\x7F\xF5\xD4\x19\xD5\xA1\xC9\xE5>K\x8C\x03\0\x12\x96\x7F\xAB\xD9\xBEw\x82\x19\x14\x15\x80\x15a\x13\x08WP`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7FExecute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F(\xD6h\xBE\xB2\x86\xA0\x83\x9CR\x82V\x18\xAB\xB59\xCB\xE2t\x8571\xDB@\x04\xCA{\xCF\x10I\xA4\xCA\x14\x15[\x80\x15a\x13nWP`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7FInstall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Ff\x97\xC6:t\xFC\xC5\xC4d\xF3\xDC\xB6\xB3\xA4\xE4\x9C\xB1\xD1\0=\xA9\x90H\xD4Lm+\x12\xFE\0\xFF:\x14\x15[\x80\x15a\x13\xD4WP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FUninstall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xCF\x0F\xB4\xAFo\xA7\x12\x03\xBF\x8C\xA4\x9A\x15\xD70\xB9\xE9\xA08ZR\x96\x1Eu\xDE\xD3\x07\x18\xD6\x9A\x94N\x14\x15[\x80\x15a\x14:WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FExit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F$\xF7?K\t\x82\xCE\xA5@\x8Ev\x08\x8D\x18 \xDC\xFC\xE3\xFD?\xB6^\x92=\xCB\x98y\xC1\xE7\xE9\x11A\x14\x15[\x80\x15a\x14\xA0WP`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81R\x7FDKIM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fc\xCA\xED\x16\x123\x96\xDD\xFB\xCC\x1E\x89:\xD0\xC1\x8E\xC4j%H\xDB\xEBQ\x18u#h5d9\xF0\xAF\x14\x15[a\x15\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcommand cannot be a reserved nam`D\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7F{tokenAmount}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x81_\xF0\x03\xEC<\xD9\x03\xB6\xB6o\xF0\xF9[T/\xD3\xA9=-\x90i<\r\x0Ck\xEA\xA3<\xCA\xAF+\x14\x15\x80\x15a\x15\xD6WP`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81R\x7F{amount}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fo\x83g\xB1\xD9\xD8\xA7\xB8-\xD8\xC3\xFE\xA7Sb\xC0aF\xBC\xA0q\x11\x9C\xF5\x9A<\xB5\x85\x01\x07k\xF0\x14\x15[\x80\x15a\x16<WP`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81R\x7F{string}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\xB0\xDD\x9C]\xFDk\x13H\x08\x959\xC8\xCD\x81F\xA5\x9F\x1F\xD2?\xF2\xDE\x9C`R\xE5M\xA8\xD2\xA6\xC0\xFB\x14\x15[\x80\x15a\x16\xA2WP`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81R\x7F{uint}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7Fo^\xA6\xF4\x05\xF6a\xD5\x06k\x9E\x0F\xF0z%\xFD.\r `W\xA7\xFC-\xFE\xF3?\xF6Z\xD2*#\x14\x15[\x80\x15a\x17\x08WP`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F{int}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x96\x11\\R(\x17\x05\0M\xB3\xCA\x7F`A\x12\xB6\xBCv\xAEP^\xD2h\xB2\xDB\xEE)\xFE\xB8\xE7\x89\x9D\x14\x15[\x80\x15a\x17nWP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7F{address}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x9A=\xE9\xFC\xA4o$ \x94\x7F-\x1DGD\xC5\xFE\xBE\xDE\xBCw\xB9\xA3\xFC\xBF\xCF\x16\xED\x02\xCB\xE4\x8Fs\x14\x15[\x80\x15a\x17\xD4WP`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81R\x7F{recipient}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x90\x82\x01 \x7F\x8A\xAF2\xFC\xBD\x17C\x99\xCB6i\x03n\x8D\xA3\xC3W\x80\x02\xBC\xCE\x06JJ\x8F)\xF7\xB5gK\xBC\xAF\x14\x15[a\x18EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcommand cannot be a template mat`D\x82\x01R\x7Fcher\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x83`f\x87\x87`@Qa\x18X\x92\x91\x90a,\"V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U\x91\x86\x16_\x90\x81R`h\x82R\x91\x90\x91 \x84Qa\x18\xC5\x92\x86\x01\x90a$0V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x81\x81R`i` R`@\x90\x81\x90 \x84\x90UQa\x18\xFD\x90\x88\x90\x88\x90a,\"V[`@Q\x80\x91\x03\x90 \x7F\xEF\x0E\x97\x1FC\xD94~\xAD|\xB6g\x91\xFD\xCC\xE2\xD7\xA5\x17\x1E\xE1\xF6R\xFE\xC7'\x82=\x1E-N\xE8\x85\x85`@Qa\x196\x92\x91\x90a,\xBAV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`j` R`@\x80\x82 \x90Q\x82\x91\x82\x91a\x19|\x90\x86\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x15a\x19\xB2W\x80\x91Pa\x19\xEBV[`g\x84`@Qa\x19\xC2\x91\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P[P\x93\x92PPPV[a\x19\xFBa\x1F\x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`j` R`@\x90\x81\x90 \x90Q\x82\x91\x90a\x1A1\x90\x85\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x1A\x92a\x1F\x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\x05\xC6\x81a\x1F\xE7V[a\x1B,a\x1F\x80V[`eT`\xFF\x16\x15a\x1B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fdefault extensions already set\0\0`D\x82\x01R`d\x01a\x04\x9BV[_[\x81\x81\x10\x15a\x1D\x14W____\x86\x86\x86\x81\x81\x10a\x1B\x9FWa\x1B\x9Fa,1V[\x90P` \x02\x81\x01\x90a\x1B\xB1\x91\x90a,\xFDV[\x81\x01\x90a\x1B\xBE\x91\x90a-eV[\x93P\x93P\x93P\x93P\x82`f\x85`@Qa\x1B\xD7\x91\x90a,\xF2V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x17\x90U\x91\x85\x16_\x90\x81R`h\x82R\x91\x90\x91 \x83Qa\x1CD\x92\x85\x01\x90a$0V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`i` R`@\x81 \x82\x90U\x82Q\x84\x91`g\x91\x85\x91\x90a\x1C\x82Wa\x1C\x82a,1V[` \x02` \x01\x01Q_\x81Q\x81\x10a\x1C\x9BWa\x1C\x9Ba,1V[` \x02` \x01\x01Q`@Qa\x1C\xB0\x91\x90a,\xF2V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90UPPP`\x01\x91\x90\x91\x01\x90Pa\x1B\x81V[PP`e\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UPV[`eTa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fcaller is not a deployer\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x1D\xE8Wa\x1D\xE3\x83a \xE1V[PPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1EmWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x1Ej\x91\x81\x01\x90a-\xE3V[`\x01[a\x1E\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01R\x7Fon is not UUPS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x1FtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01R\x7FiableUUID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[Pa\x1D\xE3\x83\x83\x83a!\xD1V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\x9BV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a \xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\t\xA8a!\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a!kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a!\xDA\x83a\"\x80V[_\x82Q\x11\x80a!\xE6WP\x80[\x15a\x1D\xE3Wa!\xF5\x83\x83a\"\xCCV[PPPPV[_Ta\x01\0\x90\x04`\xFF\x16a\"wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x9BV[a\t\xA83a\x1F\xE7V[a\"\x89\x81a \xE1V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\"\xF1\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a/]`'\x919a\"\xF8V[\x93\x92PPPV[``__\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85`@Qa#!\x91\x90a,\xF2V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a#YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a#^V[``\x91P[P\x91P\x91Pa#o\x86\x83\x83\x87a#yV[\x96\x95PPPPPPV[``\x83\x15a#\xF4W\x82Q_\x03a#\xEDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x04\x9BV[P\x81a#\xFEV[a#\xFE\x83\x83a$\x06V[\x94\x93PPPPV[\x81Q\x15a$\x16W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x9B\x91\x90a)\x12V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a${W\x91` \x02\x82\x01[\x82\x81\x11\x15a${W\x82Q\x80Qa$k\x91\x84\x91` \x90\x91\x01\x90a$\x8BV[P\x91` \x01\x91\x90`\x01\x01\x90a$NV[Pa$\x87\x92\x91Pa$\xDBV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a$\xCFW\x91` \x02\x82\x01[\x82\x81\x11\x15a$\xCFW\x82Q\x82\x90a$\xBF\x90\x82a.EV[P\x91` \x01\x91\x90`\x01\x01\x90a$\xA9V[Pa$\x87\x92\x91Pa$\xF7V[\x80\x82\x11\x15a$\x87W_a$\xEE\x82\x82a%\x13V[P`\x01\x01a$\xDBV[\x80\x82\x11\x15a$\x87W_a%\n\x82\x82a%.V[P`\x01\x01a$\xF7V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x05\xC6\x91\x90a$\xF7V[P\x80Ta%:\x90a+\xD1V[_\x82U\x80`\x1F\x10a%IWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x05\xC6\x91\x90[\x80\x82\x11\x15a$\x87W_\x81U`\x01\x01a%aV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xE8Wa%\xE8a%tV[`@R\x91\x90PV[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a&\nWa&\na%tV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a&=\x81a%\xA1V[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a&QW__\xFD[\x82\x82` \x83\x017_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&vW__\xFD[a\"\xF1\x83\x835` \x85\x01a%\xF0V[_` \x82\x84\x03\x12\x15a&\x95W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xABW__\xFD[a#\xFE\x84\x82\x85\x01a&gV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xC6W__\xFD[\x805a&\xE3\x81a&\xB7V[\x91\x90PV[_` \x82\x84\x03\x12\x15a&\xF8W__\xFD[\x815a\"\xF1\x81a&\xB7V[__`@\x83\x85\x03\x12\x15a'\x14W__\xFD[\x825a'\x1F\x81a&\xB7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a':W__\xFD[a'F\x85\x82\x86\x01a&gV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'aW__\xFD[\x825a'l\x81a&\xB7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a'\x97W__\xFD[a'F\x85\x825` \x84\x01a%\xF0V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a(\xC2W\x84\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x88R\x81Q\x80Q\x80\x85R` \x91\x82\x01\x91\x80\x86\x01\x91\x90`\x05\x82\x90\x1B\x87\x01\x01_[\x82\x81\x10\x15a(\xA8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x88\x83\x03\x01\x84Ra(\x93\x82\x86Qa'\xA6V[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01a(YV[P` \x9B\x8C\x01\x9B\x90\x96P\x94\x90\x94\x01\x93PPP`\x01\x01a(\x0EV[P\x90\x96\x95PPPPPPV[` \x81R_a\"\xF1` \x83\x01\x84a'\xF2V[___``\x84\x86\x03\x12\x15a(\xF2W__\xFD[\x835a(\xFD\x81a&\xB7V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x81R_a\"\xF1` \x83\x01\x84a'\xA6V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)=Wa)=a%tV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a)VW__\xFD[\x815a)ia)d\x82a)$V[a%\xA1V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a)\x8AW__\xFD[` \x85\x01[\x83\x81\x10\x15a*HW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xADW__\xFD[\x86\x01`?\x81\x01\x88\x13a)\xBDW__\xFD[` \x81\x015a)\xCEa)d\x82a)$V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8A\x83\x11\x15a)\xF1W__\xFD[`@\x84\x01[\x83\x81\x10\x15a*2W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x14W__\xFD[a*#\x8D`@\x88\x84\x01\x01a&gV[\x84RP` \x92\x83\x01\x92\x01a)\xF6V[P\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa)\x8FV[P\x95\x94PPPPPV[_____`\x80\x86\x88\x03\x12\x15a*fW__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*|W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a*\x8CW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xA2W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a*\xB3W__\xFD[` \x91\x82\x01\x96P\x94Pa*\xC7\x90\x87\x01a&\xD8V[\x92P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE2W__\xFD[a*\xEE\x88\x82\x89\x01a)GV[\x95\x98\x94\x97P\x92\x95``\x015\x93\x92PPPV[___``\x84\x86\x03\x12\x15a+\x12W__\xFD[\x835a+\x1D\x81a&\xB7V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+8W__\xFD[a+D\x86\x82\x87\x01a&gV[\x92PP`@\x84\x015a+U\x81a&\xB7V[\x80\x91PP\x92P\x92P\x92V[__` \x83\x85\x03\x12\x15a+qW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+\x97W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xADW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a+\xC1W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a+\xE5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,\x1CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a,\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[`@\x81R_a,\xCC`@\x83\x01\x85a'\xF2V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\"\xF1\x82\x84a,\xDBV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a-0W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-JW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a-^W__\xFD[\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a-xW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\x8EW__\xFD[a-\x9A\x87\x82\x88\x01a&gV[\x94PP` \x85\x015a-\xAB\x81a&\xB7V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xC6W__\xFD[a-\xD2\x87\x82\x88\x01a)GV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[_` \x82\x84\x03\x12\x15a-\xF3W__\xFD[PQ\x91\x90PV[`\x1F\x82\x11\x15a\x1D\xE3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\x1FWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a.>W_\x81U`\x01\x01a.+V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a._Wa._a%tV[a.s\x81a.m\x84Ta+\xD1V[\x84a-\xFAV[` `\x1F\x82\x11`\x01\x81\x14a.\xC4W_\x83\x15a.\x8EWP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua.>V[_\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15a/\x11W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a.\xF1V[P\x84\x82\x10\x15a/MW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xD6\x8F\xFD\x18\x0Fa\x10\xEE\xD9{:w \x9B\x98\xA4Z)\xB4\x93\x04u/\x91\xD6,\x8B\x1Di\x95\xDE\xFDdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static EXTENSIONHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExtensionHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExtensionHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExtensionHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExtensionHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExtensionHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExtensionHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExtensionHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXTENSIONHANDLER_ABI.clone(),
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
                EXTENSIONHANDLER_ABI.clone(),
                EXTENSIONHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addressOfExtensionName` (0x93f87842) function
        pub fn address_of_extension_name(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([147, 248, 120, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultExtensionOfCommand` (0x0db79b93) function
        pub fn default_extension_of_command(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 183, 155, 147], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExtensionForCommand` (0xe086a822) function
        pub fn get_extension_for_command(
            &self,
            wallet_addr: ::ethers::core::types::Address,
            command: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([224, 134, 168, 34], (wallet_addr, command))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubjectTemplatesOfExtension` (0x6dfbe35e) function
        pub fn get_subject_templates_of_extension(
            &self,
            extension_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        > {
            self.0
                .method_hash([109, 251, 227, 94], extension_addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxGasOfExtension` (0x2f1d7ddc) function
        pub fn max_gas_of_extension(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 29, 125, 220], p0)
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
        ///Calls the contract's `publishExtension` (0xbc27aeb3) function
        pub fn publish_extension(
            &self,
            name: ::std::string::String,
            addr: ::ethers::core::types::Address,
            subject_templates: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
            max_execution_gas: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 39, 174, 179],
                    (name, addr, subject_templates, max_execution_gas),
                )
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
        ///Calls the contract's `setDefaultExtensions` (0xf884630b) function
        pub fn set_default_extensions(
            &self,
            default_extensions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 132, 99, 11], default_extensions)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setExtensionForCommand` (0xf1fbbba1) function
        pub fn set_extension_for_command(
            &self,
            wallet_addr: ::ethers::core::types::Address,
            command: ::std::string::String,
            extension_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [241, 251, 187, 161],
                    (wallet_addr, command, extension_addr),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subjectTemplatesOfExtension` (0x93ec4971) function
        pub fn subject_templates_of_extension(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([147, 236, 73, 113], (p0, p1, p2))
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
        ///Calls the contract's `userExtensionOfCommand` (0x3a03738a) function
        pub fn user_extension_of_command(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 3, 115, 138], (p0, p1))
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
        ///Gets the contract's `ExtensionPublished` event
        pub fn extension_published_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExtensionPublishedFilter,
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
            ExtensionHandlerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExtensionHandler<M> {
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
    #[ethevent(
        name = "ExtensionPublished",
        abi = "ExtensionPublished(string,address,string[][],uint256)"
    )]
    pub struct ExtensionPublishedFilter {
        #[ethevent(indexed)]
        pub name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub extension_addr: ::ethers::core::types::Address,
        pub subject_templates: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        pub max_execution_gas: ::ethers::core::types::U256,
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
    pub enum ExtensionHandlerEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        ExtensionPublishedFilter(ExtensionPublishedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ExtensionHandlerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = ExtensionPublishedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::ExtensionPublishedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ExtensionHandlerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExtensionPublishedFilter(element) => {
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
    impl ::core::convert::From<AdminChangedFilter> for ExtensionHandlerEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for ExtensionHandlerEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<ExtensionPublishedFilter> for ExtensionHandlerEvents {
        fn from(value: ExtensionPublishedFilter) -> Self {
            Self::ExtensionPublishedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for ExtensionHandlerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ExtensionHandlerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for ExtensionHandlerEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addressOfExtensionName` function with signature `addressOfExtensionName(string)` and selector `0x93f87842`
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
    #[ethcall(name = "addressOfExtensionName", abi = "addressOfExtensionName(string)")]
    pub struct AddressOfExtensionNameCall(pub ::std::string::String);
    ///Container type for all input parameters for the `defaultExtensionOfCommand` function with signature `defaultExtensionOfCommand(string)` and selector `0x0db79b93`
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
        name = "defaultExtensionOfCommand",
        abi = "defaultExtensionOfCommand(string)"
    )]
    pub struct DefaultExtensionOfCommandCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getExtensionForCommand` function with signature `getExtensionForCommand(address,string)` and selector `0xe086a822`
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
        name = "getExtensionForCommand",
        abi = "getExtensionForCommand(address,string)"
    )]
    pub struct GetExtensionForCommandCall {
        pub wallet_addr: ::ethers::core::types::Address,
        pub command: ::std::string::String,
    }
    ///Container type for all input parameters for the `getSubjectTemplatesOfExtension` function with signature `getSubjectTemplatesOfExtension(address)` and selector `0x6dfbe35e`
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
        name = "getSubjectTemplatesOfExtension",
        abi = "getSubjectTemplatesOfExtension(address)"
    )]
    pub struct GetSubjectTemplatesOfExtensionCall {
        pub extension_addr: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `maxGasOfExtension` function with signature `maxGasOfExtension(address)` and selector `0x2f1d7ddc`
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
    #[ethcall(name = "maxGasOfExtension", abi = "maxGasOfExtension(address)")]
    pub struct MaxGasOfExtensionCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `publishExtension` function with signature `publishExtension(string,address,string[][],uint256)` and selector `0xbc27aeb3`
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
        name = "publishExtension",
        abi = "publishExtension(string,address,string[][],uint256)"
    )]
    pub struct PublishExtensionCall {
        pub name: ::std::string::String,
        pub addr: ::ethers::core::types::Address,
        pub subject_templates: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        pub max_execution_gas: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setDefaultExtensions` function with signature `setDefaultExtensions(bytes[])` and selector `0xf884630b`
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
    #[ethcall(name = "setDefaultExtensions", abi = "setDefaultExtensions(bytes[])")]
    pub struct SetDefaultExtensionsCall {
        pub default_extensions: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `setExtensionForCommand` function with signature `setExtensionForCommand(address,string,address)` and selector `0xf1fbbba1`
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
        name = "setExtensionForCommand",
        abi = "setExtensionForCommand(address,string,address)"
    )]
    pub struct SetExtensionForCommandCall {
        pub wallet_addr: ::ethers::core::types::Address,
        pub command: ::std::string::String,
        pub extension_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `subjectTemplatesOfExtension` function with signature `subjectTemplatesOfExtension(address,uint256,uint256)` and selector `0x93ec4971`
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
        name = "subjectTemplatesOfExtension",
        abi = "subjectTemplatesOfExtension(address,uint256,uint256)"
    )]
    pub struct SubjectTemplatesOfExtensionCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `userExtensionOfCommand` function with signature `userExtensionOfCommand(address,string)` and selector `0x3a03738a`
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
        name = "userExtensionOfCommand",
        abi = "userExtensionOfCommand(address,string)"
    )]
    pub struct UserExtensionOfCommandCall(
        pub ::ethers::core::types::Address,
        pub ::std::string::String,
    );
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExtensionHandlerCalls {
        AddressOfExtensionName(AddressOfExtensionNameCall),
        DefaultExtensionOfCommand(DefaultExtensionOfCommandCall),
        GetExtensionForCommand(GetExtensionForCommandCall),
        GetSubjectTemplatesOfExtension(GetSubjectTemplatesOfExtensionCall),
        Initialize(InitializeCall),
        MaxGasOfExtension(MaxGasOfExtensionCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        PublishExtension(PublishExtensionCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDefaultExtensions(SetDefaultExtensionsCall),
        SetExtensionForCommand(SetExtensionForCommandCall),
        SubjectTemplatesOfExtension(SubjectTemplatesOfExtensionCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        UserExtensionOfCommand(UserExtensionOfCommandCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExtensionHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddressOfExtensionNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressOfExtensionName(decoded));
            }
            if let Ok(decoded) = <DefaultExtensionOfCommandCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultExtensionOfCommand(decoded));
            }
            if let Ok(decoded) = <GetExtensionForCommandCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExtensionForCommand(decoded));
            }
            if let Ok(decoded) = <GetSubjectTemplatesOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubjectTemplatesOfExtension(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MaxGasOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxGasOfExtension(decoded));
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
            if let Ok(decoded) = <PublishExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PublishExtension(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetDefaultExtensionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDefaultExtensions(decoded));
            }
            if let Ok(decoded) = <SetExtensionForCommandCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetExtensionForCommand(decoded));
            }
            if let Ok(decoded) = <SubjectTemplatesOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubjectTemplatesOfExtension(decoded));
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
            if let Ok(decoded) = <UserExtensionOfCommandCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserExtensionOfCommand(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExtensionHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressOfExtensionName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultExtensionOfCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetExtensionForCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubjectTemplatesOfExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxGasOfExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PublishExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDefaultExtensions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetExtensionForCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubjectTemplatesOfExtension(element) => {
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
                Self::UserExtensionOfCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExtensionHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressOfExtensionName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultExtensionOfCommand(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetExtensionForCommand(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubjectTemplatesOfExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxGasOfExtension(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublishExtension(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultExtensions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetExtensionForCommand(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubjectTemplatesOfExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserExtensionOfCommand(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressOfExtensionNameCall> for ExtensionHandlerCalls {
        fn from(value: AddressOfExtensionNameCall) -> Self {
            Self::AddressOfExtensionName(value)
        }
    }
    impl ::core::convert::From<DefaultExtensionOfCommandCall> for ExtensionHandlerCalls {
        fn from(value: DefaultExtensionOfCommandCall) -> Self {
            Self::DefaultExtensionOfCommand(value)
        }
    }
    impl ::core::convert::From<GetExtensionForCommandCall> for ExtensionHandlerCalls {
        fn from(value: GetExtensionForCommandCall) -> Self {
            Self::GetExtensionForCommand(value)
        }
    }
    impl ::core::convert::From<GetSubjectTemplatesOfExtensionCall>
    for ExtensionHandlerCalls {
        fn from(value: GetSubjectTemplatesOfExtensionCall) -> Self {
            Self::GetSubjectTemplatesOfExtension(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ExtensionHandlerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MaxGasOfExtensionCall> for ExtensionHandlerCalls {
        fn from(value: MaxGasOfExtensionCall) -> Self {
            Self::MaxGasOfExtension(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ExtensionHandlerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for ExtensionHandlerCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<PublishExtensionCall> for ExtensionHandlerCalls {
        fn from(value: PublishExtensionCall) -> Self {
            Self::PublishExtension(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ExtensionHandlerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDefaultExtensionsCall> for ExtensionHandlerCalls {
        fn from(value: SetDefaultExtensionsCall) -> Self {
            Self::SetDefaultExtensions(value)
        }
    }
    impl ::core::convert::From<SetExtensionForCommandCall> for ExtensionHandlerCalls {
        fn from(value: SetExtensionForCommandCall) -> Self {
            Self::SetExtensionForCommand(value)
        }
    }
    impl ::core::convert::From<SubjectTemplatesOfExtensionCall>
    for ExtensionHandlerCalls {
        fn from(value: SubjectTemplatesOfExtensionCall) -> Self {
            Self::SubjectTemplatesOfExtension(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ExtensionHandlerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for ExtensionHandlerCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for ExtensionHandlerCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<UserExtensionOfCommandCall> for ExtensionHandlerCalls {
        fn from(value: UserExtensionOfCommandCall) -> Self {
            Self::UserExtensionOfCommand(value)
        }
    }
    ///Container type for all return fields from the `addressOfExtensionName` function with signature `addressOfExtensionName(string)` and selector `0x93f87842`
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
    pub struct AddressOfExtensionNameReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultExtensionOfCommand` function with signature `defaultExtensionOfCommand(string)` and selector `0x0db79b93`
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
    pub struct DefaultExtensionOfCommandReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getExtensionForCommand` function with signature `getExtensionForCommand(address,string)` and selector `0xe086a822`
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
    pub struct GetExtensionForCommandReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubjectTemplatesOfExtension` function with signature `getSubjectTemplatesOfExtension(address)` and selector `0x6dfbe35e`
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
    pub struct GetSubjectTemplatesOfExtensionReturn(
        pub ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    );
    ///Container type for all return fields from the `maxGasOfExtension` function with signature `maxGasOfExtension(address)` and selector `0x2f1d7ddc`
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
    pub struct MaxGasOfExtensionReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `subjectTemplatesOfExtension` function with signature `subjectTemplatesOfExtension(address,uint256,uint256)` and selector `0x93ec4971`
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
    pub struct SubjectTemplatesOfExtensionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `userExtensionOfCommand` function with signature `userExtensionOfCommand(address,string)` and selector `0x3a03738a`
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
    pub struct UserExtensionOfCommandReturn(pub ::ethers::core::types::Address);
}
