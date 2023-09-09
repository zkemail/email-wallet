use halo2_regex::vrm::DecomposedRegexConfig;
use neon::prelude::*;
pub mod extract_substrs;
pub mod parse_email;
mod statics;
use parse_email::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseEmail", parse_email_node)?;
    Ok(())
}
