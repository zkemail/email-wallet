use std::convert::TryInto;

use anyhow;
use ethers::types::U256;
use halo2curves::ff::PrimeField;
use itertools::Itertools;
use neon::prelude::*;
use num_bigint::BigInt;
use poseidon_rs::*;

pub use zk_regex_apis::padding::pad_string;

use crate::circuit::{CIRCOM_BIGINT_K, CIRCOM_BIGINT_N};

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

pub fn hex2field_node(cx: &mut FunctionContext, input_strs: &str) -> NeonResult<Fr> {
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

/// Converts a 64-bit integer to an array of 8 bytes in big-endian format.
pub fn int64_to_bytes(num: u64) -> Vec<u8> {
    num.to_be_bytes().to_vec()
}

/// Converts an 8-bit integer to a Vec<u8> with a single element.
pub fn int8_to_bytes(num: u8) -> Vec<u8> {
    vec![num]
}

/// Merges two Vec<u8> into a single Vec<u8>.
pub fn merge_u8_arrays(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    [a, b].concat()
}

pub fn uint8_array_to_char_array(bytes: Vec<u8>) -> Vec<String> {
    bytes.iter().map(|&b| b.to_string()).collect()
}

fn big_int_to_chunked_bytes(num: BigInt, bits_per_chunk: usize, num_chunks: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut remainder = num;
    let two = BigInt::from(2);
    let chunk_size = two.pow(bits_per_chunk as u32);

    // Divide the number into chunks and convert each to a decimal string
    for _ in 0..num_chunks {
        let chunk = &remainder % &chunk_size;
        remainder = remainder >> bits_per_chunk;
        // Convert chunk to decimal string
        chunks.push(chunk.to_string());
    }

    chunks
}

pub fn to_circom_bigint_bytes(num: BigInt) -> Vec<String> {
    big_int_to_chunked_bytes(num, CIRCOM_BIGINT_N, CIRCOM_BIGINT_K)
}

pub fn vec_u8_to_bigint(bytes: Vec<u8>) -> BigInt {
    bytes
        .iter()
        .fold(BigInt::from(0), |acc, &b| (acc << 8) | BigInt::from(b))
}

pub fn u256_to_bytes32(x: &U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    x.to_big_endian(&mut bytes);
    bytes
}

pub fn u256_to_hex(x: &U256) -> String {
    "0x".to_string() + &hex::encode(u256_to_bytes32(x))
}

pub fn hex_to_u256(hex: &str) -> Result<U256, hex::FromHexError> {
    let bytes: Vec<u8> = hex::decode(&hex[2..])?;
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Ok(U256::from_big_endian(&array))
}

pub fn fr_to_bytes32(fr: &Fr) -> Result<[u8; 32], hex::FromHexError> {
    let hex = field2hex(fr);
    let bytes = hex::decode(&hex[2..])?;
    let mut result = [0u8; 32];
    result.copy_from_slice(&bytes);
    Ok(result)
}

pub fn bytes32_to_fr(bytes32: &[u8; 32]) -> Result<Fr, hex::FromHexError> {
    let hex: String = "0x".to_string() + &hex::encode(bytes32);
    hex2field(&hex).map_err(|_e| hex::FromHexError::InvalidStringLength)
}
