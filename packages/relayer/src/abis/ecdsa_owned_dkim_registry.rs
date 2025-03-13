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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa!\x9E8\x03\x80a!\x9E\x839\x81\x01`@\x81\x90R`+\x91`\x8DV[`@Q`5\x90`\x80V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15`MW=__>=_\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`\xB8V[a\tV\x80a\x18H\x839\x01\x90V[_` \x82\x84\x03\x12\x15`\x9CW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xB1W__\xFD[\x93\x92PPPV[a\x17\x83\x80a\0\xC5_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80c\xAE\xC7\x93a\x11a\0XW\x80c\xAE\xC7\x93a\x14a\x01TW\x80c\xD5\x07\xC3 \x14a\x01gW\x80c\xE7\xA7\x97z\x14a\x01\xA3W\x80c\xF6\xB4\x93D\x14a\x01\xC6W__\xFD[\x80c\x07\xF1\xEA\xF5\x14a\0\x89W\x80c#\x8A\xC93\x14a\0\xDBW\x80cd#\xF1\xE2\x14a\x01 W\x80c\x97\x17\x0F+\x14a\x01?W[__\xFD[a\0\xC5`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD2\x91\x90a\x12\x8AV[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xFB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD2V[_Ta\0\xFB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Ra\x01M6`\x04a\x13|V[a\x01\xD9V[\0[a\0\xC5a\x01b6`\x04a\x14'V[a\x05\xF0V[a\0\xC5`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xB6a\x01\xB16`\x04a\x14\xBDV[a\x06*V[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\x01Ra\x01\xD46`\x04a\x13|V[a\x06\xCAV[\x83Q_\x03a\x02HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82Q_\x03a\x02\xB2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x81a\x03\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[a\x03#\x83\x83a\x06*V[\x15a\x03\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FpublicKeyHash is already set\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1A\x91\x90a\x14\xFFV[\x15a\x04\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is revoked\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_a\x04\xC3`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x05\xF0V[\x90P_a\x04\xCF\x82a\n\xA9V[\x90P_a\x04\xDC\x82\x85a\n\xE3V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\x05cW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7F\xC1\\\xFF\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x05\xBA\x90\x89\x90\x89\x90`\x04\x01a\x15\x1EV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xD1W__\xFD[PZ\xF1\x15\x80\x15a\x05\xE3W=__>=_\xFD[PPPPPPPPPPPV[``\x84\x84\x84a\x05\xFE\x85a\x0B\x05V[`@Q` \x01a\x06\x11\x94\x93\x92\x91\x90a\x15VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[_\x80T`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x06\x82\x90\x86\x90\x86\x90`\x04\x01a\x15\x1EV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a\x14\xFFV[\x90P[\x92\x91PPV[\x83Q_\x03a\x074W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x82Q_\x03a\x07\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x81a\x08\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[a\x08\x0F\x83\x83a\x06*V[\x15\x15`\x01\x14a\x08zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\n\x91\x90a\x14\xFFV[\x15a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x02?V[_a\t\xB3`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x05\xF0V[\x90P_a\t\xBF\x82a\n\xA9V[\x90P_a\t\xCC\x82\x85a\n\xE3V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\nSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7F\x15\xD2Q.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x05\xBAV[_a\n\xB4\x82Qa\x0B\x1CV[\x82`@Q` \x01a\n\xC6\x92\x91\x90a\x16&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[___a\n\xF0\x85\x85a\x0B\xD8V[\x91P\x91Pa\n\xFD\x81a\x0C\x1AV[P\x93\x92PPPV[``a\x06\xC4\x82a\x0B\x14\x84a\r\xCFV[`\x01\x01a\x0E8V[``_a\x0B(\x83a\x10uV[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BGWa\x0BGa\x12\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0BqW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B{WP\x93\x92PPPV[__\x82Q`A\x03a\x0C\x0CW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x0C\0\x87\x82\x85\x85a\x11VV[\x94P\x94PPPPa\x0C\x13V[P_\x90P`\x02[\x92P\x92\x90PV[_\x81`\x04\x81\x11\x15a\x0C-Wa\x0C-a\x16hV[\x03a\x0C5WPV[`\x01\x81`\x04\x81\x11\x15a\x0CIWa\x0CIa\x16hV[\x03a\x0C\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[`\x02\x81`\x04\x81\x11\x15a\x0C\xC4Wa\x0C\xC4a\x16hV[\x03a\r+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02?V[`\x03\x81`\x04\x81\x11\x15a\r?Wa\r?a\x16hV[\x03a\r\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02?V[PV[_\x80`\x80\x83\x90\x1C\x15a\r\xE6W`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\r\xFBW`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\x0E\x10W` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\x0E%W`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x06\xC4W`\x01\x01\x92\x91PPV[``_a\x0EF\x83`\x02a\x16\xC2V[a\x0EQ\x90`\x02a\x16\xD9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EiWa\x0Eia\x12\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x93W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x0E\xC9Wa\x0E\xC9a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x0F+Wa\x0F+a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP_a\x0Fe\x84`\x02a\x16\xC2V[a\x0Fp\x90`\x01a\x16\xD9V[\x90P[`\x01\x81\x11\x15a\x10\x0CW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x0F\xB1Wa\x0F\xB1a\x16\xECV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xC7Wa\x0F\xC7a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x10\x05\x81a\x17\x19V[\x90Pa\x0FsV[P\x83\x15a\x06\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x02?V[_\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x10\xBDWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x10\xE9Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x11\x07Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x11\x1FWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x113Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x11EW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06\xC4W`\x01\x01\x92\x91PPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x11\x8BWP_\x90P`\x03a\x125V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\xDCW=__>=_\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x12/W_`\x01\x92P\x92PPa\x125V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x06\xC1` \x83\x01\x84a\x12>V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x12\xE3Wa\x12\xE3a\x12\x9CV[P`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x130Wa\x130a\x12\x9CV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x13GW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13mW__\xFD[a\x06\xC1\x83\x835` \x85\x01a\x12\xC9V[____`\x80\x85\x87\x03\x12\x15a\x13\x8FW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xA5W__\xFD[a\x13\xB1\x87\x82\x88\x01a\x13^V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xCDW__\xFD[a\x13\xD9\x87\x82\x88\x01a\x13^V[\x93PP`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xFCW__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x14\x0CW__\xFD[a\x14\x1B\x87\x825` \x84\x01a\x12\xC9V[\x91PP\x92\x95\x91\x94P\x92PV[____`\x80\x85\x87\x03\x12\x15a\x14:W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14PW__\xFD[a\x14\\\x87\x82\x88\x01a\x13^V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14xW__\xFD[a\x14\x84\x87\x82\x88\x01a\x13^V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xA0W__\xFD[a\x14\xAC\x87\x82\x88\x01a\x13^V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x14\xCEW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE4W__\xFD[a\x14\xF0\x85\x82\x86\x01a\x13^V[\x95` \x94\x90\x94\x015\x94PPPPV[_` \x82\x84\x03\x12\x15a\x15\x0FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xC1W__\xFD[`@\x81R_a\x150`@\x83\x01\x85a\x12>V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x15a\x82\x87a\x15?V[\x7Fselector=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\x91`\t\x82\x01\x87a\x15?V[\x90P\x7F;domain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\xC3`\x08\x82\x01\x86a\x15?V[\x90P\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\xF5`\x11\x82\x01\x85a\x15?V[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x01\x97\x96PPPPPPPV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R_a\x16`a\x16Z`\x1A\x84\x01\x86a\x15?V[\x84a\x15?V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xC4Wa\x06\xC4a\x16\x95V[\x80\x82\x01\x80\x82\x11\x15a\x06\xC4Wa\x06\xC4a\x16\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81a\x17'Wa\x17'a\x16\x95V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 \xB5e#\xE8\xC56\xFF\xE0\x8F\xA3<E\xB9\x88\x11:\xE1\xF4$7\xE9\xE7`\x12#\xB8I\x90\xD5\x9D\xA0\xE8dsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x163`\x1AV[`iV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x08\xE0\x80a\0v_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9FW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0rW\x80c\xE7\xA7\x97z\x11a\0XW\x80c\xE7\xA7\x97z\x14a\x01nW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x81W\x80c\xF4\x9E\xB1d\x14a\x01\x94W__\xFD[\x80c\x8D\xA5\xCB[\x14a\x014W\x80c\xC1\\\xFF\xAB\x14a\x01[W__\xFD[\x80c\x06\x90\xBD8\x14a\0\xA3W\x80c\x15\xD2Q.\x14a\0\xF5W\x80cB\xD7\xCB\x98\x14a\x01\nW\x80cqP\x18\xA6\x14a\x01,W[__\xFD[a\0\xE0a\0\xB16`\x04a\x06\xA1V[\x81Q` \x81\x84\x01\x81\x01\x80Q`\x01\x82R\x92\x82\x01\x94\x82\x01\x94\x90\x94 \x91\x90\x93R\x90\x91R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x08a\x01\x036`\x04a\x06\xE3V[a\x01\xA7V[\0[a\0\xE0a\x01\x186`\x04a\x06\xE3V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\x08a\x02\"V[_T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xECV[a\x01\x08a\x01i6`\x04a\x06\xA1V[a\x025V[a\0\xE0a\x01|6`\x04a\x06\xA1V[a\x03MV[a\x01\x08a\x01\x8F6`\x04a\x06\xFAV[a\x03\xADV[a\x01\x08a\x01\xA26`\x04a\x074V[a\x04dV[a\x01\xAFa\x04\xA7V[_\x81\x81R`\x02` R`@\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x7F\xB8\x0F\xFF+Lo=\xDF\x80Hw\x92|w\xB2\xFE\x18q\xCE\xCA\xA5\xADC\xD2\xC7\xC4/As1\xF8e\x90a\x02\x17\x90\x83\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[a\x02*a\x04\xA7V[a\x023_a\x05'V[V[a\x02=a\x04\xA7V[_\x81\x81R`\x02` R`@\x90 T`\xFF\x16\x15a\x02\xBAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fcannot set revoked pubkey\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80\x83`@Qa\x02\xCB\x91\x90a\x08\rV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 _\x86\x81R\x93R\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91U\x7FQ\r\xAC\x88\xEA\xF2\xDF\xBDS\x90BA\xFC\x19\x9A\xD4k c\xE6\xBFl?\xB2\x91\xF8\xCE\x86Cf4\x19\x90a\x03A\x90\x84\x90\x84\x90a\x08#V[`@Q\x80\x91\x03\x90\xA1PPV[_\x81\x81R`\x02` R`@\x81 T`\xFF\x16\x15a\x03jWP_a\x03\xA7V[`\x01\x83`@Qa\x03z\x91\x90a\x08\rV[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 _\x85\x81R\x92R\x90 T`\xFF\x16\x15a\x03\xA4WP`\x01a\x03\xA7V[P_[\x92\x91PPV[a\x03\xB5a\x04\xA7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04XW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xB1V[a\x04a\x81a\x05'V[PV[a\x04la\x04\xA7V[_[\x81Q\x81\x10\x15a\x04\xA2Wa\x04\x9A\x83\x83\x83\x81Q\x81\x10a\x04\x8DWa\x04\x8Da\x08}V[` \x02` \x01\x01Qa\x025V[`\x01\x01a\x04nV[PPPV[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x023W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xB1V[_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x0FWa\x06\x0Fa\x05\x9BV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x06&W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06@Wa\x06@a\x05\x9BV[a\x06q` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05\xC8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x85W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x06\xB2W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC8W__\xFD[a\x06\xD4\x85\x82\x86\x01a\x06\x17V[\x95` \x94\x90\x94\x015\x94PPPPV[_` \x82\x84\x03\x12\x15a\x06\xF3W__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x07\nW__\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07-W__\xFD[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x07EW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07[W__\xFD[a\x07g\x85\x82\x86\x01a\x06\x17V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x83W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x07\x93W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xADWa\x07\xADa\x05\x9BV[\x80`\x05\x1Ba\x07\xBD` \x82\x01a\x05\xC8V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x88\x84\x11\x15a\x07\xD8W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x07\xFEW\x845\x80\x83R` \x95\x86\x01\x95\x90\x93P\x90\x91\x01\x90a\x07\xDFV[\x80\x95PPPPPP\x92P\x92\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x01R``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x97V53\xD6S\xBE\xE2L\xEB\x82\xBE*\xF2\xCA\x82\xDD\xE5~\xB7p\x87{P?\x07\x97\xC2u \"\xE8dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static ECDSAOWNEDDKIMREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80c\xAE\xC7\x93a\x11a\0XW\x80c\xAE\xC7\x93a\x14a\x01TW\x80c\xD5\x07\xC3 \x14a\x01gW\x80c\xE7\xA7\x97z\x14a\x01\xA3W\x80c\xF6\xB4\x93D\x14a\x01\xC6W__\xFD[\x80c\x07\xF1\xEA\xF5\x14a\0\x89W\x80c#\x8A\xC93\x14a\0\xDBW\x80cd#\xF1\xE2\x14a\x01 W\x80c\x97\x17\x0F+\x14a\x01?W[__\xFD[a\0\xC5`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD2\x91\x90a\x12\x8AV[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xFB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD2V[_Ta\0\xFB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Ra\x01M6`\x04a\x13|V[a\x01\xD9V[\0[a\0\xC5a\x01b6`\x04a\x14'V[a\x05\xF0V[a\0\xC5`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xB6a\x01\xB16`\x04a\x14\xBDV[a\x06*V[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\x01Ra\x01\xD46`\x04a\x13|V[a\x06\xCAV[\x83Q_\x03a\x02HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82Q_\x03a\x02\xB2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x81a\x03\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[a\x03#\x83\x83a\x06*V[\x15a\x03\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FpublicKeyHash is already set\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1A\x91\x90a\x14\xFFV[\x15a\x04\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is revoked\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_a\x04\xC3`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x05\xF0V[\x90P_a\x04\xCF\x82a\n\xA9V[\x90P_a\x04\xDC\x82\x85a\n\xE3V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\x05cW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7F\xC1\\\xFF\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC1\\\xFF\xAB\x90a\x05\xBA\x90\x89\x90\x89\x90`\x04\x01a\x15\x1EV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xD1W__\xFD[PZ\xF1\x15\x80\x15a\x05\xE3W=__>=_\xFD[PPPPPPPPPPPV[``\x84\x84\x84a\x05\xFE\x85a\x0B\x05V[`@Q` \x01a\x06\x11\x94\x93\x92\x91\x90a\x15VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[_\x80T`@Q\x7F\xE7\xA7\x97z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE7\xA7\x97z\x90a\x06\x82\x90\x86\x90\x86\x90`\x04\x01a\x15\x1EV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a\x14\xFFV[\x90P[\x92\x91PPV[\x83Q_\x03a\x074W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FInvalid selector\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x82Q_\x03a\x07\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FInvalid domain name\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[\x81a\x08\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid public key hash\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[a\x08\x0F\x83\x83a\x06*V[\x15\x15`\x01\x14a\x08zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FpublicKeyHash is not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7FB\xD7\xCB\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cB\xD7\xCB\x98\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\n\x91\x90a\x14\xFFV[\x15a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FpublicKeyHash is already revoked`D\x82\x01R`d\x01a\x02?V[_a\t\xB3`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86\x86a\x05\xF0V[\x90P_a\t\xBF\x82a\n\xA9V[\x90P_a\t\xCC\x82\x85a\n\xE3V[`\x01T\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x14a\nSW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[_T`@Q\x7F\x15\xD2Q.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x87\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x15\xD2Q.\x90`$\x01a\x05\xBAV[_a\n\xB4\x82Qa\x0B\x1CV[\x82`@Q` \x01a\n\xC6\x92\x91\x90a\x16&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[___a\n\xF0\x85\x85a\x0B\xD8V[\x91P\x91Pa\n\xFD\x81a\x0C\x1AV[P\x93\x92PPPV[``a\x06\xC4\x82a\x0B\x14\x84a\r\xCFV[`\x01\x01a\x0E8V[``_a\x0B(\x83a\x10uV[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BGWa\x0BGa\x12\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0BqW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B{WP\x93\x92PPPV[__\x82Q`A\x03a\x0C\x0CW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x0C\0\x87\x82\x85\x85a\x11VV[\x94P\x94PPPPa\x0C\x13V[P_\x90P`\x02[\x92P\x92\x90PV[_\x81`\x04\x81\x11\x15a\x0C-Wa\x0C-a\x16hV[\x03a\x0C5WPV[`\x01\x81`\x04\x81\x11\x15a\x0CIWa\x0CIa\x16hV[\x03a\x0C\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02?V[`\x02\x81`\x04\x81\x11\x15a\x0C\xC4Wa\x0C\xC4a\x16hV[\x03a\r+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02?V[`\x03\x81`\x04\x81\x11\x15a\r?Wa\r?a\x16hV[\x03a\r\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02?V[PV[_\x80`\x80\x83\x90\x1C\x15a\r\xE6W`\x80\x92\x90\x92\x1C\x91`\x10\x01[`@\x83\x90\x1C\x15a\r\xFBW`@\x92\x90\x92\x1C\x91`\x08\x01[` \x83\x90\x1C\x15a\x0E\x10W` \x92\x90\x92\x1C\x91`\x04\x01[`\x10\x83\x90\x1C\x15a\x0E%W`\x10\x92\x90\x92\x1C\x91`\x02\x01[`\x08\x83\x90\x1C\x15a\x06\xC4W`\x01\x01\x92\x91PPV[``_a\x0EF\x83`\x02a\x16\xC2V[a\x0EQ\x90`\x02a\x16\xD9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EiWa\x0Eia\x12\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x93W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x0E\xC9Wa\x0E\xC9a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x0F+Wa\x0F+a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP_a\x0Fe\x84`\x02a\x16\xC2V[a\x0Fp\x90`\x01a\x16\xD9V[\x90P[`\x01\x81\x11\x15a\x10\x0CW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x0F\xB1Wa\x0F\xB1a\x16\xECV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xC7Wa\x0F\xC7a\x16\xECV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x10\x05\x81a\x17\x19V[\x90Pa\x0FsV[P\x83\x15a\x06\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x02?V[_\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x10\xBDWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x10\xE9Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x11\x07Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x11\x1FWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x113Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x11EW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06\xC4W`\x01\x01\x92\x91PPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x11\x8BWP_\x90P`\x03a\x125V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\xDCW=__>=_\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x12/W_`\x01\x92P\x92PPa\x125V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x06\xC1` \x83\x01\x84a\x12>V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x12\xE3Wa\x12\xE3a\x12\x9CV[P`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x130Wa\x130a\x12\x9CV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x13GW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13mW__\xFD[a\x06\xC1\x83\x835` \x85\x01a\x12\xC9V[____`\x80\x85\x87\x03\x12\x15a\x13\x8FW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xA5W__\xFD[a\x13\xB1\x87\x82\x88\x01a\x13^V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xCDW__\xFD[a\x13\xD9\x87\x82\x88\x01a\x13^V[\x93PP`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xFCW__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x14\x0CW__\xFD[a\x14\x1B\x87\x825` \x84\x01a\x12\xC9V[\x91PP\x92\x95\x91\x94P\x92PV[____`\x80\x85\x87\x03\x12\x15a\x14:W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14PW__\xFD[a\x14\\\x87\x82\x88\x01a\x13^V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14xW__\xFD[a\x14\x84\x87\x82\x88\x01a\x13^V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xA0W__\xFD[a\x14\xAC\x87\x82\x88\x01a\x13^V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x14\xCEW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE4W__\xFD[a\x14\xF0\x85\x82\x86\x01a\x13^V[\x95` \x94\x90\x94\x015\x94PPPPV[_` \x82\x84\x03\x12\x15a\x15\x0FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xC1W__\xFD[`@\x81R_a\x150`@\x83\x01\x85a\x12>V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x15a\x82\x87a\x15?V[\x7Fselector=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\x91`\t\x82\x01\x87a\x15?V[\x90P\x7F;domain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\xC3`\x08\x82\x01\x86a\x15?V[\x90P\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x15\xF5`\x11\x82\x01\x85a\x15?V[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x01\x97\x96PPPPPPPV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R_a\x16`a\x16Z`\x1A\x84\x01\x86a\x15?V[\x84a\x15?V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xC4Wa\x06\xC4a\x16\x95V[\x80\x82\x01\x80\x82\x11\x15a\x06\xC4Wa\x06\xC4a\x16\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81a\x17'Wa\x17'a\x16\x95V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 \xB5e#\xE8\xC56\xFF\xE0\x8F\xA3<E\xB9\x88\x11:\xE1\xF4$7\xE9\xE7`\x12#\xB8I\x90\xD5\x9D\xA0\xE8dsolcC\0\x08\x1C\x003";
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
