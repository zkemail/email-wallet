pub use i_oauth::*;
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
pub mod i_oauth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getInfoOfWalletAndNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInfoOfWalletAndNonce",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenAllowancesOfWalletAndNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTokenAllowancesOfWalletAndNonce",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getUsernameOfWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getUsernameOfWallet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("wallet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getWalletOfUsername"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getWalletOfUsername",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("username"),
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
                    ::std::borrow::ToOwned::to_owned("reduceTokenAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reduceTokenAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
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
                    ::std::borrow::ToOwned::to_owned("signin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAllowances"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct TokenAllowance[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signup"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signup"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("username"),
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
                (
                    ::std::borrow::ToOwned::to_owned("validateEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSignature"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ReducedTokenAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ReducedTokenAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RegisteredEpheAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RegisteredEpheAddr"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("epheAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Signin"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Signin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAllowances"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Signup"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Signup"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("username"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
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
    pub static IOAUTH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IOauth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IOauth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IOauth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IOauth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IOauth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IOauth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOauth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IOAUTH_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getInfoOfWalletAndNonce` (0xb5703a55) function
        pub fn get_info_of_wallet_and_nonce(
            &self,
            wallet: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([181, 112, 58, 85], (wallet, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAllowancesOfWalletAndNonce` (0xeb1202ff) function
        pub fn get_token_allowances_of_wallet_and_nonce(
            &self,
            wallet: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            token_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 18, 2, 255], (wallet, nonce, token_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUsernameOfWallet` (0x7218fda4) function
        pub fn get_username_of_wallet(
            &self,
            wallet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 24, 253, 164], wallet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWalletOfUsername` (0x364da6af) function
        pub fn get_wallet_of_username(
            &self,
            username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([54, 77, 166, 175], username)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reduceTokenAllowance` (0xf0e384a5) function
        pub fn reduce_token_allowance(
            &self,
            nonce: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 227, 132, 165], (nonce, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerEpheAddr` (0x24619a7c) function
        pub fn register_ephe_addr(
            &self,
            wallet: ::ethers::core::types::Address,
            ephe_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 97, 154, 124], (wallet, ephe_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signin` (0x5dcb7f88) function
        pub fn signin(
            &self,
            username: ::std::string::String,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            token_allowances: ::std::vec::Vec<TokenAllowance>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [93, 203, 127, 136],
                    (username, nonce, expiry, token_allowances),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signup` (0x519c6377) function
        pub fn signup(
            &self,
            username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 156, 99, 119], username)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateEpheAddr` (0xf6434da5) function
        pub fn validate_ephe_addr(
            &self,
            wallet: ::ethers::core::types::Address,
            ephe_addr: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 67, 77, 165], (wallet, ephe_addr, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSignature` (0x971604c6) function
        pub fn validate_signature(
            &self,
            ephe_addr: ::ethers::core::types::Address,
            hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 22, 4, 198], (ephe_addr, hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ReducedTokenAllowance` event
        pub fn reduced_token_allowance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ReducedTokenAllowanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RegisteredEpheAddr` event
        pub fn registered_ephe_addr_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisteredEpheAddrFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Signin` event
        pub fn signin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SigninFilter> {
            self.0.event()
        }
        ///Gets the contract's `Signup` event
        pub fn signup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignupFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IOauthEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IOauth<M> {
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
        name = "ReducedTokenAllowance",
        abi = "ReducedTokenAllowance(address,address,uint256,uint256)"
    )]
    pub struct ReducedTokenAllowanceFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
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
        name = "RegisteredEpheAddr",
        abi = "RegisteredEpheAddr(address,address,uint256,string)"
    )]
    pub struct RegisteredEpheAddrFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ephe_addr: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
        pub username: ::std::string::String,
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
        name = "Signin",
        abi = "Signin(address,string,uint256,uint256,(address,uint256)[])"
    )]
    pub struct SigninFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub username: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub token_allowances: ::std::vec::Vec<TokenAllowance>,
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
    #[ethevent(name = "Signup", abi = "Signup(address,string)")]
    pub struct SignupFilter {
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub username: ::ethers::core::types::H256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IOauthEvents {
        ReducedTokenAllowanceFilter(ReducedTokenAllowanceFilter),
        RegisteredEpheAddrFilter(RegisteredEpheAddrFilter),
        SigninFilter(SigninFilter),
        SignupFilter(SignupFilter),
    }
    impl ::ethers::contract::EthLogDecode for IOauthEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ReducedTokenAllowanceFilter::decode_log(log) {
                return Ok(IOauthEvents::ReducedTokenAllowanceFilter(decoded));
            }
            if let Ok(decoded) = RegisteredEpheAddrFilter::decode_log(log) {
                return Ok(IOauthEvents::RegisteredEpheAddrFilter(decoded));
            }
            if let Ok(decoded) = SigninFilter::decode_log(log) {
                return Ok(IOauthEvents::SigninFilter(decoded));
            }
            if let Ok(decoded) = SignupFilter::decode_log(log) {
                return Ok(IOauthEvents::SignupFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IOauthEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ReducedTokenAllowanceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredEpheAddrFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SigninFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignupFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ReducedTokenAllowanceFilter> for IOauthEvents {
        fn from(value: ReducedTokenAllowanceFilter) -> Self {
            Self::ReducedTokenAllowanceFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredEpheAddrFilter> for IOauthEvents {
        fn from(value: RegisteredEpheAddrFilter) -> Self {
            Self::RegisteredEpheAddrFilter(value)
        }
    }
    impl ::core::convert::From<SigninFilter> for IOauthEvents {
        fn from(value: SigninFilter) -> Self {
            Self::SigninFilter(value)
        }
    }
    impl ::core::convert::From<SignupFilter> for IOauthEvents {
        fn from(value: SignupFilter) -> Self {
            Self::SignupFilter(value)
        }
    }
    ///Container type for all input parameters for the `getInfoOfWalletAndNonce` function with signature `getInfoOfWalletAndNonce(address,uint256)` and selector `0xb5703a55`
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
        name = "getInfoOfWalletAndNonce",
        abi = "getInfoOfWalletAndNonce(address,uint256)"
    )]
    pub struct GetInfoOfWalletAndNonceCall {
        pub wallet: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTokenAllowancesOfWalletAndNonce` function with signature `getTokenAllowancesOfWalletAndNonce(address,uint256,address)` and selector `0xeb1202ff`
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
        name = "getTokenAllowancesOfWalletAndNonce",
        abi = "getTokenAllowancesOfWalletAndNonce(address,uint256,address)"
    )]
    pub struct GetTokenAllowancesOfWalletAndNonceCall {
        pub wallet: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub token_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUsernameOfWallet` function with signature `getUsernameOfWallet(address)` and selector `0x7218fda4`
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
    #[ethcall(name = "getUsernameOfWallet", abi = "getUsernameOfWallet(address)")]
    pub struct GetUsernameOfWalletCall {
        pub wallet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getWalletOfUsername` function with signature `getWalletOfUsername(string)` and selector `0x364da6af`
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
    #[ethcall(name = "getWalletOfUsername", abi = "getWalletOfUsername(string)")]
    pub struct GetWalletOfUsernameCall {
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `reduceTokenAllowance` function with signature `reduceTokenAllowance(uint256,address,uint256)` and selector `0xf0e384a5`
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
        name = "reduceTokenAllowance",
        abi = "reduceTokenAllowance(uint256,address,uint256)"
    )]
    pub struct ReduceTokenAllowanceCall {
        pub nonce: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerEpheAddr` function with signature `registerEpheAddr(address,address)` and selector `0x24619a7c`
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
    #[ethcall(name = "registerEpheAddr", abi = "registerEpheAddr(address,address)")]
    pub struct RegisterEpheAddrCall {
        pub wallet: ::ethers::core::types::Address,
        pub ephe_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signin` function with signature `signin(string,uint256,uint256,(address,uint256)[])` and selector `0x5dcb7f88`
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
        name = "signin",
        abi = "signin(string,uint256,uint256,(address,uint256)[])"
    )]
    pub struct SigninCall {
        pub username: ::std::string::String,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub token_allowances: ::std::vec::Vec<TokenAllowance>,
    }
    ///Container type for all input parameters for the `signup` function with signature `signup(string)` and selector `0x519c6377`
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
    #[ethcall(name = "signup", abi = "signup(string)")]
    pub struct SignupCall {
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `validateEpheAddr` function with signature `validateEpheAddr(address,address,uint256)` and selector `0xf6434da5`
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
        name = "validateEpheAddr",
        abi = "validateEpheAddr(address,address,uint256)"
    )]
    pub struct ValidateEpheAddrCall {
        pub wallet: ::ethers::core::types::Address,
        pub ephe_addr: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `validateSignature` function with signature `validateSignature(address,bytes32,bytes)` and selector `0x971604c6`
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
        name = "validateSignature",
        abi = "validateSignature(address,bytes32,bytes)"
    )]
    pub struct ValidateSignatureCall {
        pub ephe_addr: ::ethers::core::types::Address,
        pub hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IOauthCalls {
        GetInfoOfWalletAndNonce(GetInfoOfWalletAndNonceCall),
        GetTokenAllowancesOfWalletAndNonce(GetTokenAllowancesOfWalletAndNonceCall),
        GetUsernameOfWallet(GetUsernameOfWalletCall),
        GetWalletOfUsername(GetWalletOfUsernameCall),
        ReduceTokenAllowance(ReduceTokenAllowanceCall),
        RegisterEpheAddr(RegisterEpheAddrCall),
        Signin(SigninCall),
        Signup(SignupCall),
        ValidateEpheAddr(ValidateEpheAddrCall),
        ValidateSignature(ValidateSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOauthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetInfoOfWalletAndNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInfoOfWalletAndNonce(decoded));
            }
            if let Ok(decoded) =
                <GetTokenAllowancesOfWalletAndNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetTokenAllowancesOfWalletAndNonce(decoded));
            }
            if let Ok(decoded) =
                <GetUsernameOfWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUsernameOfWallet(decoded));
            }
            if let Ok(decoded) =
                <GetWalletOfUsernameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetWalletOfUsername(decoded));
            }
            if let Ok(decoded) =
                <ReduceTokenAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReduceTokenAllowance(decoded));
            }
            if let Ok(decoded) =
                <RegisterEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterEpheAddr(decoded));
            }
            if let Ok(decoded) = <SigninCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Signin(decoded));
            }
            if let Ok(decoded) = <SignupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Signup(decoded));
            }
            if let Ok(decoded) =
                <ValidateEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateEpheAddr(decoded));
            }
            if let Ok(decoded) =
                <ValidateSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOauthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetInfoOfWalletAndNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAllowancesOfWalletAndNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUsernameOfWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletOfUsername(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReduceTokenAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterEpheAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Signin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Signup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateEpheAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IOauthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetInfoOfWalletAndNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenAllowancesOfWalletAndNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUsernameOfWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWalletOfUsername(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReduceTokenAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signup(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetInfoOfWalletAndNonceCall> for IOauthCalls {
        fn from(value: GetInfoOfWalletAndNonceCall) -> Self {
            Self::GetInfoOfWalletAndNonce(value)
        }
    }
    impl ::core::convert::From<GetTokenAllowancesOfWalletAndNonceCall> for IOauthCalls {
        fn from(value: GetTokenAllowancesOfWalletAndNonceCall) -> Self {
            Self::GetTokenAllowancesOfWalletAndNonce(value)
        }
    }
    impl ::core::convert::From<GetUsernameOfWalletCall> for IOauthCalls {
        fn from(value: GetUsernameOfWalletCall) -> Self {
            Self::GetUsernameOfWallet(value)
        }
    }
    impl ::core::convert::From<GetWalletOfUsernameCall> for IOauthCalls {
        fn from(value: GetWalletOfUsernameCall) -> Self {
            Self::GetWalletOfUsername(value)
        }
    }
    impl ::core::convert::From<ReduceTokenAllowanceCall> for IOauthCalls {
        fn from(value: ReduceTokenAllowanceCall) -> Self {
            Self::ReduceTokenAllowance(value)
        }
    }
    impl ::core::convert::From<RegisterEpheAddrCall> for IOauthCalls {
        fn from(value: RegisterEpheAddrCall) -> Self {
            Self::RegisterEpheAddr(value)
        }
    }
    impl ::core::convert::From<SigninCall> for IOauthCalls {
        fn from(value: SigninCall) -> Self {
            Self::Signin(value)
        }
    }
    impl ::core::convert::From<SignupCall> for IOauthCalls {
        fn from(value: SignupCall) -> Self {
            Self::Signup(value)
        }
    }
    impl ::core::convert::From<ValidateEpheAddrCall> for IOauthCalls {
        fn from(value: ValidateEpheAddrCall) -> Self {
            Self::ValidateEpheAddr(value)
        }
    }
    impl ::core::convert::From<ValidateSignatureCall> for IOauthCalls {
        fn from(value: ValidateSignatureCall) -> Self {
            Self::ValidateSignature(value)
        }
    }
    ///Container type for all return fields from the `getInfoOfWalletAndNonce` function with signature `getInfoOfWalletAndNonce(address,uint256)` and selector `0xb5703a55`
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
    pub struct GetInfoOfWalletAndNonceReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getTokenAllowancesOfWalletAndNonce` function with signature `getTokenAllowancesOfWalletAndNonce(address,uint256,address)` and selector `0xeb1202ff`
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
    pub struct GetTokenAllowancesOfWalletAndNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUsernameOfWallet` function with signature `getUsernameOfWallet(address)` and selector `0x7218fda4`
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
    pub struct GetUsernameOfWalletReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getWalletOfUsername` function with signature `getWalletOfUsername(string)` and selector `0x364da6af`
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
    pub struct GetWalletOfUsernameReturn(pub ::ethers::core::types::Address);
    ///`TokenAllowance(address,uint256)`
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
    pub struct TokenAllowance {
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
