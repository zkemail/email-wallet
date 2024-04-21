use std::cmp;

use crate::*;
use anyhow::Result;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use crate::{generate_partial_sha, sha256_pad, to_circom_bigint_bytes, uint8_array_to_char_array};

pub const MAX_HEADER_PADDED_BYTES: usize = 1024;
pub const MAX_BODY_PADDED_BYTES: usize = 1536;
pub const CIRCOM_BIGINT_N: usize = 121;
pub const CIRCOM_BIGINT_K: usize = 17;

#[derive(Serialize, Deserialize)]
struct EmailSenderInput {
    in_padded: Vec<String>,
    pubkey: Vec<String>,
    signature: Vec<String>,
    in_padded_len: String,
    sender_account_key: String,
    sender_email_idx: usize,
    subject_idx: usize,
    recipient_email_idx: usize,
    domain_idx: usize,
    timestamp_idx: usize,
}

#[derive(Serialize, Deserialize)]
struct AccountCreationInput {
    in_padded: Vec<String>,
    pubkey: Vec<String>,
    signature: Vec<String>,
    in_padded_len: String,
    relayer_rand: String,
    sender_email_idx: usize,
    code_idx: usize,
    domain_idx: usize,
    timestamp_idx: usize,
}

#[derive(Serialize, Deserialize)]
struct ClaimInput {
    email_addr: Vec<u8>,
    cm_rand: String,
    account_key: String,
}

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

pub async fn generate_email_sender_input(email: &str, account_key: &str) -> Result<String> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let circuit_input_params = circuit::CircuitInputParams::new(
        vec![],
        parsed_email.canonicalized_header.as_bytes().to_vec(),
        "".to_string(),
        vec_u8_to_bigint(parsed_email.clone().signature),
        vec_u8_to_bigint(parsed_email.clone().public_key),
        None,
        Some(1024),
        Some(64),
        Some(true),
    );
    let email_circuit_inputs = circuit::generate_circuit_inputs(circuit_input_params);

    let sender_email_idx = parsed_email.get_from_addr_idxes().unwrap();
    let domain_idx = parsed_email.get_email_domain_idxes().unwrap();
    let subject_idx = parsed_email.get_subject_all_idxes().unwrap();
    let recipient_email_idx = match parsed_email.get_email_addr_in_subject_idxes() {
        Ok(idx) => idx.0,
        Err(_) => {
            0 // Assuming 0 is a safe default or placeholder value
        }
    };
    let timestamp_idx = parsed_email.get_timestamp_idxes().unwrap();

    let email_sender_input = EmailSenderInput {
        in_padded: email_circuit_inputs.in_padded,
        pubkey: email_circuit_inputs.pubkey,
        signature: email_circuit_inputs.signature,
        in_padded_len: email_circuit_inputs.in_len_padded_bytes,
        sender_account_key: account_key.to_string(),
        sender_email_idx: sender_email_idx.0,
        subject_idx: subject_idx.0,
        recipient_email_idx: recipient_email_idx,
        domain_idx: domain_idx.0,
        timestamp_idx: timestamp_idx.0,
    };

    Ok(serde_json::to_string(&email_sender_input)?)
}

pub async fn generate_account_creation_input(email: &str, relayer_rand: &str) -> Result<String> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let circuit_input_params = circuit::CircuitInputParams::new(
        vec![],
        parsed_email.canonicalized_header.as_bytes().to_vec(),
        "".to_string(),
        vec_u8_to_bigint(parsed_email.clone().signature),
        vec_u8_to_bigint(parsed_email.clone().public_key),
        None,
        Some(1024),
        Some(64),
        Some(true),
    );
    let email_circuit_inputs = circuit::generate_circuit_inputs(circuit_input_params);

    let sender_email_idx = parsed_email.get_from_addr_idxes().unwrap();
    let domain_idx = parsed_email.get_email_domain_idxes().unwrap();
    // let subject_idx = parsed_email.get_subject_all_idxes().unwrap();
    let code_idx = parsed_email.get_invitation_code_idxes().unwrap();
    let timestamp_idx = parsed_email.get_timestamp_idxes().unwrap();

    let account_creation_input = AccountCreationInput {
        in_padded: email_circuit_inputs.in_padded,
        pubkey: email_circuit_inputs.pubkey,
        signature: email_circuit_inputs.signature,
        in_padded_len: email_circuit_inputs.in_len_padded_bytes,
        relayer_rand: relayer_rand.to_string(),
        sender_email_idx: sender_email_idx.0,
        code_idx: code_idx.0,
        domain_idx: domain_idx.0,
        timestamp_idx: timestamp_idx.0,
    };

    Ok(serde_json::to_string(&account_creation_input)?)
}

pub async fn generate_claim_input(
    email_address: &str,
    email_address_rand: &str,
    account_key: &str,
) -> Result<String> {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    let mut padded_email_addr_bytes = vec![];

    for byte in padded_email_address.padded_bytes.into_iter() {
        padded_email_addr_bytes.push(byte);
    }

    let claim_input = ClaimInput {
        email_addr: padded_email_addr_bytes,
        cm_rand: email_address_rand.to_string(),
        account_key: account_key.to_string(),
    };

    Ok(serde_json::to_string(&claim_input)?)
}

pub fn generate_account_creation_input_node(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let email = cx.argument::<JsString>(0)?.value(&mut cx);
    let relayer_rand = cx.argument::<JsString>(1)?.value(&mut cx);

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    let rt = runtime(&mut cx)?;

    rt.spawn(async move {
        let account_creation_input = generate_account_creation_input(&email, &relayer_rand).await;
        deferred.settle_with(&channel, move |mut cx| match account_creation_input {
            Ok(account_creation_input) => {
                let account_creation_input = cx.string(account_creation_input);
                Ok(account_creation_input)
            }
            Err(err) => cx.throw_error(format!("Could not generate email sender input: {}", err)),
        });
    });

    Ok(promise)
}

pub fn generate_email_sender_input_node(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let email = cx.argument::<JsString>(0)?.value(&mut cx);
    let account_key = cx.argument::<JsString>(1)?.value(&mut cx);

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    let rt = runtime(&mut cx)?;

    rt.spawn(async move {
        let email_sender_input = generate_email_sender_input(&email, &account_key).await;
        deferred.settle_with(&channel, move |mut cx| match email_sender_input {
            Ok(email_sender_input) => {
                let email_sender_input = cx.string(email_sender_input);
                Ok(email_sender_input)
            }
            Err(err) => cx.throw_error(format!("Could not generate email sender input: {}", err)),
        });
    });

    Ok(promise)
}
