#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]

use crate::*;
use chrono::{DateTime, Local};
use ethers::{
    abi::{self, ParamType, Token},
    types::{Address, U256},
};
use relayer_utils::*;
use serde_json::Value;

use ::serde::Deserialize;

use std::path::Path;

use tokio::fs::{read_to_string, remove_file};

const DOMAIN_FIELDS: usize = 9;
const SUBJECT_FIELDS: usize = 17;
const EMAIL_ADDR_FIELDS: usize = 9;

pub enum Asset {
    ERC20 {
        token_addr: Address,
        token_name: String,
        amount: U256,
        amount_str: String,
    },
    ERC721 {
        token_addr: Address,
        token_name: String,
        token_id: U256,
        token_uri: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProverRes {
    proof: ProofJson,
    pub_signals: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProofJson {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
}

impl ProofJson {
    pub fn to_eth_bytes(&self) -> Result<Bytes> {
        let pi_a = Token::FixedArray(vec![
            Token::Uint(U256::from_dec_str(self.pi_a[0].as_str())?),
            Token::Uint(U256::from_dec_str(self.pi_a[1].as_str())?),
        ]);
        let pi_b = Token::FixedArray(vec![
            Token::FixedArray(vec![
                Token::Uint(U256::from_dec_str(self.pi_b[0][1].as_str())?),
                Token::Uint(U256::from_dec_str(self.pi_b[0][0].as_str())?),
            ]),
            Token::FixedArray(vec![
                Token::Uint(U256::from_dec_str(self.pi_b[1][1].as_str())?),
                Token::Uint(U256::from_dec_str(self.pi_b[1][0].as_str())?),
            ]),
        ]);
        let pi_c = Token::FixedArray(vec![
            Token::Uint(U256::from_dec_str(self.pi_c[0].as_str())?),
            Token::Uint(U256::from_dec_str(self.pi_c[1].as_str())?),
        ]);
        Ok(Bytes::from(abi::encode(&[pi_a, pi_b, pi_c])))
    }
}

pub async fn compute_psi_point(
    circuits_dir_path: &Path,
    email_addr: &str,
    rand: &str,
) -> Result<Point> {
    info!(LOG, "compute_psi_point, path {:?}", circuits_dir_path);
    let input_file_name = PathBuf::new()
        .join(INPUT_FILES_DIR.get().unwrap())
        .join(email_addr.to_string() + "psi" + ".json");
    info!(LOG, "input_file_name: {:?}", input_file_name);

    let command_str = format!(
        "--cwd {} psi-step1 --email-addr {} --client-rand {} --output {}",
        circuits_dir_path.to_str().unwrap(),
        email_addr,
        rand,
        input_file_name.to_str().unwrap()
    );

    info!(LOG, "command_str: {}", command_str);

    let mut proc = tokio::process::Command::new("yarn")
        .args(command_str.split_whitespace())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let output = proc.wait_with_output().await?;
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let status = proc.wait().await?;
    info!(LOG, "status: {:?}", status);
    assert!(status.success());

    let result = read_to_string(&input_file_name).await?;
    remove_file(input_file_name).await?;

    let point: Point = serde_json::from_str(&result)?;

    Ok(point)
}

pub fn calculate_addr_commitment(email_address: &str, rand: Fr) -> Fr {
    let padded_email_address = PaddedEmailAddr::from_email_addr(email_address);
    padded_email_address.to_commitment(&rand).unwrap()
}

#[named]
pub async fn generate_proof(
    input: &str,
    request: &str,
    address: &str,
) -> Result<(Bytes, Vec<U256>)> {
    let client = reqwest::Client::new();
    info!(LOG, "prover input {}", input; "func" => function_name!());
    let res = client
        .post(format!("{}/prove/{}", address, request))
        .json(&serde_json::json!({ "input": input }))
        .send()
        .await?
        .error_for_status()?;
    let res_json = res.json::<ProverRes>().await?;
    info!(LOG, "prover response {:?}", res_json; "func" => function_name!());
    let proof = res_json.proof.to_eth_bytes()?;
    let pub_signals = res_json
        .pub_signals
        .into_iter()
        .map(|str| U256::from_dec_str(&str).expect("pub signal should be u256"))
        .collect();
    Ok((proof, pub_signals))
}

pub fn get_psi_point_bytes(x: U256, y: U256) -> Bytes {
    Bytes::from(abi::encode(&[Token::Uint(x), Token::Uint(y)]))
}

pub fn now() -> i64 {
    let dt: DateTime<Local> = Local::now();
    dt.timestamp()
}

pub fn derive_relayer_rand(private_key: &str) -> Result<RelayerRand> {
    let mut seed = hex::decode(&private_key[2..])?;
    seed.append(&mut b"EMAIL WALLET RELAYER RAND".to_vec());
    Ok(RelayerRand::new_from_seed(&seed)?)
}

pub async fn search_user_assets(email_addr: &str) -> Result<Vec<Asset>> {
    let claims = DB.get_claims_by_email_addr(email_addr).await?;
    let _is_for_nft_demo = false;
    let mut assets = vec![];
    for claim in claims {
        if claim.is_fund {
            let unclaim_fund = CLIENT.query_unclaimed_fund(claim.id).await?;
            let token_decimal = CLIENT
                .query_decimals_of_erc20_address(unclaim_fund.token_addr)
                .await?;
            let amount =
                uint_to_decimal_string(unclaim_fund.amount.as_u128(), token_decimal as usize);
            let name = CLIENT.query_token_name(unclaim_fund.token_addr).await?;
            assets.push(Asset::ERC20 {
                token_addr: unclaim_fund.token_addr,
                token_name: name,
                amount: unclaim_fund.amount,
                amount_str: amount,
            });
            continue;
        }
        let unclaimed_state = CLIENT.query_unclaimed_state(claim.id).await?;
        if unclaimed_state.extension_addr
            != CLIENT.query_default_extension_for_command("NFT").await?
        {
            continue;
        }
        let (nft_addr, nft_id, nft_name, nft_uri) = get_nft_info(&unclaimed_state.state).await?;
        assets.push(Asset::ERC721 {
            token_addr: nft_addr,
            token_name: nft_name,
            token_id: nft_id,
            token_uri: nft_uri,
        });
    }
    Ok(assets)
}

pub async fn get_nft_info(state: &[u8]) -> Result<(Address, U256, String, String)> {
    let decoded = abi::decode(&[ParamType::Address, ParamType::Uint(256)], &state)?;
    let nft_addr = decoded[0].clone().into_address().unwrap();
    let nft_id = decoded[1].clone().into_uint().unwrap();
    let nft_name = CLIENT.query_nft_name_of_address(nft_addr).await?;
    let nft_uri = CLIENT
        .query_erc721_token_uri_of_token(nft_addr, nft_id)
        .await?;
    Ok((nft_addr, nft_id, nft_name, nft_uri))
}

pub async fn download_img_from_uri(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await.map_err(|e| anyhow!(e))?;
    Ok(bytes.to_vec())
}

pub async fn generate_asset_list_body(
    assets: &[Asset],
    mut assets_msgs: Vec<String>,
) -> Result<(String, Vec<Value>)> {
    let mut images = vec![None; assets_msgs.len()];
    for asset in assets {
        match asset {
            Asset::ERC20 {
                token_addr: _,
                token_name,
                amount: _,
                amount_str,
            } => {
                let mut token_name = token_name.clone();
                if token_name == "WETH" {
                    token_name = "ETH".to_string();
                }
                assets_msgs.push(format!("ERC20: {} {}", amount_str, token_name));
                images.push(None);
            }
            Asset::ERC721 {
                token_addr: _,
                token_name,
                token_id,
                token_uri,
            } => {
                let json_uri: Value = serde_json::from_str(
                    &String::from_utf8(
                        base64::decode(token_uri.split(",").nth(1).expect("Invalid base64 string"))
                            .expect("Failed to decode base64 string"),
                    )
                    .expect("Invalid UTF-8 sequence"),
                )
                .expect("Failed to parse JSON");
                let img = json_uri.clone()["image"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                images.push(Some(img));
                assets_msgs.push(format!("NFT: ID {} of {}", token_id, token_name));
            }
        }
    }
    let mut assets_list_plain = String::new();
    for asset_msg in assets_msgs.iter() {
        assets_list_plain.push_str(&format!("{}\n", asset_msg));
    }
    let mut assets_list_html = vec![];
    println!("assets_msgs: {:?}", assets_msgs);
    for (idx, (asset_msg, image)) in assets_msgs.iter().zip(images.into_iter()).enumerate() {
        if image.is_some() {
            let value = serde_json::json!({
                "msg": asset_msg,
                "img": image.unwrap(),
                "is_img": true,
            });
            assets_list_html.push(value);
        }
        let value = serde_json::json!({
            "msg": asset_msg,
            "img": "",
            "is_img": false,
        });
        assets_list_html.push(value);
    }
    Ok((assets_list_plain, assets_list_html))
}
