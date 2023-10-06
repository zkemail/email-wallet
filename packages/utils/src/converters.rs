use std::convert::TryInto;
use std::str::FromStr;

use anyhow;
use halo2curves::ff::derive::rand_core::OsRng;
use halo2curves::ff::{Field, PrimeField};
use halo2curves::{ff::derive::rand_core::RngCore, serde::SerdeObject};
use itertools::Itertools;
use neon::prelude::*;
use neon::result::Throw;
use num_bigint::BigUint;
use num_traits::Zero;
use poseidon_rs::*;
pub use zk_regex_apis::padding::{pad_string, pad_string_node};

pub fn hex2field(input_hex: &str) -> anyhow::Result<Fr> {
    if &input_hex[0..2] != "0x" {
        return Err(anyhow::anyhow!(format!(
            "the input string {} must be hex string with 0x prefix",
            &input_hex
        )));
    }
    let mut bytes = match hex::decode(&input_hex[2..]) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(anyhow::anyhow!(format!(
                "the input string {} is invalid hex: {}",
                &input_hex, e
            )));
        }
    };
    bytes.reverse();
    if bytes.len() != 32 {
        return Err(anyhow::anyhow!(format!(
            "the input string {} must be 32 bytes but is {} bytes",
            &input_hex,
            bytes.len()
        )));
    }
    let bytes: [u8; 32] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(anyhow::anyhow!(format!(
                "the bytes {:?} is not valid 32 bytes",
                e
            )))
        }
    };
    let field = Fr::from_bytes(&bytes).expect("fail to convert bytes to a field value");
    Ok(field)
}

pub fn field2hex(field: &Fr) -> String {
    format!("{:?}", field)
}

pub(crate) fn hex2field_node(cx: &mut FunctionContext, input_strs: &str) -> NeonResult<Fr> {
    match hex2field(input_strs) {
        Ok(field) => Ok(field),
        Err(e) => cx.throw_error(e.to_string()),
    }
}
