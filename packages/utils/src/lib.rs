use neon::prelude::*;
use zk_regex_apis::extract_substrs::extract_substr_idxes_node;
pub mod cryptos;
pub mod parse_email;
mod statics;
use cryptos::*;
use parse_email::*;
pub use zk_regex_apis::extract_substrs::*;
pub use zk_regex_apis::padding::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseEmail", parse_email_node)?;
    cx.export_function("padString", pad_string_node)?;
    cx.export_function("extractSubstrIdxes", extract_substr_idxes_node)?;
    cx.export_function("genRelayerRand", gen_relayer_rand_node)?;
    cx.export_function("relayerRandHash", relayer_rand_hash_node)?;
    cx.export_function("padEmailAddr", pad_email_addr_node)?;
    cx.export_function("emailAddrPointer", email_addr_pointer_node)?;
    cx.export_function("emailAddrCommitRand", email_addr_commit_rand_node)?;
    cx.export_function("emailAddrCommit", email_addr_commit_node)?;
    cx.export_function(
        "emailAddrCommitWithSignature",
        email_addr_commit_with_signature_node,
    )?;
    cx.export_function("genAccountKey", gen_account_key_node)?;
    cx.export_function("accountKeyCommit", account_key_commit_node)?;
    cx.export_function("walletSalt", wallet_salt_node)?;
    // cx.export_function("extAccountSalt", ext_account_salt_node)?;
    cx.export_function("publicKeyHash", public_key_hash_node)?;
    cx.export_function("emailNullifier", email_nullifier_node)?;
    Ok(())
}
