pub use relayer_handler::*;
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
pub mod relayer_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getRandHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRandHash"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("relayer"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("registerRelayer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerRelayer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("randHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("emailAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hostname"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relayerOfEmailAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("relayerOfEmailAddr"),
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
                    ::std::borrow::ToOwned::to_owned("relayerOfRandHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("relayerOfRandHash"),
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
                    ::std::borrow::ToOwned::to_owned("relayers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("relayers"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("randHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("emailAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hostname"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateRelayerConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateRelayerConfig",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("hostname"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RelayerConfigUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RelayerConfigUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("addr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hostname"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayerRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RelayerRegistered"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("addr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("randHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("emailAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hostname"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
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
    pub static RELAYERHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0Ck\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c?V\x15\xA4\x14a\0gW\x80cS\0\xF8A\x14a\0\xA3W\x80c]#9,\x14a\0\xC5W\x80c~/+\xBC\x14a\0\xDAW\x80c\x82\x83\xB5u\x14a\0\xEDW\x80c\xCD\t,\x1A\x14a\x01.W[`\0\x80\xFD[a\0\x90a\0u6`\x04a\x06\xD8V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xB16`\x04a\x06\xD8V[a\x01bV[`@Qa\0\x9A\x93\x92\x91\x90a\x07NV[a\0\xD8a\0\xD36`\x04a\x07\xCBV[a\x02\x95V[\0[a\0\xD8a\0\xE86`\x04a\x08DV[a\x05\xFDV[a\x01\x16a\0\xFB6`\x04a\x08\x85V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9AV[a\x01\x16a\x01<6`\x04a\x08\xB4V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x01\x84\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xB0\x90a\tdV[\x80\x15a\x01\xFDW\x80`\x1F\x10a\x01\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x02\x12\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02>\x90a\tdV[\x80\x15a\x02\x8BW\x80`\x1F\x10a\x02`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x8BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[\x84a\x02\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwrandHash cannot be empty`@\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83\x90\x03a\x03/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxemailAddr cannot be empty`8\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0\x81\x90\x03a\x03PW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD9\x90a\t\x9EV[3`\0\x90\x81R` \x81\x90R`@\x90 T\x15a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x1C\x99[\x18^Y\\\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0\x85\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x1C\x98[\x99\x12\x18\\\xDA\x08\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`*\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02\x85\x85`@Qa\x04*\x92\x91\x90a\t\xD0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\x19[XZ[\x10Y\x19\x1C\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\"\x1B`D\x82\x01R`d\x01a\x02\xD9V[`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x91\x81\x01\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP3\x81R` \x81\x81R`@\x90\x91 \x83Q\x81U\x90\x83\x01Q\x90\x91P`\x01\x82\x01\x90a\x05:\x90\x82a\nDV[P`@\x82\x01Q`\x02\x82\x01\x90a\x05O\x90\x82a\nDV[PPP`\0\x85\x81R`\x01` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x90Q`\x02\x90a\x05\x87\x90\x87\x90\x87\x90a\t\xD0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U3\x90\x7F\x07\xD4\x89J\xB9\xBDd\xE8\xAB\xA0\x8Ay\xDCA\x88\x0B7\xAF\x16\xC7\\\x8E\xD7>v\x1D\xA6R<4\xE3\xFA\x90a\x05\xEE\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90a\x0B&V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x81\x90\x03a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD9\x90a\t\x9EV[3`\0\x90\x81R` \x81\x90R`@\x90 Ta\x06sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`D\x82\x01R`d\x01a\x02\xD9V[3`\0\x90\x81R` \x81\x90R`@\x90 `\x02\x01a\x06\x90\x82\x84\x83a\x0B_V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7Fo\xF0Tl\x9B\x98H\x13\x86.\xE7\xFDL\x0B\xCC\xB1\xD8<\x1DVp\xF9S\xF0\\\xA4\xBA\xE5d\x18\xD5\xEE\x83\x83`@Qa\x06\xCC\x92\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xA2PPV[`\0` \x82\x84\x03\x12\x15a\x06\xEAW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x01W`\0\x80\xFD[\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x07.W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07\x12V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R``` \x82\x01R`\0a\x07g``\x83\x01\x85a\x07\x08V[\x82\x81\x03`@\x84\x01Ra\x07y\x81\x85a\x07\x08V[\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x95W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xACW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x07\xC4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x07\xE3W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\x01W`\0\x80\xFD[a\x08\r\x89\x83\x8A\x01a\x07\x83V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x08&W`\0\x80\xFD[Pa\x083\x88\x82\x89\x01a\x07\x83V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x08WW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08mW`\0\x80\xFD[a\x08y\x85\x82\x86\x01a\x07\x83V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x97W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\xDDW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x08\xF1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\t\x03Wa\t\x03a\x08\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t+Wa\t+a\x08\x9EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\tDW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\txW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x98WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01Rwhostname cannot be empty`@\x1B`@\x82\x01R``\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\n*W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\n\x07WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\n&W\x82\x81U`\x01\x01a\n\x13V[PPP[PPPV[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n]Wa\n]a\x08\x9EV[a\nq\x81a\nk\x84Ta\tdV[\x84a\t\xE0V[` \x80`\x1F\x83\x11`\x01\x81\x14a\n\xA0W`\0\x84\x15a\n\x8EWP\x85\x83\x01Q[a\n\x98\x85\x82a\n/V[\x86UPa\n&V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\n\xCFW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\n\xB0V[P\x85\x82\x10\x15a\n\xEDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0a\x0B@``\x83\x01\x86\x88a\n\xFDV[\x82\x81\x03`@\x84\x01Ra\x0BS\x81\x85\x87a\n\xFDV[\x98\x97PPPPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x0BvWa\x0Bva\x08\x9EV[a\x0B\x8A\x83a\x0B\x84\x83Ta\tdV[\x83a\t\xE0V[`\0`\x1F\x84\x11`\x01\x81\x14a\x0B\xB8W`\0\x85\x15a\x0B\xA6WP\x83\x82\x015[a\x0B\xB0\x86\x82a\n/V[\x84UPa\x0C\x12V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0B\xE9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0B\xC9V[P\x86\x82\x10\x15a\x0C\x06W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[` \x81R`\0a\x0C-` \x83\x01\x84\x86a\n\xFDV[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 =\xD3\xB5\xAAy'\xC4\xE1\x11\xB61\xE7\xCA\x0B\xEEl\xE5\x1F`}\xF5*a\x96$\xD8D\x07\x11\\\xDD?dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static RELAYERHANDLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c?V\x15\xA4\x14a\0gW\x80cS\0\xF8A\x14a\0\xA3W\x80c]#9,\x14a\0\xC5W\x80c~/+\xBC\x14a\0\xDAW\x80c\x82\x83\xB5u\x14a\0\xEDW\x80c\xCD\t,\x1A\x14a\x01.W[`\0\x80\xFD[a\0\x90a\0u6`\x04a\x06\xD8V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xB16`\x04a\x06\xD8V[a\x01bV[`@Qa\0\x9A\x93\x92\x91\x90a\x07NV[a\0\xD8a\0\xD36`\x04a\x07\xCBV[a\x02\x95V[\0[a\0\xD8a\0\xE86`\x04a\x08DV[a\x05\xFDV[a\x01\x16a\0\xFB6`\x04a\x08\x85V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9AV[a\x01\x16a\x01<6`\x04a\x08\xB4V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x01\x84\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xB0\x90a\tdV[\x80\x15a\x01\xFDW\x80`\x1F\x10a\x01\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x02\x12\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02>\x90a\tdV[\x80\x15a\x02\x8BW\x80`\x1F\x10a\x02`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x8BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[\x84a\x02\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwrandHash cannot be empty`@\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83\x90\x03a\x03/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxemailAddr cannot be empty`8\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0\x81\x90\x03a\x03PW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD9\x90a\t\x9EV[3`\0\x90\x81R` \x81\x90R`@\x90 T\x15a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x1C\x99[\x18^Y\\\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0\x85\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x1C\x98[\x99\x12\x18\\\xDA\x08\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`*\x1B`D\x82\x01R`d\x01a\x02\xD9V[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02\x85\x85`@Qa\x04*\x92\x91\x90a\t\xD0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\x19[XZ[\x10Y\x19\x1C\x88\x18[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\"\x1B`D\x82\x01R`d\x01a\x02\xD9V[`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x91\x81\x01\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP3\x81R` \x81\x81R`@\x90\x91 \x83Q\x81U\x90\x83\x01Q\x90\x91P`\x01\x82\x01\x90a\x05:\x90\x82a\nDV[P`@\x82\x01Q`\x02\x82\x01\x90a\x05O\x90\x82a\nDV[PPP`\0\x85\x81R`\x01` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x90Q`\x02\x90a\x05\x87\x90\x87\x90\x87\x90a\t\xD0V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U3\x90\x7F\x07\xD4\x89J\xB9\xBDd\xE8\xAB\xA0\x8Ay\xDCA\x88\x0B7\xAF\x16\xC7\\\x8E\xD7>v\x1D\xA6R<4\xE3\xFA\x90a\x05\xEE\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90a\x0B&V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x81\x90\x03a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD9\x90a\t\x9EV[3`\0\x90\x81R` \x81\x90R`@\x90 Ta\x06sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`D\x82\x01R`d\x01a\x02\xD9V[3`\0\x90\x81R` \x81\x90R`@\x90 `\x02\x01a\x06\x90\x82\x84\x83a\x0B_V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7Fo\xF0Tl\x9B\x98H\x13\x86.\xE7\xFDL\x0B\xCC\xB1\xD8<\x1DVp\xF9S\xF0\\\xA4\xBA\xE5d\x18\xD5\xEE\x83\x83`@Qa\x06\xCC\x92\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xA2PPV[`\0` \x82\x84\x03\x12\x15a\x06\xEAW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x01W`\0\x80\xFD[\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x07.W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07\x12V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R``` \x82\x01R`\0a\x07g``\x83\x01\x85a\x07\x08V[\x82\x81\x03`@\x84\x01Ra\x07y\x81\x85a\x07\x08V[\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x95W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xACW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x07\xC4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x07\xE3W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\x01W`\0\x80\xFD[a\x08\r\x89\x83\x8A\x01a\x07\x83V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x08&W`\0\x80\xFD[Pa\x083\x88\x82\x89\x01a\x07\x83V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x08WW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08mW`\0\x80\xFD[a\x08y\x85\x82\x86\x01a\x07\x83V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x97W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\xDDW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x08\xF1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\t\x03Wa\t\x03a\x08\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t+Wa\t+a\x08\x9EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\tDW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\txW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x98WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01Rwhostname cannot be empty`@\x1B`@\x82\x01R``\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\n*W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\n\x07WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\n&W\x82\x81U`\x01\x01a\n\x13V[PPP[PPPV[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n]Wa\n]a\x08\x9EV[a\nq\x81a\nk\x84Ta\tdV[\x84a\t\xE0V[` \x80`\x1F\x83\x11`\x01\x81\x14a\n\xA0W`\0\x84\x15a\n\x8EWP\x85\x83\x01Q[a\n\x98\x85\x82a\n/V[\x86UPa\n&V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\n\xCFW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\n\xB0V[P\x85\x82\x10\x15a\n\xEDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0a\x0B@``\x83\x01\x86\x88a\n\xFDV[\x82\x81\x03`@\x84\x01Ra\x0BS\x81\x85\x87a\n\xFDV[\x98\x97PPPPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x0BvWa\x0Bva\x08\x9EV[a\x0B\x8A\x83a\x0B\x84\x83Ta\tdV[\x83a\t\xE0V[`\0`\x1F\x84\x11`\x01\x81\x14a\x0B\xB8W`\0\x85\x15a\x0B\xA6WP\x83\x82\x015[a\x0B\xB0\x86\x82a\n/V[\x84UPa\x0C\x12V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0B\xE9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0B\xC9V[P\x86\x82\x10\x15a\x0C\x06W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[` \x81R`\0a\x0C-` \x83\x01\x84\x86a\n\xFDV[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 =\xD3\xB5\xAAy'\xC4\xE1\x11\xB61\xE7\xCA\x0B\xEEl\xE5\x1F`}\xF5*a\x96$\xD8D\x07\x11\\\xDD?dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static RELAYERHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct RelayerHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RelayerHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RelayerHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RelayerHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RelayerHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RelayerHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RelayerHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                RELAYERHANDLER_ABI.clone(),
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
                RELAYERHANDLER_ABI.clone(),
                RELAYERHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getRandHash` (0x3f5615a4) function
        pub fn get_rand_hash(
            &self,
            relayer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([63, 86, 21, 164], relayer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerRelayer` (0x5d23392c) function
        pub fn register_relayer(
            &self,
            rand_hash: [u8; 32],
            email_addr: ::std::string::String,
            hostname: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 35, 57, 44], (rand_hash, email_addr, hostname))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerOfEmailAddr` (0xcd092c1a) function
        pub fn relayer_of_email_addr(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([205, 9, 44, 26], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerOfRandHash` (0x8283b575) function
        pub fn relayer_of_rand_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([130, 131, 181, 117], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayers` (0x5300f841) function
        pub fn relayers(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::std::string::String, ::std::string::String),
        > {
            self.0
                .method_hash([83, 0, 248, 65], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRelayerConfig` (0x7e2f2bbc) function
        pub fn update_relayer_config(
            &self,
            hostname: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 47, 43, 188], hostname)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RelayerConfigUpdated` event
        pub fn relayer_config_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RelayerConfigUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RelayerRegistered` event
        pub fn relayer_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RelayerRegisteredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RelayerHandlerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for RelayerHandler<M>
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
        name = "RelayerConfigUpdated",
        abi = "RelayerConfigUpdated(address,string)"
    )]
    pub struct RelayerConfigUpdatedFilter {
        #[ethevent(indexed)]
        pub addr: ::ethers::core::types::Address,
        pub hostname: ::std::string::String,
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
        name = "RelayerRegistered",
        abi = "RelayerRegistered(address,bytes32,string,string)"
    )]
    pub struct RelayerRegisteredFilter {
        #[ethevent(indexed)]
        pub addr: ::ethers::core::types::Address,
        pub rand_hash: [u8; 32],
        pub email_addr: ::std::string::String,
        pub hostname: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RelayerHandlerEvents {
        RelayerConfigUpdatedFilter(RelayerConfigUpdatedFilter),
        RelayerRegisteredFilter(RelayerRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for RelayerHandlerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RelayerConfigUpdatedFilter::decode_log(log) {
                return Ok(RelayerHandlerEvents::RelayerConfigUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RelayerRegisteredFilter::decode_log(log) {
                return Ok(RelayerHandlerEvents::RelayerRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RelayerHandlerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RelayerConfigUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RelayerConfigUpdatedFilter> for RelayerHandlerEvents {
        fn from(value: RelayerConfigUpdatedFilter) -> Self {
            Self::RelayerConfigUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RelayerRegisteredFilter> for RelayerHandlerEvents {
        fn from(value: RelayerRegisteredFilter) -> Self {
            Self::RelayerRegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `getRandHash` function with signature `getRandHash(address)` and selector `0x3f5615a4`
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
    #[ethcall(name = "getRandHash", abi = "getRandHash(address)")]
    pub struct GetRandHashCall {
        pub relayer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `registerRelayer` function with signature `registerRelayer(bytes32,string,string)` and selector `0x5d23392c`
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
        name = "registerRelayer",
        abi = "registerRelayer(bytes32,string,string)"
    )]
    pub struct RegisterRelayerCall {
        pub rand_hash: [u8; 32],
        pub email_addr: ::std::string::String,
        pub hostname: ::std::string::String,
    }
    ///Container type for all input parameters for the `relayerOfEmailAddr` function with signature `relayerOfEmailAddr(string)` and selector `0xcd092c1a`
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
    #[ethcall(name = "relayerOfEmailAddr", abi = "relayerOfEmailAddr(string)")]
    pub struct RelayerOfEmailAddrCall(pub ::std::string::String);
    ///Container type for all input parameters for the `relayerOfRandHash` function with signature `relayerOfRandHash(bytes32)` and selector `0x8283b575`
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
    #[ethcall(name = "relayerOfRandHash", abi = "relayerOfRandHash(bytes32)")]
    pub struct RelayerOfRandHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `relayers` function with signature `relayers(address)` and selector `0x5300f841`
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
    #[ethcall(name = "relayers", abi = "relayers(address)")]
    pub struct RelayersCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `updateRelayerConfig` function with signature `updateRelayerConfig(string)` and selector `0x7e2f2bbc`
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
    #[ethcall(name = "updateRelayerConfig", abi = "updateRelayerConfig(string)")]
    pub struct UpdateRelayerConfigCall {
        pub hostname: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RelayerHandlerCalls {
        GetRandHash(GetRandHashCall),
        RegisterRelayer(RegisterRelayerCall),
        RelayerOfEmailAddr(RelayerOfEmailAddrCall),
        RelayerOfRandHash(RelayerOfRandHashCall),
        Relayers(RelayersCall),
        UpdateRelayerConfig(UpdateRelayerConfigCall),
    }
    impl ::ethers::core::abi::AbiDecode for RelayerHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetRandHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRandHash(decoded));
            }
            if let Ok(decoded) =
                <RegisterRelayerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterRelayer(decoded));
            }
            if let Ok(decoded) =
                <RelayerOfEmailAddrCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RelayerOfEmailAddr(decoded));
            }
            if let Ok(decoded) =
                <RelayerOfRandHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RelayerOfRandHash(decoded));
            }
            if let Ok(decoded) = <RelayersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Relayers(decoded));
            }
            if let Ok(decoded) =
                <UpdateRelayerConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateRelayerConfig(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RelayerHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetRandHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterRelayer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RelayerOfEmailAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayerOfRandHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Relayers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateRelayerConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RelayerHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetRandHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerOfEmailAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerOfRandHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relayers(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRelayerConfig(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetRandHashCall> for RelayerHandlerCalls {
        fn from(value: GetRandHashCall) -> Self {
            Self::GetRandHash(value)
        }
    }
    impl ::core::convert::From<RegisterRelayerCall> for RelayerHandlerCalls {
        fn from(value: RegisterRelayerCall) -> Self {
            Self::RegisterRelayer(value)
        }
    }
    impl ::core::convert::From<RelayerOfEmailAddrCall> for RelayerHandlerCalls {
        fn from(value: RelayerOfEmailAddrCall) -> Self {
            Self::RelayerOfEmailAddr(value)
        }
    }
    impl ::core::convert::From<RelayerOfRandHashCall> for RelayerHandlerCalls {
        fn from(value: RelayerOfRandHashCall) -> Self {
            Self::RelayerOfRandHash(value)
        }
    }
    impl ::core::convert::From<RelayersCall> for RelayerHandlerCalls {
        fn from(value: RelayersCall) -> Self {
            Self::Relayers(value)
        }
    }
    impl ::core::convert::From<UpdateRelayerConfigCall> for RelayerHandlerCalls {
        fn from(value: UpdateRelayerConfigCall) -> Self {
            Self::UpdateRelayerConfig(value)
        }
    }
    ///Container type for all return fields from the `getRandHash` function with signature `getRandHash(address)` and selector `0x3f5615a4`
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
    pub struct GetRandHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `relayerOfEmailAddr` function with signature `relayerOfEmailAddr(string)` and selector `0xcd092c1a`
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
    pub struct RelayerOfEmailAddrReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `relayerOfRandHash` function with signature `relayerOfRandHash(bytes32)` and selector `0x8283b575`
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
    pub struct RelayerOfRandHashReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `relayers` function with signature `relayers(address)` and selector `0x5300f841`
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
    pub struct RelayersReturn {
        pub rand_hash: [u8; 32],
        pub email_addr: ::std::string::String,
        pub hostname: ::std::string::String,
    }
}
