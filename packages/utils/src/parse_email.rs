use itertools::Itertools;
// use cfdkim::*;
// use mail_auth::common::verify::VerifySignature;
// use mail_auth::trust_dns_resolver::proto::rr::dnssec::public_key;
use std::error::Error;
// use trust_dns_resolver::error::ResolveError;
// use mail_auth::Error;
use crate::statics::*;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use hex;
// use mail_auth::{AuthenticatedMessage, DkimOutput, DkimResult, Resolver};

use cfdkim::{canonicalize_signed_email, resolve_public_key, SignerBuilder};
use neon::prelude::*;
use rsa::{PublicKeyParts, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use slog;
use std::env;
// use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
// use trust_dns_resolver::proto::rr::{RData, RecordType};
// use trust_dns_resolver::AsyncResolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEmail {
    pub canonicalized_header: Vec<u8>,
    // pub signed_header: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    // pub dkim_domain: String,
}

pub async fn parse_email(raw_email: &str) -> Result<ParsedEmail> {
    let logger = slog::Logger::root(slog::Discard, slog::o!());
    let public_key = resolve_public_key(&logger, raw_email.as_bytes())
        .await
        .unwrap();
    let public_key = match public_key {
        cfdkim::DkimPublicKey::Rsa(pk) => pk,
        _ => panic!("not supportted public key type."),
    };
    let (canonicalized_header, _, signature_bytes) =
        canonicalize_signed_email(&raw_email.as_bytes()).unwrap();
    // let resolver = Resolver::new_cloudflare_tls()?;
    // let authenticated_message = AuthenticatedMessage::parse(raw_email.as_bytes())
    //     .ok_or_else(|| anyhow!("AuthenticatedMessage is not found."))?;

    // // Validate signature
    // let result = resolver.verify_dkim(&authenticated_message).await;
    // for s in result.iter() {
    //     if s.result() != &DkimResult::Pass {
    //         return Err(anyhow!("DKIM validation failed for {:?}.", s));
    //     }
    // }

    // // Extract the parsed + canonicalized headers along with the signed value for them
    // let (canonicalized_header, signed_header) = authenticated_message
    //     .get_canonicalized_header()
    //     .or_else(|err| Err(anyhow!("canonicalized header is not extracted: {}", err)))?;
    // let signature = result[0]
    //     .signature()
    //     .ok_or_else(|| anyhow!("signature is not found"))?;

    // let dkim_domain = signature.domain_key();
    // let public_key_str = get_public_key(&dkim_domain).await?;
    // let public_key = general_purpose::STANDARD.decode(&public_key_str)?;
    // public_key_str.as_bytes().to_vec();
    let parsed_email = ParsedEmail {
        canonicalized_header,
        signature: signature_bytes.into_iter().collect_vec(),
        public_key: public_key.n().to_bytes_be(),
    };
    Ok(parsed_email)
}

// async fn get_public_key(domain: &str) -> Result<String> {
//     let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
//     let response = resolver.lookup(domain, RecordType::TXT).await?;
//     // println!("Response: {:?}", response);
//     for record in response.iter() {
//         if let RData::TXT(ref txt) = *record {
//             let txt_data = txt.txt_data();
//             // Format txt data to convert from u8 to string
//             let data_bytes: Vec<u8> = txt_data.iter().flat_map(|b| b.as_ref()).cloned().collect();
//             let data = String::from_utf8_lossy(&data_bytes);
//             // println!("Data from txt record: {:?}", data);

//             if data.contains("k=rsa;") {
//                 let parts: Vec<_> = data.split("; ").collect();
//                 for part in parts {
//                     if part.starts_with("p=") {
//                         assert_eq!(part.strip_prefix("p=").unwrap(), part[2..].to_string());
//                         return Ok(part.strip_prefix("p=").unwrap().to_string());
//                     }
//                 }
//             }
//         }
//     }
//     return Err(anyhow!("RSA public key is not found."));
// }

pub fn parse_email_node(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw_email = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    let rt = runtime(&mut cx)?;

    rt.spawn(async move {
        let parsed_email = parse_email(&raw_email).await;
        deferred.settle_with(&channel, move |mut cx| {
            match parsed_email {
                // Resolve the promise with the release date
                Ok(parsed_email) => {
                    let obj = cx.empty_object();
                    let canonicalized_header =
                        cx.string(&String::from_utf8(parsed_email.canonicalized_header).unwrap());
                    obj.set(&mut cx, "canonicalizedHeader", canonicalized_header)?;
                    // let signed_header = cx.string(
                    //     "0x".to_string() + hex::encode(parsed_email.signed_header).as_str(),
                    // );
                    // obj.set(&mut cx, "signedHeader", signed_header)?;
                    let signature =
                        cx.string("0x".to_string() + hex::encode(parsed_email.signature).as_str());
                    obj.set(&mut cx, "signature", signature)?;

                    let public_key =
                        cx.string("0x".to_string() + hex::encode(parsed_email.public_key).as_str());
                    obj.set(&mut cx, "publicKey", public_key)?;
                    // let dkim_domain = cx.string(&parsed_email.dkim_domain);
                    // obj.set(&mut cx, "dkimDomain", dkim_domain)?;
                    Ok(obj)
                }

                // Reject the `Promise` if the version could not be found
                Err(err) => cx.throw_error(format!(
                    "Could not parse the raw email: {}",
                    err.to_string()
                )),
            }
        });
    });

    Ok(promise)
}

// pub fn extract_from(email: &str) -> Result<String, Box<dyn Error>> {
//     let mut from_addresses: Vec<String> = Vec::new();
//     let mut email_lines = email.lines();
//     while let Some(line) = email_lines.next() {
//         if line.starts_with("From:") {
//             let from_line = line;
//             let email_start = from_line.find('<');
//             let email_end = from_line.find('>');
//             if let (Some(start), Some(end)) = (email_start, email_end) {
//                 let from = &from_line[start + 1..end];
//                 println!("From email address: {}", from);
//                 from_addresses.push(from.to_string());
//             }
//         }
//     }

//     if !from_addresses.is_empty() {
//         return Ok(from_addresses.join(", "));
//     }
//     Err("Could not find from email address".into())
// }

// pub fn extract_subject(email: &str) -> Result<String, Box<dyn Error>> {
//     if let Some(subject_start) = email.find("Subject:") {
//         let subject_line_start = &email[subject_start..];
//         if let Some(subject_end) = subject_line_start.find("\r\n") {
//             let subject_line = &subject_line_start[..subject_end];
//             println!("Subject line: {}", subject_line);
//             return Ok(subject_line.to_string());
//         }
//     }
//     Err("Could not find subject".into())
// }

// pub fn extract_message_id(email: &str) -> Result<String, Box<dyn Error>> {
//     if let Some(message_id_start) = email.find("Message-ID:") {
//         let message_id_line_start = &email[message_id_start..];
//         if let Some(message_id_end) = message_id_line_start.find("\r\n") {
//             let message_id_line = &message_id_line_start[..message_id_end];
//             let email_start = message_id_line.find('<');
//             let email_end = message_id_line.find('>');
//             if let (Some(start), Some(end)) = (email_start, email_end) {
//                 let message_id = &message_id_line[start + 1..end];
//                 println!("message_id value: {}", message_id);
//                 return Ok(message_id.to_string());
//             }
//         }
//     }
//     Err("Could not find message_id value".into())
// }
