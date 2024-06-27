pub use i_oauth::*;
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
pub mod i_oauth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("reduceTokenAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reduceTokenAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonceHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerEpheAddr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("username"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("signin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAllowances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TokenAllowance[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isSudo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("signup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("username"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("validateEpheAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isSudo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonceHash"),
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
                    ::std::borrow::ToOwned::to_owned("validateSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
    pub static IOAUTH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
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
            f.debug_tuple(::core::stringify!(IOauth)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOauth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IOAUTH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `reduceTokenAllowance` (0xcddb4d66) function
        pub fn reduce_token_allowance(
            &self,
            nonce_hash: [u8; 32],
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 219, 77, 102], (nonce_hash, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerEpheAddr` (0x385ec107) function
        pub fn register_ephe_addr(
            &self,
            username: ::std::string::String,
            ephe_addr: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 94, 193, 7], (username, ephe_addr, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signin` (0x933a96a9) function
        pub fn signin(
            &self,
            username: ::std::string::String,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            token_allowances: ::std::vec::Vec<TokenAllowance>,
            is_sudo: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [147, 58, 150, 169],
                    (username, nonce, expiry, token_allowances, is_sudo),
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
        ///Calls the contract's `validateEpheAddr` (0x106a4cf8) function
        pub fn validate_ephe_addr(
            &self,
            wallet: ::ethers::core::types::Address,
            ephe_addr: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            is_sudo: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([16, 106, 76, 248], (wallet, ephe_addr, nonce, is_sudo))
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IOauth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `reduceTokenAllowance` function with signature `reduceTokenAllowance(bytes32,address,uint256)` and selector `0xcddb4d66`
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
        name = "reduceTokenAllowance",
        abi = "reduceTokenAllowance(bytes32,address,uint256)"
    )]
    pub struct ReduceTokenAllowanceCall {
        pub nonce_hash: [u8; 32],
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerEpheAddr` function with signature `registerEpheAddr(string,address,bytes)` and selector `0x385ec107`
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
    #[ethcall(name = "registerEpheAddr", abi = "registerEpheAddr(string,address,bytes)")]
    pub struct RegisterEpheAddrCall {
        pub username: ::std::string::String,
        pub ephe_addr: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `signin` function with signature `signin(string,uint256,uint256,(address,uint256)[],bool)` and selector `0x933a96a9`
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
        name = "signin",
        abi = "signin(string,uint256,uint256,(address,uint256)[],bool)"
    )]
    pub struct SigninCall {
        pub username: ::std::string::String,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub token_allowances: ::std::vec::Vec<TokenAllowance>,
        pub is_sudo: bool,
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
        Hash
    )]
    #[ethcall(name = "signup", abi = "signup(string)")]
    pub struct SignupCall {
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `validateEpheAddr` function with signature `validateEpheAddr(address,address,uint256,bool)` and selector `0x106a4cf8`
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
        name = "validateEpheAddr",
        abi = "validateEpheAddr(address,address,uint256,bool)"
    )]
    pub struct ValidateEpheAddrCall {
        pub wallet: ::ethers::core::types::Address,
        pub ephe_addr: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub is_sudo: bool,
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
        Hash
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
            if let Ok(decoded) = <ReduceTokenAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReduceTokenAllowance(decoded));
            }
            if let Ok(decoded) = <RegisterEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterEpheAddr(decoded));
            }
            if let Ok(decoded) = <SigninCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Signin(decoded));
            }
            if let Ok(decoded) = <SignupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Signup(decoded));
            }
            if let Ok(decoded) = <ValidateEpheAddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateEpheAddr(decoded));
            }
            if let Ok(decoded) = <ValidateSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOauthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ReduceTokenAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterEpheAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Signin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Signup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateEpheAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IOauthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ReduceTokenAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Signup(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateEpheAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSignature(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    ///Container type for all return fields from the `validateEpheAddr` function with signature `validateEpheAddr(address,address,uint256,bool)` and selector `0x106a4cf8`
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
    pub struct ValidateEpheAddrReturn {
        pub nonce_hash: [u8; 32],
    }
    ///`TokenAllowance(address,uint256)`
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
    pub struct TokenAllowance {
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
