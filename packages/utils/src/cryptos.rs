use std::convert::TryInto;
use std::str::FromStr;

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

pub const MAX_EMAIL_ADDR_BYTES: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct RelayerRand(Fr);

impl RelayerRand {
    pub fn new<R: RngCore>(mut r: R) -> Self {
        Self(Fr::random(&mut r))
    }

    pub fn hash(&self) -> Result<Fr, PoseidonError> {
        poseidon_fields(&[self.0.clone()])
    }
}

#[derive(Debug, Clone)]
pub struct PaddedEmailAddr {
    pub padded_bytes: Vec<u8>,
    pub email_addr_len: usize,
}

impl PaddedEmailAddr {
    pub fn from_email_addr(email_addr: &str) -> Self {
        let email_addr_len = email_addr.as_bytes().len();
        // let mut padded_bytes = email_addr.as_bytes().to_vec();
        // padded_bytes.append(&mut vec![0; MAX_EMAIL_ADDR_BYTES - email_addr_len]);
        let padded_bytes = pad_string(email_addr, MAX_EMAIL_ADDR_BYTES);
        Self {
            padded_bytes,
            email_addr_len,
        }
    }

    pub fn to_email_addr_fields(&self) -> Vec<Fr> {
        bytes2fields(&self.padded_bytes)
    }

    pub fn to_pointer(&self, relayer_rand: &RelayerRand) -> Result<Fr, PoseidonError> {
        self.to_commitment(&relayer_rand.0)
    }

    pub fn to_commitment(&self, rand: &Fr) -> Result<Fr, PoseidonError> {
        let mut inputs = vec![rand.clone()];
        inputs.append(&mut self.to_email_addr_fields());
        poseidon_fields(&inputs)
    }

    pub fn to_commitment_with_signature(&self, signature: &[u8]) -> Result<Fr, PoseidonError> {
        let mut inputs = bytes_chunk_fields(signature, 121, 2);
        inputs.push(Fr::one());
        let cm_rand = poseidon_fields(&inputs)?;
        poseidon_fields(&[vec![cm_rand], self.to_email_addr_fields()].concat())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ViewingKey(Fr);

impl ViewingKey {
    pub fn new<R: RngCore>(rng: R) -> Self {
        Self(Fr::random(rng))
    }

    pub fn to_commitment(
        &self,
        email_addr: &PaddedEmailAddr,
        relayer_rand_hash: &Fr,
    ) -> Result<Fr, PoseidonError> {
        let mut inputs = vec![self.0.clone()];
        inputs.append(&mut email_addr.to_email_addr_fields());
        inputs.push(relayer_rand_hash.clone());
        poseidon_fields(&inputs)
    }

    pub fn to_wallet_salt(&self) -> Result<WalletSalt, PoseidonError> {
        let field = poseidon_fields(&[self.0.clone(), Fr::zero()])?;
        Ok(WalletSalt(field))
    }

    pub fn to_ext_account_salt(&self) -> Result<ExtAccountSalt, PoseidonError> {
        let field = poseidon_fields(&[self.0.clone(), Fr::one()])?;
        Ok(ExtAccountSalt(field))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WalletSalt(Fr);

#[derive(Debug, Clone, Copy)]
pub struct ExtAccountSalt(Fr);

fn bytes2fields(bytes: &[u8]) -> Vec<Fr> {
    bytes
        .chunks(31)
        .map(|bytes| {
            let mut bytes32 = [0; 32];
            bytes32[0..bytes.len()].clone_from_slice(bytes);
            Fr::from_bytes(&bytes32).expect("fail to convert bytes to a field value")
        })
        .collect_vec()
}

fn bytes_chunk_fields(bytes: &[u8], chunk_size: usize, num_chunk_in_field: usize) -> Vec<Fr> {
    let bits = bytes
        .into_iter()
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
            for (i, word) in words.iter().enumerate() {
                input += coeff * word;
                coeff *= offset;
            }
            input
        })
        .collect_vec();
    fields
}

/// `public_key_n` is little endian.
pub fn public_key_hash(public_key_n: &[u8]) -> Result<Fr, PoseidonError> {
    let inputs = bytes_chunk_fields(public_key_n, 121, 2);
    poseidon_fields(&inputs)
}

/// `signature` is little endian.
pub fn email_nullifier(signature: &[u8]) -> Result<Fr, PoseidonError> {
    let inputs = bytes_chunk_fields(signature, 121, 2);
    let sign_rand = poseidon_fields(&inputs)?;
    poseidon_fields(&[sign_rand])
}

pub fn gen_relayer_rand_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut rng = OsRng;
    let relayer_rand = RelayerRand::new(&mut rng);
    let relayer_rand_str = field2str(&relayer_rand.0);
    Ok(cx.string(relayer_rand_str))
}

pub fn relayer_rand_hash_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let relayer_rand = cx.argument::<JsString>(0)?.value(&mut cx);
    let relayer_rand = str2field_node(&mut cx, &relayer_rand)?;
    let relayer_rand_hash = match RelayerRand(relayer_rand).hash() {
        Ok(fr) => fr,
        Err(e) => return cx.throw_error(&format!("RelayerRand hash failed: {}", e)),
    };
    let relayer_rand_hash_str = field2str(&relayer_rand_hash);
    Ok(cx.string(relayer_rand_hash_str))
}

// pub fn pad_string_node(mut cx: FunctionContext) -> JsResult<JsArray> {
//     let string = cx.argument::<JsString>(0)?.value(&mut cx);
//     let padded_bytes_size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
//     let padded_bytes = JsArray::new(&mut cx, padded_bytes_size as u32);
//     for (idx, byte) in string.as_bytes().into_iter().enumerate() {
//         let js_byte = cx.number(*byte);
//         padded_bytes.set(&mut cx, idx as u32, js_byte)?;
//     }
//     for idx in string.len()..padded_bytes_size {
//         let js_byte = cx.number(0);
//         padded_bytes.set(&mut cx, idx as u32, js_byte)?;
//     }
//     Ok(padded_bytes)
// }

pub fn pad_email_addr_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let email_addr = cx.argument::<JsString>(0)?.value(&mut cx);
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&email_addr);
    let padded_email_addr_bytes =
        JsArray::new(&mut cx, padded_email_addr.padded_bytes.len() as u32);
    for (idx, byte) in padded_email_addr.padded_bytes.into_iter().enumerate() {
        let js_byte = cx.number(byte);
        padded_email_addr_bytes.set(&mut cx, idx as u32, js_byte)?;
    }
    Ok(padded_email_addr_bytes)
}

pub fn email_addr_pointer_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let email_addr = cx.argument::<JsString>(0)?.value(&mut cx);
    let relayer_rand = cx.argument::<JsString>(1)?.value(&mut cx);
    let relayer_rand = str2field_node(&mut cx, &relayer_rand)?;
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&email_addr);
    let email_addr_pointer = match padded_email_addr.to_pointer(&RelayerRand(relayer_rand)) {
        Ok(fr) => fr,
        Err(e) => return cx.throw_error(&format!("EmailAddrPointer failed: {}", e)),
    };
    let email_addr_pointer_str = field2str(&email_addr_pointer);
    Ok(cx.string(email_addr_pointer_str))
}

pub fn email_addr_commit_rand_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut rng = OsRng;
    let commit_rand = Fr::random(&mut rng);
    let commit_rand_str = field2str(&commit_rand);
    Ok(cx.string(commit_rand_str))
}

pub fn email_addr_commit_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let email_addr = cx.argument::<JsString>(0)?.value(&mut cx);
    let rand = cx.argument::<JsString>(1)?.value(&mut cx);
    let rand = str2field_node(&mut cx, &rand)?;
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&email_addr);
    let email_addr_commit = match padded_email_addr.to_commitment(&rand) {
        Ok(fr) => fr,
        Err(e) => return cx.throw_error(&format!("EmailAddrCommit failed: {}", e)),
    };
    let email_addr_commit_str = field2str(&email_addr_commit);
    Ok(cx.string(email_addr_commit_str))
}

pub fn email_addr_commit_with_signature_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let email_addr = cx.argument::<JsString>(0)?.value(&mut cx);
    let signature = cx.argument::<JsString>(1)?.value(&mut cx);
    let mut signature = match hex::decode(&signature[2..]) {
        Ok(bytes) => bytes,
        Err(e) => return cx.throw_error(&format!("signature is an invalid hex string: {}", e)),
    };
    signature.reverse();
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&email_addr);
    let email_addr_commit = match padded_email_addr.to_commitment_with_signature(&signature) {
        Ok(fr) => fr,
        Err(e) => return cx.throw_error(&format!("EmailAddrCommit failed: {}", e)),
    };
    let email_addr_commit_str = field2str(&email_addr_commit);
    Ok(cx.string(email_addr_commit_str))
}

pub fn gen_viewing_key_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut rng = OsRng;
    let viewing_key = ViewingKey::new(&mut rng);
    let viewing_key_str = field2str(&viewing_key.0);
    Ok(cx.string(viewing_key_str))
}

pub fn viewing_key_commit_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let viewing_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let email_addr = cx.argument::<JsString>(1)?.value(&mut cx);
    let relayer_rand_hash = cx.argument::<JsString>(2)?.value(&mut cx);
    let viewing_key = str2field_node(&mut cx, &viewing_key)?;
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&email_addr);
    let relayer_rand_hash = str2field_node(&mut cx, &relayer_rand_hash)?;
    let viewing_key_commit =
        match ViewingKey(viewing_key).to_commitment(&padded_email_addr, &relayer_rand_hash) {
            Ok(fr) => fr,
            Err(e) => return cx.throw_error(&format!("ViewingKeyCommit failed: {}", e)),
        };
    let viewing_key_commit_str = field2str(&viewing_key_commit);
    Ok(cx.string(viewing_key_commit_str))
}

pub fn wallet_salt_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let viewing_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let viewing_key = str2field_node(&mut cx, &viewing_key)?;
    let wallet_salt = match ViewingKey(viewing_key).to_wallet_salt() {
        Ok(wallet_salt) => wallet_salt,
        Err(e) => return cx.throw_error(&format!("WalletSalt failed: {}", e)),
    };
    let wallet_salt_str = field2str(&wallet_salt.0);
    Ok(cx.string(wallet_salt_str))
}

pub fn ext_account_salt_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let viewing_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let viewing_key = str2field_node(&mut cx, &viewing_key)?;
    let ext_account_salt = match ViewingKey(viewing_key).to_ext_account_salt() {
        Ok(ext_account_salt) => ext_account_salt,
        Err(e) => return cx.throw_error(&format!("ExtAccountSalt failed: {}", e)),
    };
    let ext_account_salt_str = field2str(&ext_account_salt.0);
    Ok(cx.string(ext_account_salt_str))
}

pub fn public_key_hash_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let public_key_n = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut public_key_n = match hex::decode(&public_key_n[2..]) {
        Ok(bytes) => bytes,
        Err(e) => return cx.throw_error(&format!("public_key_n is an invalid hex string: {}", e)),
    };
    public_key_n.reverse();
    let hash_field = match public_key_hash(&public_key_n) {
        Ok(hash_field) => hash_field,
        Err(e) => return cx.throw_error(&format!("public_key_hash failed: {}", e)),
    };
    let hash_str = field2str(&hash_field);
    Ok(cx.string(hash_str))
}

pub fn email_nullifier_node(mut cx: FunctionContext) -> JsResult<JsString> {
    let signature = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut signature = match hex::decode(&signature[2..]) {
        Ok(bytes) => bytes,
        Err(e) => return cx.throw_error(&format!("signature is an invalid hex string: {}", e)),
    };
    signature.reverse();
    let nullifier = match email_nullifier(&signature) {
        Ok(nullifier) => nullifier,
        Err(e) => return cx.throw_error(&format!("email_nullifier failed: {}", e)),
    };
    let nullifier_str = field2str(&nullifier);
    Ok(cx.string(nullifier_str))
}

fn str2field_node(cx: &mut FunctionContext, input_strs: &str) -> NeonResult<Fr> {
    if &input_strs[0..2] != "0x" {
        return cx.throw_error(&format!(
            "the input string {} must be hex string with 0x prefix",
            &input_strs
        ));
    }
    let mut bytes = match hex::decode(&input_strs[2..]) {
        Ok(bytes) => bytes,
        Err(e) => {
            return cx.throw_error(&format!(
                "the input string {} is invalid hex: {}",
                &input_strs, e
            ))
        }
    };
    bytes.reverse();
    if bytes.len() != 32 {
        return cx.throw_error(&format!(
            "the input string {} must be 32 bytes but is {} bytes",
            &input_strs,
            bytes.len()
        ));
    }
    let bytes: [u8; 32] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(e) => return cx.throw_error(&format!("the bytes {:?} is not valid 32 bytes", e)),
    };
    let field = Fr::from_bytes(&bytes).expect("fail to convert bytes to a field value");
    Ok(field)
}

fn field2str(field: &Fr) -> String {
    format!("{:?}", field)
}
