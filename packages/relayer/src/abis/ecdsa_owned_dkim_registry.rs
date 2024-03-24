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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\"\x958\x03\x80b\0\"\x95\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xA3V[`@Qb\0\0B\x90b\0\0\x95V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0_W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90Ub\0\0\xD5V[a\t\x9B\x80b\0\x18\xFA\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\0\xB6W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xCEW`\0\x80\xFD[\x93\x92PPPV[a\x18\x15\x80b\0\0\xE5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xAE\xC7\x93a\x11a\0[W\x80c\xAE\xC7\x93a\x14a\x01YW\x80c\xD5\x07\xC3 \x14a\x01lW\x80c\xE7\xA7\x97z\x14a\x01\xA8W\x80c\xF6\xB4\x93D\x14a\x01\xCBW`\0\x80\xFD[\x80c\x07\xF1\xEA\xF5\x14a\0\x8DW\x80c#\x8A\xC93\x14a\0\xDFW\x80cd#\xF1\xE2\x14a\x01$W\x80c\x97\x17\x0F+\x14a\x01DW[`\0\x80\xFD[a\0\xC9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD6\x91\x90a\x12\xE3V[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xFF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD6V[`\0Ta\0\xFF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Wa\x01R6`\x04a\x13\xD9V[a\x01\xDEV[\0[a\0\xC9a\x01g6`\x04a\x14\x7FV[a\x06\x03V[a\0\xC9`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xBBa\x01\xB66`\x04a\x15\x0FV[a\x06=V[`@Q\x90\x15\x15\x81R` \x01a\0\xD6V[a\x01Wa\x01\xD96`\x04a\x13\xD9V[a\x06\xE0V[\x83Q`\0\x03a\x02NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82Q`\0\x03a\x02\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x81a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[a\x03*\x83\x83a\x06=V[\x15a\x03\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FpublicKeyHash is already set\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a\x15TV[\x15a\x04\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is revoked\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0a\x04\xCE`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x06\x03V[\x90P`\0a\x04\xDB\x82a\n\xC8V[\x90P`\0a\x04\xE9\x82\x85a\x0B\x03V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\x05pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7F\xC1\\\xFF\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x05\xC8\x90\x89\x90\x89\x90`\x04\x01a\x15vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xF6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x84\x84\x84a\x06\x11\x85a\x0B'V[`@Q` \x01a\x06$\x94\x93\x92\x91\x90a\x15\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x06\x96\x90\x86\x90\x86\x90`\x04\x01a\x15vV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD7\x91\x90a\x15TV[\x90P[\x92\x91PPV[\x83Q`\0\x03a\x07KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x82Q`\0\x03a\x07\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x81a\x08\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[a\x08'\x83\x83a\x06=V[\x15\x15`\x01\x14a\x08\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t%\x91\x90a\x15TV[\x15a\t\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x02EV[`\0a\t\xCF`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x06\x03V[\x90P`\0a\t\xDC\x82a\n\xC8V[\x90P`\0a\t\xEA\x82\x85a\x0B\x03V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\nqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7F\x15\xD2Q.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x05\xC8V[`\0a\n\xD4\x82Qa\x0B>V[\x82`@Q` \x01a\n\xE6\x92\x91\x90a\x16\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x0B\x12\x85\x85a\x0B\xFCV[\x91P\x91Pa\x0B\x1F\x81a\x0CAV[P\x93\x92PPPV[``a\x06\xDA\x82a\x0B6\x84a\r\xF7V[`\x01\x01a\x0EaV[```\0a\x0BK\x83a\x10\xA4V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BkWa\x0Bka\x12\xF6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\x9FWP\x93\x92PPPV[`\0\x80\x82Q`A\x03a\x0C2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x0C&\x87\x82\x85\x85a\x11\x86V[\x94P\x94PPPPa\x0C:V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x0CUWa\x0CUa\x16\xF3V[\x03a\x0C]WPV[`\x01\x81`\x04\x81\x11\x15a\x0CqWa\x0Cqa\x16\xF3V[\x03a\x0C\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\x02\x81`\x04\x81\x11\x15a\x0C\xECWa\x0C\xECa\x16\xF3V[\x03a\rSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02EV[`\x03\x81`\x04\x81\x11\x15a\rgWa\rga\x16\xF3V[\x03a\r\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02EV[PV[`\0\x80`\x80\x83\x90\x1C\x15a\x0E\x0FW`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\x0E$W`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\x0E9W` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\x0ENW`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x06\xDAW`\x01\x01\x92\x91PPV[```\0a\x0Ep\x83`\x02a\x17QV[a\x0E{\x90`\x02a\x17hV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x93Wa\x0E\x93a\x12\xF6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xBDW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x0E\xF4Wa\x0E\xF4a\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x0FWWa\x0FWa\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0F\x93\x84`\x02a\x17QV[a\x0F\x9E\x90`\x01a\x17hV[\x90P[`\x01\x81\x11\x15a\x10;W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x0F\xDFWa\x0F\xDFa\x17{V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xF5Wa\x0F\xF5a\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x104\x81a\x17\xAAV[\x90Pa\x0F\xA1V[P\x83\x15a\x06\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x02EV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x10\xEDWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x11\x19Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x117Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x11OWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x11cWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x11uW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06\xDAW`\x01\x01\x92\x91PPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x11\xBDWP`\0\x90P`\x03a\x12lV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12\x11W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x12eW`\0`\x01\x92P\x92PPa\x12lV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0[\x83\x81\x10\x15a\x12\x90W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12xV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\xB1\x81` \x86\x01` \x86\x01a\x12uV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06\xD7` \x83\x01\x84a\x12\x99V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x13@Wa\x13@a\x12\xF6V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x13\x86Wa\x13\x86a\x12\xF6V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x13\x9FW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x13\xCAW`\0\x80\xFD[a\x06\xD7\x83\x835` \x85\x01a\x13%V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\x07W`\0\x80\xFD[a\x14\x13\x88\x83\x89\x01a\x13\xB9V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x14)W`\0\x80\xFD[a\x145\x88\x83\x89\x01a\x13\xB9V[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x14RW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a\x14dW`\0\x80\xFD[a\x14s\x87\x825` \x84\x01a\x13%V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\x95W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xADW`\0\x80\xFD[a\x14\xB9\x88\x83\x89\x01a\x13\xB9V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x14\xCFW`\0\x80\xFD[a\x14\xDB\x88\x83\x89\x01a\x13\xB9V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x14\xF1W`\0\x80\xFD[Pa\x14\xFE\x87\x82\x88\x01a\x13\xB9V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\"W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x159W`\0\x80\xFD[a\x15E\x85\x82\x86\x01a\x13\xB9V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x15fW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xD7W`\0\x80\xFD[`@\x81R`\0a\x15\x89`@\x83\x01\x85a\x12\x99V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x85Qa\x15\xAA\x81\x84` \x8A\x01a\x12uV[\x7Fselector=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01\x90\x81R\x85Qa\x15\xE4\x81`\t\x84\x01` \x8A\x01a\x12uV[\x7F;domain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\t\x92\x90\x91\x01\x91\x82\x01R\x84Qa\x16\"\x81`\x11\x84\x01` \x89\x01a\x12uV[\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11\x92\x90\x91\x01\x91\x82\x01R\x83Qa\x16`\x81`\"\x84\x01` \x88\x01a\x12uV[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"\x92\x90\x91\x01\x91\x82\x01R`#\x01\x96\x95PPPPPPV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R`\0\x83Qa\x16\xD0\x81`\x1A\x85\x01` \x88\x01a\x12uV[\x83Q\x90\x83\x01\x90a\x16\xE7\x81`\x1A\x84\x01` \x88\x01a\x12uV[\x01`\x1A\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xDAWa\x06\xDAa\x17\"V[\x80\x82\x01\x80\x82\x11\x15a\x06\xDAWa\x06\xDAa\x17\"V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x17\xB9Wa\x17\xB9a\x17\"V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 \x9E\xD9f\xFA\xE6/\x95\x87\x91,\x85!\x0Fb\xD5\x1F\x8FLU#\xBF\xE2\xDDE\x81\xE19\xA7c\xBDw\xB7dsolcC\0\x08\x17\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\t\x1D\x80a\0~`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA3W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0vW\x80c\xE7\xA7\x97z\x11a\0[W\x80c\xE7\xA7\x97z\x14a\x01vW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x89W\x80c\xF4\x9E\xB1d\x14a\x01\x9CW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01;W\x80c\xC1\\\xFF\xAB\x14a\x01cW`\0\x80\xFD[\x80c\x06\x90\xBD8\x14a\0\xA8W\x80c\x15\xD2Q.\x14a\0\xFBW\x80cB\xD7\xCB\x98\x14a\x01\x10W\x80cqP\x18\xA6\x14a\x013W[`\0\x80\xFD[a\0\xE6a\0\xB66`\x04a\x06\xBAV[\x81Q` \x81\x84\x01\x81\x01\x80Q`\x01\x82R\x92\x82\x01\x94\x82\x01\x94\x90\x94 \x91\x90\x93R\x90\x91R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Ea\x01\t6`\x04a\x06\xFFV[a\x01\xAFV[\0[a\0\xE6a\x01\x1E6`\x04a\x06\xFFV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\x0Ea\x02+V[`\0T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xF2V[a\x01\x0Ea\x01q6`\x04a\x06\xBAV[a\x02?V[a\0\xE6a\x01\x846`\x04a\x06\xBAV[a\x03YV[a\x01\x0Ea\x01\x976`\x04a\x07\x18V[a\x03\xBDV[a\x01\x0Ea\x01\xAA6`\x04a\x07UV[a\x04tV[a\x01\xB7a\x04\xB8V[`\0\x81\x81R`\x02` R`@\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x7F\xB8\x0F\xFF+Lo=\xDF\x80Hw\x92|w\xB2\xFE\x18q\xCE\xCA\xA5\xADC\xD2\xC7\xC4/As1\xF8e\x90a\x02 \x90\x83\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[a\x023a\x04\xB8V[a\x02=`\0a\x059V[V[a\x02Ga\x04\xB8V[`\0\x81\x81R`\x02` R`@\x90 T`\xFF\x16\x15a\x02\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fcannot set revoked pubkey\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80\x83`@Qa\x02\xD6\x91\x90a\x08CV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 `\0\x86\x81R\x93R\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91U\x7FQ\r\xAC\x88\xEA\xF2\xDF\xBDS\x90BA\xFC\x19\x9A\xD4k c\xE6\xBFl?\xB2\x91\xF8\xCE\x86Cf4\x19\x90a\x03M\x90\x84\x90\x84\x90a\x08_V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81\x81R`\x02` R`@\x81 T`\xFF\x16\x15a\x03xWP`\0a\x03\xB7V[`\x01\x83`@Qa\x03\x88\x91\x90a\x08CV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x85\x81R\x92R\x90 T`\xFF\x16\x15a\x03\xB3WP`\x01a\x03\xB7V[P`\0[\x92\x91PPV[a\x03\xC5a\x04\xB8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04hW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xBCV[a\x04q\x81a\x059V[PV[a\x04|a\x04\xB8V[`\0[\x81Q\x81\x10\x15a\x04\xB3Wa\x04\xAB\x83\x83\x83\x81Q\x81\x10a\x04\x9EWa\x04\x9Ea\x08\xB8V[` \x02` \x01\x01Qa\x02?V[`\x01\x01a\x04\x7FV[PPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x02=W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xBCV[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06$Wa\x06$a\x05\xAEV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x06=W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06WWa\x06Wa\x05\xAEV[a\x06\x88` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05\xDDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x9DW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xCDW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE4W`\0\x80\xFD[a\x06\xF0\x85\x82\x86\x01a\x06,V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x07\x11W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07*W`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07NW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07hW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\x80W`\0\x80\xFD[a\x07\x8C\x86\x83\x87\x01a\x06,V[\x93P` \x91P\x81\x85\x015\x81\x81\x11\x15a\x07\xA3W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x07\xB4W`\0\x80\xFD[\x805\x82\x81\x11\x15a\x07\xC6Wa\x07\xC6a\x05\xAEV[\x80`\x05\x1B\x92Pa\x07\xD7\x84\x84\x01a\x05\xDDV[\x81\x81R\x92\x82\x01\x84\x01\x92\x84\x81\x01\x90\x89\x85\x11\x15a\x07\xF1W`\0\x80\xFD[\x92\x85\x01\x92[\x84\x84\x10\x15a\x08\x0FW\x835\x82R\x92\x85\x01\x92\x90\x85\x01\x90a\x07\xF6V[\x80\x96PPPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x08:W\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\"V[PP`\0\x91\x01RV[`\0\x82Qa\x08U\x81\x84` \x87\x01a\x08\x1FV[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x08~\x81``\x85\x01` \x88\x01a\x08\x1FV[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x98\xCBV\x9Bk\r\x1C\xD4\xA72\xD66\xF26\x07\xB7:\x88MA\x18\xCFF\x0C\xC0\xC8%\xCC:\xE9GZdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static ECDSAOWNEDDKIMREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xAE\xC7\x93a\x11a\0[W\x80c\xAE\xC7\x93a\x14a\x01YW\x80c\xD5\x07\xC3 \x14a\x01lW\x80c\xE7\xA7\x97z\x14a\x01\xA8W\x80c\xF6\xB4\x93D\x14a\x01\xCBW`\0\x80\xFD[\x80c\x07\xF1\xEA\xF5\x14a\0\x8DW\x80c#\x8A\xC93\x14a\0\xDFW\x80cd#\xF1\xE2\x14a\x01$W\x80c\x97\x17\x0F+\x14a\x01DW[`\0\x80\xFD[a\0\xC9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD6\x91\x90a\x12\xE3V[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xFF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD6V[`\0Ta\0\xFF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Wa\x01R6`\x04a\x13\xD9V[a\x01\xDEV[\0[a\0\xC9a\x01g6`\x04a\x14\x7FV[a\x06\x03V[a\0\xC9`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xBBa\x01\xB66`\x04a\x15\x0FV[a\x06=V[`@Q\x90\x15\x15\x81R` \x01a\0\xD6V[a\x01Wa\x01\xD96`\x04a\x13\xD9V[a\x06\xE0V[\x83Q`\0\x03a\x02NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82Q`\0\x03a\x02\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x81a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[a\x03*\x83\x83a\x06=V[\x15a\x03\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FpublicKeyHash is already set\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a\x15TV[\x15a\x04\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is revoked\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0a\x04\xCE`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x06\x03V[\x90P`\0a\x04\xDB\x82a\n\xC8V[\x90P`\0a\x04\xE9\x82\x85a\x0B\x03V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\x05pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7F\xC1\\\xFF\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x05\xC8\x90\x89\x90\x89\x90`\x04\x01a\x15vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xF6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x84\x84\x84a\x06\x11\x85a\x0B'V[`@Q` \x01a\x06$\x94\x93\x92\x91\x90a\x15\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[`\0\x80T`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x06\x96\x90\x86\x90\x86\x90`\x04\x01a\x15vV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD7\x91\x90a\x15TV[\x90P[\x92\x91PPV[\x83Q`\0\x03a\x07KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x82Q`\0\x03a\x07\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[\x81a\x08\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[a\x08'\x83\x83a\x06=V[\x15\x15`\x01\x14a\x08\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t%\x91\x90a\x15TV[\x15a\t\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x02EV[`\0a\t\xCF`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x06\x03V[\x90P`\0a\t\xDC\x82a\n\xC8V[\x90P`\0a\t\xEA\x82\x85a\x0B\x03V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\nqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\0T`@Q\x7F\x15\xD2Q.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x05\xC8V[`\0a\n\xD4\x82Qa\x0B>V[\x82`@Q` \x01a\n\xE6\x92\x91\x90a\x16\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x0B\x12\x85\x85a\x0B\xFCV[\x91P\x91Pa\x0B\x1F\x81a\x0CAV[P\x93\x92PPPV[``a\x06\xDA\x82a\x0B6\x84a\r\xF7V[`\x01\x01a\x0EaV[```\0a\x0BK\x83a\x10\xA4V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BkWa\x0Bka\x12\xF6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\x9FWP\x93\x92PPPV[`\0\x80\x82Q`A\x03a\x0C2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x0C&\x87\x82\x85\x85a\x11\x86V[\x94P\x94PPPPa\x0C:V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x0CUWa\x0CUa\x16\xF3V[\x03a\x0C]WPV[`\x01\x81`\x04\x81\x11\x15a\x0CqWa\x0Cqa\x16\xF3V[\x03a\x0C\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02EV[`\x02\x81`\x04\x81\x11\x15a\x0C\xECWa\x0C\xECa\x16\xF3V[\x03a\rSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02EV[`\x03\x81`\x04\x81\x11\x15a\rgWa\rga\x16\xF3V[\x03a\r\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02EV[PV[`\0\x80`\x80\x83\x90\x1C\x15a\x0E\x0FW`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\x0E$W`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\x0E9W` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\x0ENW`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x06\xDAW`\x01\x01\x92\x91PPV[```\0a\x0Ep\x83`\x02a\x17QV[a\x0E{\x90`\x02a\x17hV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x93Wa\x0E\x93a\x12\xF6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xBDW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x0E\xF4Wa\x0E\xF4a\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x0FWWa\x0FWa\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0F\x93\x84`\x02a\x17QV[a\x0F\x9E\x90`\x01a\x17hV[\x90P[`\x01\x81\x11\x15a\x10;W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x0F\xDFWa\x0F\xDFa\x17{V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xF5Wa\x0F\xF5a\x17{V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x104\x81a\x17\xAAV[\x90Pa\x0F\xA1V[P\x83\x15a\x06\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x02EV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x10\xEDWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x11\x19Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x117Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x11OWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x11cWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x11uW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06\xDAW`\x01\x01\x92\x91PPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x11\xBDWP`\0\x90P`\x03a\x12lV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12\x11W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x12eW`\0`\x01\x92P\x92PPa\x12lV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0[\x83\x81\x10\x15a\x12\x90W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12xV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\xB1\x81` \x86\x01` \x86\x01a\x12uV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06\xD7` \x83\x01\x84a\x12\x99V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x13@Wa\x13@a\x12\xF6V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x13\x86Wa\x13\x86a\x12\xF6V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x13\x9FW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x13\xCAW`\0\x80\xFD[a\x06\xD7\x83\x835` \x85\x01a\x13%V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\x07W`\0\x80\xFD[a\x14\x13\x88\x83\x89\x01a\x13\xB9V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x14)W`\0\x80\xFD[a\x145\x88\x83\x89\x01a\x13\xB9V[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x14RW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a\x14dW`\0\x80\xFD[a\x14s\x87\x825` \x84\x01a\x13%V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\x95W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xADW`\0\x80\xFD[a\x14\xB9\x88\x83\x89\x01a\x13\xB9V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x14\xCFW`\0\x80\xFD[a\x14\xDB\x88\x83\x89\x01a\x13\xB9V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x14\xF1W`\0\x80\xFD[Pa\x14\xFE\x87\x82\x88\x01a\x13\xB9V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\"W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x159W`\0\x80\xFD[a\x15E\x85\x82\x86\x01a\x13\xB9V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x15fW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xD7W`\0\x80\xFD[`@\x81R`\0a\x15\x89`@\x83\x01\x85a\x12\x99V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x85Qa\x15\xAA\x81\x84` \x8A\x01a\x12uV[\x7Fselector=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01\x90\x81R\x85Qa\x15\xE4\x81`\t\x84\x01` \x8A\x01a\x12uV[\x7F;domain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\t\x92\x90\x91\x01\x91\x82\x01R\x84Qa\x16\"\x81`\x11\x84\x01` \x89\x01a\x12uV[\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11\x92\x90\x91\x01\x91\x82\x01R\x83Qa\x16`\x81`\"\x84\x01` \x88\x01a\x12uV[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"\x92\x90\x91\x01\x91\x82\x01R`#\x01\x96\x95PPPPPPV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R`\0\x83Qa\x16\xD0\x81`\x1A\x85\x01` \x88\x01a\x12uV[\x83Q\x90\x83\x01\x90a\x16\xE7\x81`\x1A\x84\x01` \x88\x01a\x12uV[\x01`\x1A\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xDAWa\x06\xDAa\x17\"V[\x80\x82\x01\x80\x82\x11\x15a\x06\xDAWa\x06\xDAa\x17\"V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x17\xB9Wa\x17\xB9a\x17\"V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 \x9E\xD9f\xFA\xE6/\x95\x87\x91,\x85!\x0Fb\xD5\x1F\x8FLU#\xBF\xE2\xDDE\x81\xE19\xA7c\xBDw\xB7dsolcC\0\x08\x17\x003";
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
