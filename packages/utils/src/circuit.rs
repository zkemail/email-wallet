use std::cmp;

use num_bigint::BigInt;

use crate::{generate_partial_sha, sha256_pad, to_circom_bigint_bytes, uint8_array_to_char_array};

pub const MAX_HEADER_PADDED_BYTES: usize = 1024;
pub const MAX_BODY_PADDED_BYTES: usize = 1536;
pub const CIRCOM_BIGINT_N: usize = 121;
pub const CIRCOM_BIGINT_K: usize = 17;

pub struct CircuitInput {
    pub in_padded: Vec<String>,
    pub pubkey: Vec<String>,
    pub signature: Vec<String>,
    pub in_len_padded_bytes: String,
    pub precomputed_sha: Option<Vec<String>>,
    pub in_body_padded: Option<Vec<String>>,
    pub in_body_len_padded_bytes: Option<String>,
    pub body_hash_idx: Option<String>,
}

pub struct CircuitInputParams {
    body: Vec<u8>,
    message: Vec<u8>,
    body_hash: String,
    rsa_signature: BigInt,
    rsa_public_key: BigInt,
    sha_precompute_selector: Option<String>,
    max_message_length: usize,
    max_body_length: usize,
    ignore_body_hash_check: bool,
}

impl CircuitInputParams {
    // Provides default values for optional parameters
    pub fn new(
        body: Vec<u8>,
        message: Vec<u8>,
        body_hash: String,
        rsa_signature: BigInt,
        rsa_public_key: BigInt,
        sha_precompute_selector: Option<String>,
        max_message_length: Option<usize>,
        max_body_length: Option<usize>,
        ignore_body_hash_check: Option<bool>,
    ) -> Self {
        CircuitInputParams {
            body,
            message,
            body_hash,
            rsa_signature,
            rsa_public_key,
            sha_precompute_selector,
            max_message_length: max_message_length.unwrap_or(MAX_HEADER_PADDED_BYTES),
            max_body_length: max_body_length.unwrap_or(MAX_BODY_PADDED_BYTES),
            ignore_body_hash_check: ignore_body_hash_check.unwrap_or(false),
        }
    }
}

pub fn generate_circuit_inputs(params: CircuitInputParams) -> CircuitInput {
    let (message_padded, message_padded_len) =
        sha256_pad(params.message.clone(), params.max_message_length);
    let body_sha_length = ((params.body.len() + 63 + 65) / 64) * 64;
    let (body_padded, body_padded_len) = sha256_pad(
        params.body,
        cmp::max(params.max_body_length, body_sha_length),
    );

    let result = generate_partial_sha(
        body_padded,
        body_padded_len,
        params.sha_precompute_selector,
        params.max_body_length,
    );

    let (precomputed_sha, body_remaining, body_remaining_length) = match result {
        Ok((sha, remaining, len)) => (sha, remaining, len),
        Err(e) => panic!("Failed to generate partial SHA: {:?}", e),
    };

    let mut circuit_input = CircuitInput {
        in_padded: uint8_array_to_char_array(message_padded),
        pubkey: to_circom_bigint_bytes(params.rsa_public_key),
        signature: to_circom_bigint_bytes(params.rsa_signature),
        in_len_padded_bytes: message_padded_len.to_string(),
        precomputed_sha: None,
        in_body_padded: None,
        in_body_len_padded_bytes: None,
        body_hash_idx: None,
    };

    if !params.ignore_body_hash_check {
        circuit_input.precomputed_sha = Some(uint8_array_to_char_array(precomputed_sha));
        // Convert message into a string
        let message_string = String::from_utf8(params.message).expect("Found invalid UTF-8");
        let body_hash_idx = message_string
            .find(&params.body_hash)
            .unwrap_or_else(|| panic!("Body hash not found in message"));
        circuit_input.body_hash_idx = Some(body_hash_idx.to_string());
        circuit_input.in_body_padded = Some(uint8_array_to_char_array(body_remaining));
        circuit_input.in_body_len_padded_bytes = Some(body_remaining_length.to_string());
    }
    circuit_input
}
