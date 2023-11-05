pub use extension_handler::*;
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
pub mod extension_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressOfExtensionName"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addressOfExtensionName",),
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
                (
                    ::std::borrow::ToOwned::to_owned("defaultExtensionOfCommand"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultExtensionOfCommand",),
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
                (
                    ::std::borrow::ToOwned::to_owned("getExtensionForCommand"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getExtensionForCommand",),
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
                    ::std::borrow::ToOwned::to_owned("getSubjectTemplatesOfExtension"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubjectTemplatesOfExtension",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("extensionAddr"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
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
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxGasOfExtension"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("maxGasOfExtension"),
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
                    ::std::borrow::ToOwned::to_owned("publishExtension"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("setDefaultExtensions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDefaultExtensions",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("defaultExtensions"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setExtensionForCommand"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setExtensionForCommand",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("subjectTemplatesOfExtension"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("subjectTemplatesOfExtension",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("userExtensionOfCommand"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("userExtensionOfCommand",),
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
                    ::std::borrow::ToOwned::to_owned("ExtensionPublished"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXTENSIONHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x1B\x19\x80a\0~`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xAFW`\x005`\xE0\x1C\x80c\r\xB7\x9B\x93\x14a\0\xB4W\x80c/\x1D}\xDC\x14a\x01\x05W\x80c:\x03s\x8A\x14a\x013W\x80cm\xFB\xE3^\x14a\x01xW\x80cqP\x18\xA6\x14a\x01\x98W\x80c\x8D\xA5\xCB[\x14a\x01\xA2W\x80c\x93\xECIq\x14a\x01\xAAW\x80c\x93\xF8xB\x14a\x01\xCAW\x80c\xBC'\xAE\xB3\x14a\x01\xFEW\x80c\xE0\x86\xA8\"\x14a\x02\x11W\x80c\xF1\xFB\xBB\xA1\x14a\x02$W\x80c\xF2\xFD\xE3\x8B\x14a\x027W\x80c\xF8\x84c\x0B\x14a\x02JW[`\0\x80\xFD[a\0\xE8a\0\xC26`\x04a\x13\x82V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01%a\x01\x136`\x04a\x13\xE3V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xFCV[a\0\xE8a\x01A6`\x04a\x14\x07V[`\x05` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Ba\x01\x866`\x04a\x13\xE3V[a\x02]V[`@Qa\0\xFC\x91\x90a\x15>V[a\x01\xA0a\x03\x93V[\0[a\0\xE8a\x03\xA7V[a\x01\xBDa\x01\xB86`\x04a\x15QV[a\x03\xB6V[`@Qa\0\xFC\x91\x90a\x15\x86V[a\0\xE8a\x01\xD86`\x04a\x13\x82V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x01\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xA0a\x02\x0C6`\x04a\x16\xBEV[a\x04\x88V[a\0\xE8a\x02\x1F6`\x04a\x14\x07V[a\r\x97V[a\x01\xA0a\x0226`\x04a\x17mV[a\x0E\x10V[a\x01\xA0a\x02E6`\x04a\x13\xE3V[a\x0EvV[a\x01\xA0a\x02X6`\x04a\x17\xD0V[a\x0E\xEFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x03\x88W\x83\x82\x90`\0R` `\0 \x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03uW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x02\xE8\x90a\x18DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x14\x90a\x18DV[\x80\x15a\x03aW\x80`\x1F\x10a\x036Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03aV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x02\xC9V[PPPP\x81R` \x01\x90`\x01\x01\x90a\x02\x95V[PPPP\x90P\x91\x90PV[a\x03\x9Ba\x10\xB7V[a\x03\xA5`\0a\x11\x16V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x03` R\x82`\0R`@`\0 \x82\x81T\x81\x10a\x03\xD2W`\0\x80\xFD[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x03\xEAW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x92P\x92PPP\x80Ta\x04\x07\x90a\x18DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x043\x90a\x18DV[\x80\x15a\x04\x80W\x80`\x1F\x10a\x04UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x01\x86\x86`@Qa\x04\xA5\x92\x91\x90a\x18~V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x19^\x1D\x19[\x9C\xDA[\xDB\x88\x1B\x98[YH\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`*\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rxinvalid extension address`8\x1B`D\x82\x01R`d\x01a\x05\x05V[\x83a\x05\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ruinvalid extension name`P\x1B`D\x82\x01R`d\x01a\x05\x05V[`\0\x81\x11a\x06\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FmaxExecutionGas must be larger t`D\x82\x01Rghan zero`\xC0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0\x82Q\x11a\x06gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FsubjectTemplates array cannot be`D\x82\x01Re empty`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x06\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x19^\x1D\x19[\x9C\xDA[\xDB\x88\x18[\x1C\x99XY\x1EH\x1C\x1DX\x9B\x1A\\\xDA\x19Y`*\x1B`D\x82\x01R`d\x01a\x05\x05V[```\0[\x83Q\x81\x10\x15a\t@W`\0\x84\x82\x81Q\x81\x10a\x06\xEDWa\x06\xEDa\x18\x8EV[` \x02` \x01\x01QQ\x11a\x07CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FsubjectTemplate cannot be empty\0`D\x82\x01R`d\x01a\x05\x05V[\x80`\0\x03a\x07\x86W\x83\x81\x81Q\x81\x10a\x07]Wa\x07]a\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x07wWa\x07wa\x18\x8EV[` \x02` \x01\x01Q\x91Pa\x08\x1FV[a\x07\xC3\x82\x85\x83\x81Q\x81\x10a\x07\x9CWa\x07\x9Ca\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x07\xB6Wa\x07\xB6a\x18\x8EV[` \x02` \x01\x01Qa\x11fV[a\x08\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FsubjectTemplates must have same `D\x82\x01Rf\x18\xDB\xDB[X[\x99`\xCA\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0`\x01[\x85\x83\x81Q\x81\x10a\x086Wa\x086a\x18\x8EV[` \x02` \x01\x01QQ\x81\x10\x15a\x08\xCAWa\x08\xA5\x86\x84\x81Q\x81\x10a\x08[Wa\x08[a\x18\x8EV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x08tWa\x08ta\x18\x8EV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j{recipient}`\xA8\x1B\x81RPa\x11fV[\x15a\x08\xB8W\x81a\x08\xB4\x81a\x18\xA4V[\x92PP[\x80a\x08\xC2\x81a\x18\xA4V[\x91PPa\x08$V[P`\x01\x81\x11\x15a\t-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Frecipient template can only be u`D\x82\x01Rgsed once`\xC0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[P\x80a\t8\x81a\x18\xA4V[\x91PPa\x06\xD0V[P`\0[\x81Q\x81\x10\x15a\t\xCFW\x81\x81\x81Q\x81\x10a\t_Wa\t_a\x18\x8EV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\x01`\xFD\x1B\x03a\t\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x18\xDB\xDB[X[\x99\x08\x1C\xDA\x1B\xDD[\x19\x08\x18\x99H\x1B\xDB\x99H\x1D\xDB\xDC\x99`2\x1B`D\x82\x01R`d\x01a\x05\x05V[\x80a\t\xC7\x81a\x18\xA4V[\x91PPa\tDV[Pa\t\xF6\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x14\xD9[\x99`\xE2\x1B\x81RPa\x11fV[\x15\x80\x15a\n)WPa\n'\x81`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fExecute`\xC8\x1B\x81RPa\x11fV[\x15[\x80\x15a\n[WPa\nY\x81`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x12[\x9C\xDD\x18[\x1B`\xCA\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\x8FWPa\n\x8D\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x15[\x9A[\x9C\xDD\x18[\x1B`\xBA\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\xBEWPa\n\xBC\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x11^\x1A]`\xE2\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\xEDWPa\n\xEB\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cDKIM`\xE0\x1B\x81RPa\x11fV[\x15[a\x0BCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcommand cannot be a reserved nam`D\x82\x01R`e`\xF8\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x0Br\x81`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l{tokenAmount}`\x98\x1B\x81RPa\x11fV[\x15\x80\x15a\x0B\xA6WPa\x0B\xA4\x81`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g{amount}`\xC0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0B\xD9WPa\x0B\xD7\x81`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g{string}`\xC0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0C\nWPa\x0C\x08\x81`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e{uint}`\xD0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0C:WPa\x0C8\x81`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d{int}`\xD8\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0CnWPa\x0Cl\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h{address}`\xB8\x1B\x81RPa\x11fV[\x15[a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcommand cannot be a template mat`D\x82\x01Rc1\xB42\xB9`\xE1\x1B`d\x82\x01R`\x84\x01a\x05\x05V[\x83`\x01\x87\x87`@Qa\x0C\xD9\x92\x91\x90a\x18~V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x86\x16`\0\x90\x81R`\x03\x82R\x91\x90\x91 \x84Qa\r\"\x92\x86\x01\x90a\x11|V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x84\x90UQa\rN\x90\x88\x90\x88\x90a\x18~V[`@Q\x80\x91\x03\x90 \x7F\xEF\x0E\x97\x1FC\xD94~\xAD|\xB6g\x91\xFD\xCC\xE2\xD7\xA5\x17\x1E\xE1\xF6R\xFE\xC7'\x82=\x1E-N\xE8\x85\x85`@Qa\r\x87\x92\x91\x90a\x18\xCBV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[`\0\x80`\x02\x83`@Qa\r\xAA\x91\x90a\x18\xEDV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 T`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\0\x90\x81R`\x05\x90\x95R\x92\x84 \x92\x16\x93Pa\r\xE3\x90\x86\x90a\x18\xEDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80\x15a\x0E\x08W\x80\x91P[P\x93\x92PPPV[a\x0E\x18a\x10\xB7V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x05` R`@\x90\x81\x90 \x90Q\x82\x91\x90a\x0EB\x90\x85\x90a\x18\xEDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x0E~a\x10\xB7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x0E\xEC\x81a\x11\x16V[PV[a\x0E\xF7a\x10\xB7V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0FQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fdefault extensions already set\0\0`D\x82\x01R`d\x01a\x05\x05V[`\0[\x81\x81\x10\x15a\x10\x9FW`\0\x80`\0\x80\x86\x86\x86\x81\x81\x10a\x0FtWa\x0Fta\x18\x8EV[\x90P` \x02\x81\x01\x90a\x0F\x86\x91\x90a\x19\tV[\x81\x01\x90a\x0F\x93\x91\x90a\x19VV[\x93P\x93P\x93P\x93P\x82`\x01\x85`@Qa\x0F\xAC\x91\x90a\x18\xEDV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x85\x16`\0\x90\x81R`\x03\x82R\x91\x90\x91 \x83Qa\x0F\xF5\x92\x85\x01\x90a\x11|V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x82\x90U\x82Q\x84\x91`\x02\x91\x85\x91\x90a\x10'Wa\x10'a\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x10AWa\x10Aa\x18\x8EV[` \x02` \x01\x01Q`@Qa\x10V\x91\x90a\x18\xEDV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPP\x80\x80a\x10\x97\x90a\x18\xA4V[\x91PPa\x0FTV[PP`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPV[3a\x10\xC0a\x03\xA7V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x05V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x11\xC9W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x11\xC9W\x82Q\x80Qa\x11\xB9\x91\x84\x91` \x90\x91\x01\x90a\x11\xD9V[P\x91` \x01\x91\x90`\x01\x01\x90a\x11\x9CV[Pa\x11\xD5\x92\x91Pa\x12+V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\x1FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\x1FW\x82Q\x82\x90a\x12\x0F\x90\x82a\x1A$V[P\x91` \x01\x91\x90`\x01\x01\x90a\x11\xF9V[Pa\x11\xD5\x92\x91Pa\x12HV[\x80\x82\x11\x15a\x11\xD5W`\0a\x12?\x82\x82a\x12eV[P`\x01\x01a\x12+V[\x80\x82\x11\x15a\x11\xD5W`\0a\x12\\\x82\x82a\x12\x83V[P`\x01\x01a\x12HV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0E\xEC\x91\x90a\x12HV[P\x80Ta\x12\x8F\x90a\x18DV[`\0\x82U\x80`\x1F\x10a\x12\x9FWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0E\xEC\x91\x90[\x80\x82\x11\x15a\x11\xD5W`\0\x81U`\x01\x01a\x12\xB9V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x0BWa\x13\x0Ba\x12\xCDV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13=Wa\x13=a\x12\xCDV[a\x13P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\xE3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x13eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\x94W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xAAW`\0\x80\xFD[a\x13\xB6\x84\x82\x85\x01a\x13\x13V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xECW`\0\x80\xFD[\x805a\x13\xDE\x81a\x13\xBEV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\xF5W`\0\x80\xFD[\x815a\x14\0\x81a\x13\xBEV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x1AW`\0\x80\xFD[\x825a\x14%\x81a\x13\xBEV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14@W`\0\x80\xFD[a\x14L\x85\x82\x86\x01a\x13\x13V[\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x14qW\x81\x81\x01Q\x83\x82\x01R` \x01a\x14YV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\x92\x81` \x86\x01` \x86\x01a\x14VV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x15/W\x84\x84\x03\x8BR\x82Q\x80Q\x80\x86R\x90\x88\x01\x90\x88\x86\x01\x90\x80\x89\x1B\x87\x01\x8A\x01\x85[\x82\x81\x10\x15a\x15\x19W`\x1F\x19\x89\x83\x03\x01\x84Ra\x15\x07\x82\x86Qa\x14zV[\x94\x8C\x01\x94\x93\x8C\x01\x93\x91P`\x01\x01a\x14\xEBV[P\x9D\x8A\x01\x9D\x96PPP\x92\x87\x01\x92P`\x01\x01a\x14\xC6V[P\x91\x99\x98PPPPPPPPPV[` \x81R`\0a\x14\0` \x83\x01\x84a\x14\xA6V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15fW`\0\x80\xFD[\x835a\x15q\x81a\x13\xBEV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x81R`\0a\x14\0` \x83\x01\x84a\x14zV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\xB2Wa\x15\xB2a\x12\xCDV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x15\xCDW`\0\x80\xFD[\x815` a\x15\xE2a\x15\xDD\x83a\x15\x99V[a\x12\xE3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x16\x01W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16\xB3W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16$W`\0\x80\xFD[\x81\x89\x01\x91P\x89`?\x83\x01\x12a\x168W`\0\x80\xFD[\x85\x82\x015a\x16Ha\x15\xDD\x82a\x15\x99V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x87\x81\x01\x90\x8C\x83\x11\x15a\x16hW`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\x16\xA1W\x805\x85\x81\x11\x15a\x16\x84W`\0\x80\xFD[a\x16\x93\x8F`@\x83\x8A\x01\x01a\x13\x13V[\x84RP\x91\x89\x01\x91\x89\x01a\x16mV[P\x87RPPP\x92\x84\x01\x92P\x83\x01a\x16\x05V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x16\xD6W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xEDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x17\x01W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17\x10W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x17\"W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x178` \x89\x01a\x13\xD3V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x17NW`\0\x80\xFD[Pa\x17[\x88\x82\x89\x01a\x15\xBCV[\x95\x98\x94\x97P\x92\x95``\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\x82W`\0\x80\xFD[\x835a\x17\x8D\x81a\x13\xBEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA8W`\0\x80\xFD[a\x17\xB4\x86\x82\x87\x01a\x13\x13V[\x92PP`@\x84\x015a\x17\xC5\x81a\x13\xBEV[\x80\x91PP\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a\x17\xE3W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xFAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x18\x0EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18\x1DW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x182W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x18XW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18xWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x18\xC4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`@\x81R`\0a\x18\xDE`@\x83\x01\x85a\x14\xA6V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x18\xFF\x81\x84` \x87\x01a\x14VV[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19 W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19:W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19OW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19lW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x83W`\0\x80\xFD[a\x19\x8F\x88\x83\x89\x01a\x13\x13V[\x95P` \x87\x015\x91Pa\x19\xA1\x82a\x13\xBEV[\x90\x93P`@\x86\x015\x90\x80\x82\x11\x15a\x19\xB7W`\0\x80\xFD[Pa\x19\xC4\x87\x82\x88\x01a\x15\xBCV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\x1F\x82\x11\x15a\x1A\x1FW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\xFCWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A\x1BW\x82\x81U`\x01\x01a\x1A\x08V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A=Wa\x1A=a\x12\xCDV[a\x1AQ\x81a\x1AK\x84Ta\x18DV[\x84a\x19\xD5V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A\x86W`\0\x84\x15a\x1AnWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1A\x1BV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1A\xB5W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A\x96V[P\x85\x82\x10\x15a\x1A\xD3W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xE1;K\x02+\xF1\xB3GM\xE4\x08\xC5~\t\xB4\xD7-\xCF\x18)\xDA\xCD}\x96:k\xAB#*\x1E\x08\x88dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static EXTENSIONHANDLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xAFW`\x005`\xE0\x1C\x80c\r\xB7\x9B\x93\x14a\0\xB4W\x80c/\x1D}\xDC\x14a\x01\x05W\x80c:\x03s\x8A\x14a\x013W\x80cm\xFB\xE3^\x14a\x01xW\x80cqP\x18\xA6\x14a\x01\x98W\x80c\x8D\xA5\xCB[\x14a\x01\xA2W\x80c\x93\xECIq\x14a\x01\xAAW\x80c\x93\xF8xB\x14a\x01\xCAW\x80c\xBC'\xAE\xB3\x14a\x01\xFEW\x80c\xE0\x86\xA8\"\x14a\x02\x11W\x80c\xF1\xFB\xBB\xA1\x14a\x02$W\x80c\xF2\xFD\xE3\x8B\x14a\x027W\x80c\xF8\x84c\x0B\x14a\x02JW[`\0\x80\xFD[a\0\xE8a\0\xC26`\x04a\x13\x82V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01%a\x01\x136`\x04a\x13\xE3V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xFCV[a\0\xE8a\x01A6`\x04a\x14\x07V[`\x05` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Ba\x01\x866`\x04a\x13\xE3V[a\x02]V[`@Qa\0\xFC\x91\x90a\x15>V[a\x01\xA0a\x03\x93V[\0[a\0\xE8a\x03\xA7V[a\x01\xBDa\x01\xB86`\x04a\x15QV[a\x03\xB6V[`@Qa\0\xFC\x91\x90a\x15\x86V[a\0\xE8a\x01\xD86`\x04a\x13\x82V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x01\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xA0a\x02\x0C6`\x04a\x16\xBEV[a\x04\x88V[a\0\xE8a\x02\x1F6`\x04a\x14\x07V[a\r\x97V[a\x01\xA0a\x0226`\x04a\x17mV[a\x0E\x10V[a\x01\xA0a\x02E6`\x04a\x13\xE3V[a\x0EvV[a\x01\xA0a\x02X6`\x04a\x17\xD0V[a\x0E\xEFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x03\x88W\x83\x82\x90`\0R` `\0 \x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03uW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x02\xE8\x90a\x18DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x14\x90a\x18DV[\x80\x15a\x03aW\x80`\x1F\x10a\x036Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03aV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x02\xC9V[PPPP\x81R` \x01\x90`\x01\x01\x90a\x02\x95V[PPPP\x90P\x91\x90PV[a\x03\x9Ba\x10\xB7V[a\x03\xA5`\0a\x11\x16V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x03` R\x82`\0R`@`\0 \x82\x81T\x81\x10a\x03\xD2W`\0\x80\xFD[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x03\xEAW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x92P\x92PPP\x80Ta\x04\x07\x90a\x18DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x043\x90a\x18DV[\x80\x15a\x04\x80W\x80`\x1F\x10a\x04UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x01\x86\x86`@Qa\x04\xA5\x92\x91\x90a\x18~V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x19^\x1D\x19[\x9C\xDA[\xDB\x88\x1B\x98[YH\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`*\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rxinvalid extension address`8\x1B`D\x82\x01R`d\x01a\x05\x05V[\x83a\x05\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ruinvalid extension name`P\x1B`D\x82\x01R`d\x01a\x05\x05V[`\0\x81\x11a\x06\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FmaxExecutionGas must be larger t`D\x82\x01Rghan zero`\xC0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0\x82Q\x11a\x06gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FsubjectTemplates array cannot be`D\x82\x01Re empty`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x06\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x19^\x1D\x19[\x9C\xDA[\xDB\x88\x18[\x1C\x99XY\x1EH\x1C\x1DX\x9B\x1A\\\xDA\x19Y`*\x1B`D\x82\x01R`d\x01a\x05\x05V[```\0[\x83Q\x81\x10\x15a\t@W`\0\x84\x82\x81Q\x81\x10a\x06\xEDWa\x06\xEDa\x18\x8EV[` \x02` \x01\x01QQ\x11a\x07CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FsubjectTemplate cannot be empty\0`D\x82\x01R`d\x01a\x05\x05V[\x80`\0\x03a\x07\x86W\x83\x81\x81Q\x81\x10a\x07]Wa\x07]a\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x07wWa\x07wa\x18\x8EV[` \x02` \x01\x01Q\x91Pa\x08\x1FV[a\x07\xC3\x82\x85\x83\x81Q\x81\x10a\x07\x9CWa\x07\x9Ca\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x07\xB6Wa\x07\xB6a\x18\x8EV[` \x02` \x01\x01Qa\x11fV[a\x08\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FsubjectTemplates must have same `D\x82\x01Rf\x18\xDB\xDB[X[\x99`\xCA\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0`\x01[\x85\x83\x81Q\x81\x10a\x086Wa\x086a\x18\x8EV[` \x02` \x01\x01QQ\x81\x10\x15a\x08\xCAWa\x08\xA5\x86\x84\x81Q\x81\x10a\x08[Wa\x08[a\x18\x8EV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x08tWa\x08ta\x18\x8EV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j{recipient}`\xA8\x1B\x81RPa\x11fV[\x15a\x08\xB8W\x81a\x08\xB4\x81a\x18\xA4V[\x92PP[\x80a\x08\xC2\x81a\x18\xA4V[\x91PPa\x08$V[P`\x01\x81\x11\x15a\t-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Frecipient template can only be u`D\x82\x01Rgsed once`\xC0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[P\x80a\t8\x81a\x18\xA4V[\x91PPa\x06\xD0V[P`\0[\x81Q\x81\x10\x15a\t\xCFW\x81\x81\x81Q\x81\x10a\t_Wa\t_a\x18\x8EV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\x01`\xFD\x1B\x03a\t\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x18\xDB\xDB[X[\x99\x08\x1C\xDA\x1B\xDD[\x19\x08\x18\x99H\x1B\xDB\x99H\x1D\xDB\xDC\x99`2\x1B`D\x82\x01R`d\x01a\x05\x05V[\x80a\t\xC7\x81a\x18\xA4V[\x91PPa\tDV[Pa\t\xF6\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x14\xD9[\x99`\xE2\x1B\x81RPa\x11fV[\x15\x80\x15a\n)WPa\n'\x81`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fExecute`\xC8\x1B\x81RPa\x11fV[\x15[\x80\x15a\n[WPa\nY\x81`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x12[\x9C\xDD\x18[\x1B`\xCA\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\x8FWPa\n\x8D\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x15[\x9A[\x9C\xDD\x18[\x1B`\xBA\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\xBEWPa\n\xBC\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x11^\x1A]`\xE2\x1B\x81RPa\x11fV[\x15[\x80\x15a\n\xEDWPa\n\xEB\x81`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cDKIM`\xE0\x1B\x81RPa\x11fV[\x15[a\x0BCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcommand cannot be a reserved nam`D\x82\x01R`e`\xF8\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x0Br\x81`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l{tokenAmount}`\x98\x1B\x81RPa\x11fV[\x15\x80\x15a\x0B\xA6WPa\x0B\xA4\x81`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g{amount}`\xC0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0B\xD9WPa\x0B\xD7\x81`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g{string}`\xC0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0C\nWPa\x0C\x08\x81`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e{uint}`\xD0\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0C:WPa\x0C8\x81`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d{int}`\xD8\x1B\x81RPa\x11fV[\x15[\x80\x15a\x0CnWPa\x0Cl\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h{address}`\xB8\x1B\x81RPa\x11fV[\x15[a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcommand cannot be a template mat`D\x82\x01Rc1\xB42\xB9`\xE1\x1B`d\x82\x01R`\x84\x01a\x05\x05V[\x83`\x01\x87\x87`@Qa\x0C\xD9\x92\x91\x90a\x18~V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x86\x16`\0\x90\x81R`\x03\x82R\x91\x90\x91 \x84Qa\r\"\x92\x86\x01\x90a\x11|V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x84\x90UQa\rN\x90\x88\x90\x88\x90a\x18~V[`@Q\x80\x91\x03\x90 \x7F\xEF\x0E\x97\x1FC\xD94~\xAD|\xB6g\x91\xFD\xCC\xE2\xD7\xA5\x17\x1E\xE1\xF6R\xFE\xC7'\x82=\x1E-N\xE8\x85\x85`@Qa\r\x87\x92\x91\x90a\x18\xCBV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[`\0\x80`\x02\x83`@Qa\r\xAA\x91\x90a\x18\xEDV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 T`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\0\x90\x81R`\x05\x90\x95R\x92\x84 \x92\x16\x93Pa\r\xE3\x90\x86\x90a\x18\xEDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80\x15a\x0E\x08W\x80\x91P[P\x93\x92PPPV[a\x0E\x18a\x10\xB7V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x05` R`@\x90\x81\x90 \x90Q\x82\x91\x90a\x0EB\x90\x85\x90a\x18\xEDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x0E~a\x10\xB7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x0E\xEC\x81a\x11\x16V[PV[a\x0E\xF7a\x10\xB7V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0FQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fdefault extensions already set\0\0`D\x82\x01R`d\x01a\x05\x05V[`\0[\x81\x81\x10\x15a\x10\x9FW`\0\x80`\0\x80\x86\x86\x86\x81\x81\x10a\x0FtWa\x0Fta\x18\x8EV[\x90P` \x02\x81\x01\x90a\x0F\x86\x91\x90a\x19\tV[\x81\x01\x90a\x0F\x93\x91\x90a\x19VV[\x93P\x93P\x93P\x93P\x82`\x01\x85`@Qa\x0F\xAC\x91\x90a\x18\xEDV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x85\x16`\0\x90\x81R`\x03\x82R\x91\x90\x91 \x83Qa\x0F\xF5\x92\x85\x01\x90a\x11|V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x82\x90U\x82Q\x84\x91`\x02\x91\x85\x91\x90a\x10'Wa\x10'a\x18\x8EV[` \x02` \x01\x01Q`\0\x81Q\x81\x10a\x10AWa\x10Aa\x18\x8EV[` \x02` \x01\x01Q`@Qa\x10V\x91\x90a\x18\xEDV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPP\x80\x80a\x10\x97\x90a\x18\xA4V[\x91PPa\x0FTV[PP`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPV[3a\x10\xC0a\x03\xA7V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x05V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x11\xC9W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x11\xC9W\x82Q\x80Qa\x11\xB9\x91\x84\x91` \x90\x91\x01\x90a\x11\xD9V[P\x91` \x01\x91\x90`\x01\x01\x90a\x11\x9CV[Pa\x11\xD5\x92\x91Pa\x12+V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\x1FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\x1FW\x82Q\x82\x90a\x12\x0F\x90\x82a\x1A$V[P\x91` \x01\x91\x90`\x01\x01\x90a\x11\xF9V[Pa\x11\xD5\x92\x91Pa\x12HV[\x80\x82\x11\x15a\x11\xD5W`\0a\x12?\x82\x82a\x12eV[P`\x01\x01a\x12+V[\x80\x82\x11\x15a\x11\xD5W`\0a\x12\\\x82\x82a\x12\x83V[P`\x01\x01a\x12HV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0E\xEC\x91\x90a\x12HV[P\x80Ta\x12\x8F\x90a\x18DV[`\0\x82U\x80`\x1F\x10a\x12\x9FWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0E\xEC\x91\x90[\x80\x82\x11\x15a\x11\xD5W`\0\x81U`\x01\x01a\x12\xB9V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x0BWa\x13\x0Ba\x12\xCDV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13=Wa\x13=a\x12\xCDV[a\x13P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\xE3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x13eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\x94W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xAAW`\0\x80\xFD[a\x13\xB6\x84\x82\x85\x01a\x13\x13V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xECW`\0\x80\xFD[\x805a\x13\xDE\x81a\x13\xBEV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\xF5W`\0\x80\xFD[\x815a\x14\0\x81a\x13\xBEV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x1AW`\0\x80\xFD[\x825a\x14%\x81a\x13\xBEV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14@W`\0\x80\xFD[a\x14L\x85\x82\x86\x01a\x13\x13V[\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x14qW\x81\x81\x01Q\x83\x82\x01R` \x01a\x14YV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\x92\x81` \x86\x01` \x86\x01a\x14VV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x15/W\x84\x84\x03\x8BR\x82Q\x80Q\x80\x86R\x90\x88\x01\x90\x88\x86\x01\x90\x80\x89\x1B\x87\x01\x8A\x01\x85[\x82\x81\x10\x15a\x15\x19W`\x1F\x19\x89\x83\x03\x01\x84Ra\x15\x07\x82\x86Qa\x14zV[\x94\x8C\x01\x94\x93\x8C\x01\x93\x91P`\x01\x01a\x14\xEBV[P\x9D\x8A\x01\x9D\x96PPP\x92\x87\x01\x92P`\x01\x01a\x14\xC6V[P\x91\x99\x98PPPPPPPPPV[` \x81R`\0a\x14\0` \x83\x01\x84a\x14\xA6V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15fW`\0\x80\xFD[\x835a\x15q\x81a\x13\xBEV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x81R`\0a\x14\0` \x83\x01\x84a\x14zV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\xB2Wa\x15\xB2a\x12\xCDV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x15\xCDW`\0\x80\xFD[\x815` a\x15\xE2a\x15\xDD\x83a\x15\x99V[a\x12\xE3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x16\x01W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16\xB3W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16$W`\0\x80\xFD[\x81\x89\x01\x91P\x89`?\x83\x01\x12a\x168W`\0\x80\xFD[\x85\x82\x015a\x16Ha\x15\xDD\x82a\x15\x99V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x87\x81\x01\x90\x8C\x83\x11\x15a\x16hW`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\x16\xA1W\x805\x85\x81\x11\x15a\x16\x84W`\0\x80\xFD[a\x16\x93\x8F`@\x83\x8A\x01\x01a\x13\x13V[\x84RP\x91\x89\x01\x91\x89\x01a\x16mV[P\x87RPPP\x92\x84\x01\x92P\x83\x01a\x16\x05V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x16\xD6W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xEDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x17\x01W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17\x10W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x17\"W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x178` \x89\x01a\x13\xD3V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x17NW`\0\x80\xFD[Pa\x17[\x88\x82\x89\x01a\x15\xBCV[\x95\x98\x94\x97P\x92\x95``\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\x82W`\0\x80\xFD[\x835a\x17\x8D\x81a\x13\xBEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA8W`\0\x80\xFD[a\x17\xB4\x86\x82\x87\x01a\x13\x13V[\x92PP`@\x84\x015a\x17\xC5\x81a\x13\xBEV[\x80\x91PP\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a\x17\xE3W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xFAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x18\x0EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18\x1DW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x182W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x18XW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18xWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x18\xC4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`@\x81R`\0a\x18\xDE`@\x83\x01\x85a\x14\xA6V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x18\xFF\x81\x84` \x87\x01a\x14VV[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19 W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19:W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19OW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19lW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x83W`\0\x80\xFD[a\x19\x8F\x88\x83\x89\x01a\x13\x13V[\x95P` \x87\x015\x91Pa\x19\xA1\x82a\x13\xBEV[\x90\x93P`@\x86\x015\x90\x80\x82\x11\x15a\x19\xB7W`\0\x80\xFD[Pa\x19\xC4\x87\x82\x88\x01a\x15\xBCV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\x1F\x82\x11\x15a\x1A\x1FW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\xFCWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A\x1BW\x82\x81U`\x01\x01a\x1A\x08V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A=Wa\x1A=a\x12\xCDV[a\x1AQ\x81a\x1AK\x84Ta\x18DV[\x84a\x19\xD5V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A\x86W`\0\x84\x15a\x1AnWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1A\x1BV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1A\xB5W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A\x96V[P\x85\x82\x10\x15a\x1A\xD3W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xE1;K\x02+\xF1\xB3GM\xE4\x08\xC5~\t\xB4\xD7-\xCF\x18)\xDA\xCD}\x96:k\xAB#*\x1E\x08\x88dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static EXTENSIONHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                EXTENSIONHANDLER_ABI.clone(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([147, 248, 120, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultExtensionOfCommand` (0x0db79b93) function
        pub fn default_extension_of_command(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([13, 183, 155, 147], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExtensionForCommand` (0xe086a822) function
        pub fn get_extension_for_command(
            &self,
            wallet_addr: ::ethers::core::types::Address,
            command: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
                .method_hash([241, 251, 187, 161], (wallet_addr, command, extension_addr))
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
        ///Calls the contract's `userExtensionOfCommand` (0x3a03738a) function
        pub fn user_extension_of_command(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([58, 3, 115, 138], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ExtensionPublished` event
        pub fn extension_published_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExtensionPublishedFilter>
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExtensionHandlerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ExtensionHandler<M>
    {
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExtensionHandlerEvents {
        ExtensionPublishedFilter(ExtensionPublishedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for ExtensionHandlerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ExtensionPublishedFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::ExtensionPublishedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ExtensionHandlerEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ExtensionHandlerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExtensionPublishedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExtensionPublishedFilter> for ExtensionHandlerEvents {
        fn from(value: ExtensionPublishedFilter) -> Self {
            Self::ExtensionPublishedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ExtensionHandlerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
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
        Hash,
    )]
    #[ethcall(
        name = "addressOfExtensionName",
        abi = "addressOfExtensionName(string)"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "getSubjectTemplatesOfExtension",
        abi = "getSubjectTemplatesOfExtension(address)"
    )]
    pub struct GetSubjectTemplatesOfExtensionCall {
        pub extension_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxGasOfExtension` function with signature `maxGasOfExtension(address)` and selector `0x2f1d7ddc`
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
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `publishExtension` function with signature `publishExtension(string,address,string[][],uint256)` and selector `0xbc27aeb3`
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
        Hash,
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
        MaxGasOfExtension(MaxGasOfExtensionCall),
        Owner(OwnerCall),
        PublishExtension(PublishExtensionCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDefaultExtensions(SetDefaultExtensionsCall),
        SetExtensionForCommand(SetExtensionForCommandCall),
        SubjectTemplatesOfExtension(SubjectTemplatesOfExtensionCall),
        TransferOwnership(TransferOwnershipCall),
        UserExtensionOfCommand(UserExtensionOfCommandCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExtensionHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddressOfExtensionNameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressOfExtensionName(decoded));
            }
            if let Ok(decoded) =
                <DefaultExtensionOfCommandCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultExtensionOfCommand(decoded));
            }
            if let Ok(decoded) =
                <GetExtensionForCommandCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetExtensionForCommand(decoded));
            }
            if let Ok(decoded) =
                <GetSubjectTemplatesOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubjectTemplatesOfExtension(decoded));
            }
            if let Ok(decoded) =
                <MaxGasOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxGasOfExtension(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PublishExtensionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PublishExtension(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultExtensionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetDefaultExtensions(decoded));
            }
            if let Ok(decoded) =
                <SetExtensionForCommandCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetExtensionForCommand(decoded));
            }
            if let Ok(decoded) =
                <SubjectTemplatesOfExtensionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubjectTemplatesOfExtension(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UserExtensionOfCommandCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
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
                Self::MaxGasOfExtension(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PublishExtension(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDefaultExtensions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetExtensionForCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubjectTemplatesOfExtension(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserExtensionOfCommand(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExtensionHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressOfExtensionName(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultExtensionOfCommand(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExtensionForCommand(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubjectTemplatesOfExtension(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxGasOfExtension(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublishExtension(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultExtensions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetExtensionForCommand(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubjectTemplatesOfExtension(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserExtensionOfCommand(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetSubjectTemplatesOfExtensionCall> for ExtensionHandlerCalls {
        fn from(value: GetSubjectTemplatesOfExtensionCall) -> Self {
            Self::GetSubjectTemplatesOfExtension(value)
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
    impl ::core::convert::From<SubjectTemplatesOfExtensionCall> for ExtensionHandlerCalls {
        fn from(value: SubjectTemplatesOfExtensionCall) -> Self {
            Self::SubjectTemplatesOfExtension(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ExtensionHandlerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `subjectTemplatesOfExtension` function with signature `subjectTemplatesOfExtension(address,uint256,uint256)` and selector `0x93ec4971`
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
        Hash,
    )]
    pub struct UserExtensionOfCommandReturn(pub ::ethers::core::types::Address);
}
