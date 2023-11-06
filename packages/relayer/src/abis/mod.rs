pub(crate) mod account_handler;
pub(crate) mod email_wallet_core;
pub(crate) mod erc20;
pub(crate) mod events;
pub(crate) mod extension_handler;
pub(crate) mod relayer_handler;
pub(crate) mod token_registry;
pub(crate) mod unclaims_handler;

pub(crate) use account_handler::*;
pub(crate) use email_wallet_core::*;
pub(crate) use erc20::*;
pub(crate) use events::*;
pub(crate) use extension_handler::*;
pub(crate) use relayer_handler::*;
pub(crate) use token_registry::*;
pub(crate) use unclaims_handler::*;
