use std::convert::TryInto;

use anyhow;
use halo2curves::ff::PrimeField;
use itertools::Itertools;
use neon::prelude::*;
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

pub fn digits2int(input_digits: &str) -> anyhow::Result<u64> {
    Ok(input_digits.parse()?)
}

pub fn bytes2fields(bytes: &[u8]) -> Vec<Fr> {
    bytes
        .chunks(31)
        .map(|bytes| {
            let mut bytes32 = [0; 32];
            bytes32[0..bytes.len()].clone_from_slice(bytes);
            Fr::from_bytes(&bytes32).expect("fail to convert bytes to a field value")
        })
        .collect_vec()
}

pub fn bytes_chunk_fields(bytes: &[u8], chunk_size: usize, num_chunk_in_field: usize) -> Vec<Fr> {
    let bits = bytes
        .iter()
        .flat_map(|byte| {
            let mut bits = vec![];
            for i in 0..8 {
                bits.push((byte >> i) & 1);
            }
            bits
        })
        .collect_vec();
    let words = bits
        .chunks(chunk_size)
        .map(|bits| {
            let mut word = Fr::zero();
            for (i, bit) in bits.iter().enumerate() {
                if *bit == 1 {
                    word += Fr::from_u128(1u128 << i);
                }
            }
            word
        })
        .collect_vec();
    let fields = words
        .chunks(num_chunk_in_field)
        .map(|words| {
            let mut input = Fr::zero();
            let mut coeff = Fr::one();
            let offset = Fr::from_u128(1u128 << chunk_size);
            for (_, word) in words.iter().enumerate() {
                input += coeff * word;
                coeff *= offset;
            }
            input
        })
        .collect_vec();
    fields
}

pub(crate) fn hex2field_node(cx: &mut FunctionContext, input_strs: &str) -> NeonResult<Fr> {
    match hex2field(input_strs) {
        Ok(field) => Ok(field),
        Err(e) => cx.throw_error(e.to_string()),
    }
}

pub fn bytes2fields_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsArray>(0)?;
    let input_vec = input_str.to_vec(&mut cx)?;
    let mut input_bytes = vec![];
    for val in input_vec.into_iter() {
        let val = match val.downcast::<JsNumber, _>(&mut cx) {
            Ok(v) => v.value(&mut cx),
            Err(e) => return cx.throw_error(e.to_string()),
        };
        input_bytes.push(val as u8);
    }
    let fields = bytes2fields(&input_bytes);
    let js_array = JsArray::new(&mut cx, fields.len() as u32);
    for (i, field) in fields.into_iter().enumerate() {
        let field = cx.string(&field2hex(&field));
        js_array.set(&mut cx, i as u32, field)?;
    }
    Ok(js_array)
}
