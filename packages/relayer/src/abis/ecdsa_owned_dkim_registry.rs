pub use ecdsa_owned_dkim_registry::*;
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
pub mod ecdsa_owned_dkim_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_signer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prefix"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
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
                    ::std::borrow::ToOwned::to_owned("dkimRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dkimRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract DKIMRegistry"),
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
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
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
                    ::std::borrow::ToOwned::to_owned("revokeDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signer"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ECDSAOWNEDDKIMREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1B\x188\x03\x80a\x1B\x18\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x99V[`@Qa\0;\x90a\0\x8CV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0WW=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90Ua\0\xC9V[a\x08\x06\x80a\x13\x12\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\0\xABW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC2W`\0\x80\xFD[\x93\x92PPPV[a\x12:\x80a\0\xD8`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c\x07\xF1\xEA\xF5\x14a\0}W\x80c#\x8A\xC93\x14a\0\xB6W\x80cd#\xF1\xE2\x14a\0\xE1W\x80c\x97\x17\x0F+\x14a\0\xF4W\x80c\xAE\xC7\x93a\x14a\x01\tW\x80c\xD5\x07\xC3 \x14a\x01\x1CW\x80c\xE7\xA7\x97z\x14a\x01BW\x80c\xF6\xB4\x93D\x14a\x01eW[`\0\x80\xFD[a\0\xA0`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xA2\xAA\x1D`\xE1\x1B\x81RP\x81V[`@Qa\0\xAD\x91\x90a\r V[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xADV[`\0Ta\0\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x07a\x01\x026`\x04a\r\xDEV[a\x01xV[\0[a\0\xA0a\x01\x176`\x04a\x0E\x83V[a\x03\xCDV[a\0\xA0`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f)\"\xAB'\xA5\xA2\x9D`\xC9\x1B\x81RP\x81V[a\x01Ua\x01P6`\x04a\x0F\x12V[a\x04\x11V[`@Q\x90\x15\x15\x81R` \x01a\0\xADV[a\x01\x07a\x01s6`\x04a\r\xDEV[a\x04\x8EV[\x83Q`\0\x03a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0FVV[`@Q\x80\x91\x03\x90\xFD[\x82Q`\0\x03a\x01\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\x80V[\x81a\x01\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\xADV[a\x01\xEA\x83\x83a\x04\x11V[\x15a\x026W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]`\"\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0T`@Qc\x08Z\xF9s`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA3\x91\x90a\x0F\xDEV[\x15a\x02\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x1C\x99]\x9B\xDA\xD9Y`B\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0a\x03\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xA2\xAA\x1D`\xE1\x1B\x81RP\x86\x86\x86a\x03\xCDV[\x90P`\0a\x03\"\x82a\x06\xA6V[\x90P`\0a\x030\x82\x85a\x06\xE1V[`\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x03`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10\0V[`\0T`@Qc\xC1\\\xFF\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x03\x92\x90\x89\x90\x89\x90`\x04\x01a\x10+V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC0W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x84a\x03\xD9Fa\x07\x05V[\x85\x85a\x03\xE4\x86a\x07\x97V[`@Q` \x01a\x03\xF8\x95\x94\x93\x92\x91\x90a\x10MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@Qcs\xD3\xCB\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x04D\x90\x86\x90\x86\x90`\x04\x01a\x10+V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x85\x91\x90a\x0F\xDEV[\x90P[\x92\x91PPV[\x83Q`\0\x03a\x04\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0FVV[\x82Q`\0\x03a\x04\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\x80V[\x81a\x04\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\xADV[a\x04\xF7\x83\x83a\x04\x11V[\x15\x15`\x01\x14a\x05CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x1C\xD9]`B\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0T`@Qc\x08Z\xF9s`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB0\x91\x90a\x0F\xDEV[\x15a\x05\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x01\x99V[`\0a\x06*`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f)\"\xAB'\xA5\xA2\x9D`\xC9\x1B\x81RP\x86\x86\x86a\x03\xCDV[\x90P`\0a\x067\x82a\x06\xA6V[\x90P`\0a\x06E\x82\x85a\x06\xE1V[`\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x06uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10\0V[`\0T`@Qc\n\xE9(\x97`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x03\x92V[`\0a\x06\xB2\x82Qa\x07\x05V[\x82`@Q` \x01a\x06\xC4\x92\x91\x90a\x11(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x06\xF0\x85\x85a\x07\xAEV[\x91P\x91Pa\x06\xFD\x81a\x07\xF3V[P\x93\x92PPPV[```\0a\x07\x12\x83a\t;V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x071Wa\x071a\r3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x07eWP\x93\x92PPPV[``a\x04\x88\x82a\x07\xA6\x84a\n\x11V[`\x01\x01a\n{V[`\0\x80\x82Q`A\x03a\x07\xE4W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x07\xD8\x87\x82\x85\x85a\x0C\x16V[\x94P\x94PPPPa\x07\xECV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x08\x07Wa\x08\x07a\x11\x81V[\x03a\x08\x0FWPV[`\x01\x81`\x04\x81\x11\x15a\x08#Wa\x08#a\x11\x81V[\x03a\x08kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwECDSA: invalid signature`@\x1B`D\x82\x01R`d\x01a\x01\x99V[`\x02\x81`\x04\x81\x11\x15a\x08\x7FWa\x08\x7Fa\x11\x81V[\x03a\x08\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x01\x99V[`\x03\x81`\x04\x81\x11\x15a\x08\xE0Wa\x08\xE0a\x11\x81V[\x03a\t8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x01\x99V[PV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\tzWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[i\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x10a\t\xA4Wi\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\t\xC2Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\t\xDAWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\t\xEEWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\n\0W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x88W`\x01\x01\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a\n)W`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\n>W`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\nSW` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\nhW`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x04\x88W`\x01\x01\x92\x91PPV[```\0a\n\x8A\x83`\x02a\x11\xADV[a\n\x95\x90`\x02a\x11\xC4V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xACWa\n\xACa\r3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\n\xF1Wa\n\xF1a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0B Wa\x0B a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0BD\x84`\x02a\x11\xADV[a\x0BO\x90`\x01a\x11\xC4V[\x90P[`\x01\x81\x11\x15a\x0B\xC7Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0B\x83Wa\x0B\x83a\x11\xD7V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0B\x99Wa\x0B\x99a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0B\xC0\x81a\x11\xEDV[\x90Pa\x0BRV[P\x83\x15a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x01\x99V[`\0\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\x0CCWP`\0\x90P`\x03a\x0C\xC7V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0C\x97W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xC0W`\0`\x01\x92P\x92PPa\x0C\xC7V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0[\x83\x81\x10\x15a\x0C\xEBW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xD3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r\x0C\x81` \x86\x01` \x86\x01a\x0C\xD0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\x85` \x83\x01\x84a\x0C\xF4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\rcWa\rca\r3V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\r\x8BWa\r\x8Ba\r3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\r\xA4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\r\xCFW`\0\x80\xFD[a\x04\x85\x83\x835` \x85\x01a\rIV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\x0BW`\0\x80\xFD[a\x0E\x17\x88\x83\x89\x01a\r\xBEV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x0E-W`\0\x80\xFD[a\x0E9\x88\x83\x89\x01a\r\xBEV[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0EVW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a\x0EhW`\0\x80\xFD[a\x0Ew\x87\x825` \x84\x01a\rIV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E\x99W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xB0W`\0\x80\xFD[a\x0E\xBC\x88\x83\x89\x01a\r\xBEV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x0E\xD2W`\0\x80\xFD[a\x0E\xDE\x88\x83\x89\x01a\r\xBEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x0E\xF4W`\0\x80\xFD[Pa\x0F\x01\x87\x82\x88\x01a\r\xBEV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F;W`\0\x80\xFD[a\x0FG\x85\x82\x86\x01a\r\xBEV[\x95` \x94\x90\x94\x015\x94PPPPV[` \x80\x82R`\x10\x90\x82\x01Ro$\xB7;0\xB64\xB2\x109\xB2\xB62\xB1\xBA7\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x13\x90\x82\x01RrInvalid domain name`h\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x17\x90\x82\x01Rv\t-\xCE\xCC-\x8D,\x84\x0E\x0E\xACM\x8D,d\rl\xAF$\r\x0C.m`K\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0F\xF0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\x85W`\0\x80\xFD[` \x80\x82R`\x11\x90\x82\x01RpInvalid signature`x\x1B`@\x82\x01R``\x01\x90V[`@\x81R`\0a\x10>`@\x83\x01\x85a\x0C\xF4V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x86Qa\x10_\x81\x84` \x8B\x01a\x0C\xD0V[hchain_id=`\xB8\x1B\x90\x83\x01\x90\x81R\x86Qa\x10\x85\x81`\t\x84\x01` \x8B\x01a\x0C\xD0V[i;selector=`\xB0\x1B`\t\x92\x90\x91\x01\x91\x82\x01R\x85Qa\x10\xB0\x81`\x13\x84\x01` \x8A\x01a\x0C\xD0V[g;domain=`\xC0\x1B`\x13\x92\x90\x91\x01\x91\x82\x01R\x84Qa\x10\xD9\x81`\x1B\x84\x01` \x89\x01a\x0C\xD0V[p;public_key_hash=`x\x1B`\x1B\x92\x90\x91\x01\x91\x82\x01R\x83Qa\x11\x0B\x81`,\x84\x01` \x88\x01a\x0C\xD0V[`;`\xF8\x1B`,\x92\x90\x91\x01\x91\x82\x01R`-\x01\x97\x96PPPPPPPV[y\x0C\xA2\xBA42\xB92\xBA\xB6\x90)\xB4\xB3\xB72\xB2\x10&\xB2\xB9\xB9\xB0\xB3\xB2\x9D\x05`1\x1B\x81R\x82Q`\0\x90a\x11^\x81`\x1A\x85\x01` \x88\x01a\x0C\xD0V[\x83Q\x90\x83\x01\x90a\x11u\x81`\x1A\x84\x01` \x88\x01a\x0C\xD0V[\x01`\x1A\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x88Wa\x04\x88a\x11\x97V[\x80\x82\x01\x80\x82\x11\x15a\x04\x88Wa\x04\x88a\x11\x97V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x11\xFCWa\x11\xFCa\x11\x97V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 r\xA9a\xFF\xA0\x10\xA20\x123*\xB1\xD9\n\0\x85\xC6>\x105\x84wv\x93v\x89\xF8\xA3\xB3\xC3\x92\xCBdsolcC\0\x08\x17\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x07\x88\x80a\0~`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x83W`\x005`\xE0\x1C\x80c\x06\x90\xBD8\x14a\0\x88W\x80c\x15\xD2Q.\x14a\0\xDBW\x80cB\xD7\xCB\x98\x14a\0\xF0W\x80cqP\x18\xA6\x14a\x01\x13W\x80c\x8D\xA5\xCB[\x14a\x01\x1BW\x80c\xC1\\\xFF\xAB\x14a\x01;W\x80c\xE7\xA7\x97z\x14a\x01NW\x80c\xF2\xFD\xE3\x8B\x14a\x01aW\x80c\xF4\x9E\xB1d\x14a\x01tW[`\0\x80\xFD[a\0\xC6a\0\x966`\x04a\x05kV[\x81Q` \x81\x84\x01\x81\x01\x80Q`\x01\x82R\x92\x82\x01\x94\x82\x01\x94\x90\x94 \x91\x90\x93R\x90\x91R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xE96`\x04a\x05\xAFV[a\x01\x87V[\0[a\0\xC6a\0\xFE6`\x04a\x05\xAFV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xEEa\x01\xE5V[a\x01#a\x01\xF9V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD2V[a\0\xEEa\x01I6`\x04a\x05kV[a\x02\x08V[a\0\xC6a\x01\\6`\x04a\x05kV[a\x02\xE6V[a\0\xEEa\x01o6`\x04a\x05\xC8V[a\x03JV[a\0\xEEa\x01\x826`\x04a\x05\xF8V[a\x03\xC3V[a\x01\x8Fa\x04\x07V[`\0\x81\x81R`\x02` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xB8\x0F\xFF+Lo=\xDF\x80Hw\x92|w\xB2\xFE\x18q\xCE\xCA\xA5\xADC\xD2\xC7\xC4/As1\xF8e\x90a\x01\xDA\x90\x83\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[a\x01\xEDa\x04\x07V[a\x01\xF7`\0a\x04fV[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x02\x10a\x04\x07V[`\0\x81\x81R`\x02` R`@\x90 T`\xFF\x16\x15a\x02pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rxcannot set revoked pubkey`8\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80\x83`@Qa\x02\x81\x91\x90a\x06\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 `\0\x86\x81R\x93R\x91 \x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91U\x7FQ\r\xAC\x88\xEA\xF2\xDF\xBDS\x90BA\xFC\x19\x9A\xD4k c\xE6\xBFl?\xB2\x91\xF8\xCE\x86Cf4\x19\x90a\x02\xDA\x90\x84\x90\x84\x90a\x07\x01V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81\x81R`\x02` R`@\x81 T`\xFF\x16\x15a\x03\x05WP`\0a\x03DV[`\x01\x83`@Qa\x03\x15\x91\x90a\x06\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x85\x81R\x92R\x90 T`\xFF\x16\x15a\x03@WP`\x01a\x03DV[P`\0[\x92\x91PPV[a\x03Ra\x04\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x02gV[a\x03\xC0\x81a\x04fV[PV[a\x03\xCBa\x04\x07V[`\0[\x81Q\x81\x10\x15a\x04\x02Wa\x03\xFA\x83\x83\x83\x81Q\x81\x10a\x03\xEDWa\x03\xEDa\x07<V[` \x02` \x01\x01Qa\x02\x08V[`\x01\x01a\x03\xCEV[PPPV[3a\x04\x10a\x01\xF9V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02gV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04\xF4Wa\x04\xF4a\x04\xB6V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05\rW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05&Wa\x05&a\x04\xB6V[a\x059`\x1F\x82\x01`\x1F\x19\x16` \x01a\x04\xCCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05NW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05~W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\x94W`\0\x80\xFD[a\x05\xA0\x85\x82\x86\x01a\x04\xFCV[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x05\xC1W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xDAW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xF1W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x0BW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x06\"W`\0\x80\xFD[a\x06.\x86\x83\x87\x01a\x04\xFCV[\x93P` \x91P\x81\x85\x015\x81\x81\x11\x15a\x06EW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x06VW`\0\x80\xFD[\x805\x82\x81\x11\x15a\x06hWa\x06ha\x04\xB6V[\x80`\x05\x1B\x92Pa\x06y\x84\x84\x01a\x04\xCCV[\x81\x81R\x92\x82\x01\x84\x01\x92\x84\x81\x01\x90\x89\x85\x11\x15a\x06\x93W`\0\x80\xFD[\x92\x85\x01\x92[\x84\x84\x10\x15a\x06\xB1W\x835\x82R\x92\x85\x01\x92\x90\x85\x01\x90a\x06\x98V[\x80\x96PPPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x06\xDCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xC4V[PP`\0\x91\x01RV[`\0\x82Qa\x06\xF7\x81\x84` \x87\x01a\x06\xC1V[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x07 \x81``\x85\x01` \x88\x01a\x06\xC1V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01``\x01\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xB1~[\x1E\xEB\x81\x91b\xEFmu\xEB\xC7\xD1]\xAC\x01\xD7\r\x86\xC2>;2\xB8\xDF8\xCC\x030\x89\xB0dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static ECDSAOWNEDDKIMREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c\x07\xF1\xEA\xF5\x14a\0}W\x80c#\x8A\xC93\x14a\0\xB6W\x80cd#\xF1\xE2\x14a\0\xE1W\x80c\x97\x17\x0F+\x14a\0\xF4W\x80c\xAE\xC7\x93a\x14a\x01\tW\x80c\xD5\x07\xC3 \x14a\x01\x1CW\x80c\xE7\xA7\x97z\x14a\x01BW\x80c\xF6\xB4\x93D\x14a\x01eW[`\0\x80\xFD[a\0\xA0`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xA2\xAA\x1D`\xE1\x1B\x81RP\x81V[`@Qa\0\xAD\x91\x90a\r V[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xADV[`\0Ta\0\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x07a\x01\x026`\x04a\r\xDEV[a\x01xV[\0[a\0\xA0a\x01\x176`\x04a\x0E\x83V[a\x03\xCDV[a\0\xA0`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f)\"\xAB'\xA5\xA2\x9D`\xC9\x1B\x81RP\x81V[a\x01Ua\x01P6`\x04a\x0F\x12V[a\x04\x11V[`@Q\x90\x15\x15\x81R` \x01a\0\xADV[a\x01\x07a\x01s6`\x04a\r\xDEV[a\x04\x8EV[\x83Q`\0\x03a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0FVV[`@Q\x80\x91\x03\x90\xFD[\x82Q`\0\x03a\x01\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\x80V[\x81a\x01\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\xADV[a\x01\xEA\x83\x83a\x04\x11V[\x15a\x026W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]`\"\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0T`@Qc\x08Z\xF9s`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA3\x91\x90a\x0F\xDEV[\x15a\x02\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x1C\x99]\x9B\xDA\xD9Y`B\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0a\x03\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xA2\xAA\x1D`\xE1\x1B\x81RP\x86\x86\x86a\x03\xCDV[\x90P`\0a\x03\"\x82a\x06\xA6V[\x90P`\0a\x030\x82\x85a\x06\xE1V[`\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x03`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10\0V[`\0T`@Qc\xC1\\\xFF\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x03\x92\x90\x89\x90\x89\x90`\x04\x01a\x10+V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC0W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x84a\x03\xD9Fa\x07\x05V[\x85\x85a\x03\xE4\x86a\x07\x97V[`@Q` \x01a\x03\xF8\x95\x94\x93\x92\x91\x90a\x10MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@Qcs\xD3\xCB\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x04D\x90\x86\x90\x86\x90`\x04\x01a\x10+V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x85\x91\x90a\x0F\xDEV[\x90P[\x92\x91PPV[\x83Q`\0\x03a\x04\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0FVV[\x82Q`\0\x03a\x04\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\x80V[\x81a\x04\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x0F\xADV[a\x04\xF7\x83\x83a\x04\x11V[\x15\x15`\x01\x14a\x05CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x1C\x1DX\x9B\x1AX\xD2\xD9^R\x18\\\xDA\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x1C\xD9]`B\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0T`@Qc\x08Z\xF9s`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB0\x91\x90a\x0F\xDEV[\x15a\x05\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x01\x99V[`\0a\x06*`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f)\"\xAB'\xA5\xA2\x9D`\xC9\x1B\x81RP\x86\x86\x86a\x03\xCDV[\x90P`\0a\x067\x82a\x06\xA6V[\x90P`\0a\x06E\x82\x85a\x06\xE1V[`\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x06uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10\0V[`\0T`@Qc\n\xE9(\x97`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x03\x92V[`\0a\x06\xB2\x82Qa\x07\x05V[\x82`@Q` \x01a\x06\xC4\x92\x91\x90a\x11(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x06\xF0\x85\x85a\x07\xAEV[\x91P\x91Pa\x06\xFD\x81a\x07\xF3V[P\x93\x92PPPV[```\0a\x07\x12\x83a\t;V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x071Wa\x071a\r3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x07eWP\x93\x92PPPV[``a\x04\x88\x82a\x07\xA6\x84a\n\x11V[`\x01\x01a\n{V[`\0\x80\x82Q`A\x03a\x07\xE4W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x07\xD8\x87\x82\x85\x85a\x0C\x16V[\x94P\x94PPPPa\x07\xECV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x08\x07Wa\x08\x07a\x11\x81V[\x03a\x08\x0FWPV[`\x01\x81`\x04\x81\x11\x15a\x08#Wa\x08#a\x11\x81V[\x03a\x08kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwECDSA: invalid signature`@\x1B`D\x82\x01R`d\x01a\x01\x99V[`\x02\x81`\x04\x81\x11\x15a\x08\x7FWa\x08\x7Fa\x11\x81V[\x03a\x08\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x01\x99V[`\x03\x81`\x04\x81\x11\x15a\x08\xE0Wa\x08\xE0a\x11\x81V[\x03a\t8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x01\x99V[PV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\tzWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[i\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x10a\t\xA4Wi\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\t\xC2Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\t\xDAWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\t\xEEWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\n\0W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x88W`\x01\x01\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a\n)W`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\n>W`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\nSW` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\nhW`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x04\x88W`\x01\x01\x92\x91PPV[```\0a\n\x8A\x83`\x02a\x11\xADV[a\n\x95\x90`\x02a\x11\xC4V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xACWa\n\xACa\r3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\n\xF1Wa\n\xF1a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0B Wa\x0B a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0BD\x84`\x02a\x11\xADV[a\x0BO\x90`\x01a\x11\xC4V[\x90P[`\x01\x81\x11\x15a\x0B\xC7Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0B\x83Wa\x0B\x83a\x11\xD7V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0B\x99Wa\x0B\x99a\x11\xD7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0B\xC0\x81a\x11\xEDV[\x90Pa\x0BRV[P\x83\x15a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x01\x99V[`\0\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\x0CCWP`\0\x90P`\x03a\x0C\xC7V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0C\x97W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xC0W`\0`\x01\x92P\x92PPa\x0C\xC7V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0[\x83\x81\x10\x15a\x0C\xEBW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xD3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r\x0C\x81` \x86\x01` \x86\x01a\x0C\xD0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\x85` \x83\x01\x84a\x0C\xF4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\rcWa\rca\r3V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\r\x8BWa\r\x8Ba\r3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\r\xA4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\r\xCFW`\0\x80\xFD[a\x04\x85\x83\x835` \x85\x01a\rIV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\x0BW`\0\x80\xFD[a\x0E\x17\x88\x83\x89\x01a\r\xBEV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x0E-W`\0\x80\xFD[a\x0E9\x88\x83\x89\x01a\r\xBEV[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0EVW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a\x0EhW`\0\x80\xFD[a\x0Ew\x87\x825` \x84\x01a\rIV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E\x99W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xB0W`\0\x80\xFD[a\x0E\xBC\x88\x83\x89\x01a\r\xBEV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x0E\xD2W`\0\x80\xFD[a\x0E\xDE\x88\x83\x89\x01a\r\xBEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x0E\xF4W`\0\x80\xFD[Pa\x0F\x01\x87\x82\x88\x01a\r\xBEV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F;W`\0\x80\xFD[a\x0FG\x85\x82\x86\x01a\r\xBEV[\x95` \x94\x90\x94\x015\x94PPPPV[` \x80\x82R`\x10\x90\x82\x01Ro$\xB7;0\xB64\xB2\x109\xB2\xB62\xB1\xBA7\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x13\x90\x82\x01RrInvalid domain name`h\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x17\x90\x82\x01Rv\t-\xCE\xCC-\x8D,\x84\x0E\x0E\xACM\x8D,d\rl\xAF$\r\x0C.m`K\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0F\xF0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\x85W`\0\x80\xFD[` \x80\x82R`\x11\x90\x82\x01RpInvalid signature`x\x1B`@\x82\x01R``\x01\x90V[`@\x81R`\0a\x10>`@\x83\x01\x85a\x0C\xF4V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x86Qa\x10_\x81\x84` \x8B\x01a\x0C\xD0V[hchain_id=`\xB8\x1B\x90\x83\x01\x90\x81R\x86Qa\x10\x85\x81`\t\x84\x01` \x8B\x01a\x0C\xD0V[i;selector=`\xB0\x1B`\t\x92\x90\x91\x01\x91\x82\x01R\x85Qa\x10\xB0\x81`\x13\x84\x01` \x8A\x01a\x0C\xD0V[g;domain=`\xC0\x1B`\x13\x92\x90\x91\x01\x91\x82\x01R\x84Qa\x10\xD9\x81`\x1B\x84\x01` \x89\x01a\x0C\xD0V[p;public_key_hash=`x\x1B`\x1B\x92\x90\x91\x01\x91\x82\x01R\x83Qa\x11\x0B\x81`,\x84\x01` \x88\x01a\x0C\xD0V[`;`\xF8\x1B`,\x92\x90\x91\x01\x91\x82\x01R`-\x01\x97\x96PPPPPPPV[y\x0C\xA2\xBA42\xB92\xBA\xB6\x90)\xB4\xB3\xB72\xB2\x10&\xB2\xB9\xB9\xB0\xB3\xB2\x9D\x05`1\x1B\x81R\x82Q`\0\x90a\x11^\x81`\x1A\x85\x01` \x88\x01a\x0C\xD0V[\x83Q\x90\x83\x01\x90a\x11u\x81`\x1A\x84\x01` \x88\x01a\x0C\xD0V[\x01`\x1A\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x88Wa\x04\x88a\x11\x97V[\x80\x82\x01\x80\x82\x11\x15a\x04\x88Wa\x04\x88a\x11\x97V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x11\xFCWa\x11\xFCa\x11\x97V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 r\xA9a\xFF\xA0\x10\xA20\x123*\xB1\xD9\n\0\x85\xC6>\x105\x84wv\x93v\x89\xF8\xA3\xB3\xC3\x92\xCBdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static ECDSAOWNEDDKIMREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ECDSAOwnedDKIMRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ECDSAOwnedDKIMRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ECDSAOwnedDKIMRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ECDSAOwnedDKIMRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ECDSAOwnedDKIMRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ECDSAOwnedDKIMRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ECDSAOwnedDKIMRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ECDSAOWNEDDKIMREGISTRY_ABI.clone(),
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
                ECDSAOWNEDDKIMREGISTRY_ABI.clone(),
                ECDSAOWNEDDKIMREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `REVOKE_PREFIX` (0xd507c320) function
        pub fn revoke_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([213, 7, 195, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SET_PREFIX` (0x07f1eaf5) function
        pub fn set_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([7, 241, 234, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeSignedMsg` (0xaec79361) function
        pub fn compute_signed_msg(
            &self,
            prefix: ::std::string::String,
            selector: ::std::string::String,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash(
                    [174, 199, 147, 97],
                    (prefix, selector, domain_name, public_key_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dkimRegistry` (0x6423f1e2) function
        pub fn dkim_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([100, 35, 241, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0xe7a7977a) function
        pub fn is_dkim_public_key_hash_valid(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([231, 167, 151, 122], (domain_name, public_key_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeDKIMPublicKeyHash` (0xf6b49344) function
        pub fn revoke_dkim_public_key_hash(
            &self,
            selector: ::std::string::String,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [246, 180, 147, 68],
                    (selector, domain_name, public_key_hash, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDKIMPublicKeyHash` (0x97170f2b) function
        pub fn set_dkim_public_key_hash(
            &self,
            selector: ::std::string::String,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [151, 23, 15, 43],
                    (selector, domain_name, public_key_hash, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signer` (0x238ac933) function
        pub fn signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([35, 138, 201, 51], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ECDSAOwnedDKIMRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
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
    #[ethcall(name = "REVOKE_PREFIX", abi = "REVOKE_PREFIX()")]
    pub struct RevokePrefixCall;
    ///Container type for all input parameters for the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
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
    #[ethcall(name = "SET_PREFIX", abi = "SET_PREFIX()")]
    pub struct SetPrefixCall;
    ///Container type for all input parameters for the `computeSignedMsg` function with signature `computeSignedMsg(string,string,string,bytes32)` and selector `0xaec79361`
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
        name = "computeSignedMsg",
        abi = "computeSignedMsg(string,string,string,bytes32)"
    )]
    pub struct ComputeSignedMsgCall {
        pub prefix: ::std::string::String,
        pub selector: ::std::string::String,
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `dkimRegistry` function with signature `dkimRegistry()` and selector `0x6423f1e2`
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
    #[ethcall(name = "dkimRegistry", abi = "dkimRegistry()")]
    pub struct DkimRegistryCall;
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
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
        abi = "isDKIMPublicKeyHashValid(string,bytes32)"
    )]
    pub struct IsDKIMPublicKeyHashValidCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `revokeDKIMPublicKeyHash` function with signature `revokeDKIMPublicKeyHash(string,string,bytes32,bytes)` and selector `0xf6b49344`
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
        name = "revokeDKIMPublicKeyHash",
        abi = "revokeDKIMPublicKeyHash(string,string,bytes32,bytes)"
    )]
    pub struct RevokeDKIMPublicKeyHashCall {
        pub selector: ::std::string::String,
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDKIMPublicKeyHash` function with signature `setDKIMPublicKeyHash(string,string,bytes32,bytes)` and selector `0x97170f2b`
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
        name = "setDKIMPublicKeyHash",
        abi = "setDKIMPublicKeyHash(string,string,bytes32,bytes)"
    )]
    pub struct SetDKIMPublicKeyHashCall {
        pub selector: ::std::string::String,
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `signer` function with signature `signer()` and selector `0x238ac933`
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
    #[ethcall(name = "signer", abi = "signer()")]
    pub struct SignerCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ECDSAOwnedDKIMRegistryCalls {
        RevokePrefix(RevokePrefixCall),
        SetPrefix(SetPrefixCall),
        ComputeSignedMsg(ComputeSignedMsgCall),
        DkimRegistry(DkimRegistryCall),
        IsDKIMPublicKeyHashValid(IsDKIMPublicKeyHashValidCall),
        RevokeDKIMPublicKeyHash(RevokeDKIMPublicKeyHashCall),
        SetDKIMPublicKeyHash(SetDKIMPublicKeyHashCall),
        Signer(SignerCall),
    }
    impl ::ethers::core::abi::AbiDecode for ECDSAOwnedDKIMRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RevokePrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePrefix(decoded));
            }
            if let Ok(decoded) = <SetPrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrefix(decoded));
            }
            if let Ok(decoded) = <ComputeSignedMsgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSignedMsg(decoded));
            }
            if let Ok(decoded) = <DkimRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DkimRegistry(decoded));
            }
            if let Ok(decoded) = <IsDKIMPublicKeyHashValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDKIMPublicKeyHashValid(decoded));
            }
            if let Ok(decoded) = <RevokeDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <SetDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <SignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Signer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ECDSAOwnedDKIMRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RevokePrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeSignedMsg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DkimRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Signer(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ECDSAOwnedDKIMRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RevokePrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeSignedMsg(element) => ::core::fmt::Display::fmt(element, f),
                Self::DkimRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Signer(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RevokePrefixCall> for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: RevokePrefixCall) -> Self {
            Self::RevokePrefix(value)
        }
    }
    impl ::core::convert::From<SetPrefixCall> for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: SetPrefixCall) -> Self {
            Self::SetPrefix(value)
        }
    }
    impl ::core::convert::From<ComputeSignedMsgCall> for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: ComputeSignedMsgCall) -> Self {
            Self::ComputeSignedMsg(value)
        }
    }
    impl ::core::convert::From<DkimRegistryCall> for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: DkimRegistryCall) -> Self {
            Self::DkimRegistry(value)
        }
    }
    impl ::core::convert::From<IsDKIMPublicKeyHashValidCall>
    for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: IsDKIMPublicKeyHashValidCall) -> Self {
            Self::IsDKIMPublicKeyHashValid(value)
        }
    }
    impl ::core::convert::From<RevokeDKIMPublicKeyHashCall>
    for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: RevokeDKIMPublicKeyHashCall) -> Self {
            Self::RevokeDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<SetDKIMPublicKeyHashCall>
    for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: SetDKIMPublicKeyHashCall) -> Self {
            Self::SetDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<SignerCall> for ECDSAOwnedDKIMRegistryCalls {
        fn from(value: SignerCall) -> Self {
            Self::Signer(value)
        }
    }
    ///Container type for all return fields from the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
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
    pub struct RevokePrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
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
    pub struct SetPrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `computeSignedMsg` function with signature `computeSignedMsg(string,string,string,bytes32)` and selector `0xaec79361`
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
    pub struct ComputeSignedMsgReturn(pub ::std::string::String);
    ///Container type for all return fields from the `dkimRegistry` function with signature `dkimRegistry()` and selector `0x6423f1e2`
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
    pub struct DkimRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
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
    ///Container type for all return fields from the `signer` function with signature `signer()` and selector `0x238ac933`
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
    pub struct SignerReturn(pub ::ethers::core::types::Address);
}
