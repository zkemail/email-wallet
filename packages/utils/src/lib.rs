use halo2_regex::vrm::DecomposedRegexConfig;
use neon::prelude::*;
pub mod cryptos;
pub mod extract_substrs;
pub mod parse_email;
mod statics;
use cryptos::*;
use extract_substrs::*;
use parse_email::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseEmail", parse_email_node)?;
    cx.export_function("extractSubstrIdxes", extract_substr_idxes_node)?;
    cx.export_function("genRelayerRand", gen_relayer_rand_node)?;
    cx.export_function("relayerRandHash", relayer_rand_hash_node)?;
    cx.export_function("paddedEmailAddr", padded_email_addr_node)?;
    cx.export_function("emailAddrPointer", email_addr_pointer_node)?;
    cx.export_function("emailAddrCommit", email_addr_commit_node)?;
    cx.export_function("genViewingKey", gen_viewing_key_node)?;
    cx.export_function("viewingKeyCommit", viewing_key_commit_node)?;
    cx.export_function("walletSalt", wallet_salt_node)?;
    cx.export_function("extAccountSalt", ext_account_salt_node)?;
    Ok(())
}
