use anyhow::{anyhow, Result};

use base64::prelude::*;

use ethers::{
    abi::{self, ParamType},
    types::{Address, U256},
};

use relayer::CLIENT;
use relayer::*;
use reqwest;
use serde_json::{value, Value};
use slog::info;

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
    // let mut image_names = vec![None; assets_msgs.len()];
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
                // image_names.push(None);
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
                // info!(LOG, "img bytes {:?}", img);
                // let image_name = format!("nft_{}_id_{}", token_name, token_id);
                // image_names.push(Some(image_name));
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
    // let mut attachments = vec![];
    // assets_list_html.push_str("<ul>\n");
    for (idx, (asset_msg, image)) in assets_msgs.iter().zip(images.into_iter()).enumerate() {
        // let image_str = image
        //     .as_ref()
        //     .map(|img| format!("data:image/png;base64,{}", BASE64_STANDARD.encode(img)));
        // let image_str = match image_name {
        //     Some(name) => format!("cid:{}", name),
        //     None => String::new(),
        // };
        if image.is_some() {
            let value = serde_json::json!({
                "msg": asset_msg,
                "img": image.unwrap(),
                "is_img": true,
            });
            assets_list_html.push(value);
        }

        // if let Some(img) = image_bytes {
        //     let email_attachment = EmailAttachment {
        //         inline_id: idx.to_string(),
        //         content_type: "image/png".to_string(),
        //         contents: img,
        //     };
        //     attachments.push(email_attachment);
        // }
        // if let Some(image) = image {
        //     let img_base64 = BASE64_STANDARD.encode(image);

        //     assets_list_html.push((
        //         asset_msg.clone(),
        //         format!("data:image/png;base64,{}", img_base64),
        //     ));
        //     // assets_list_html.push_str(&format!(
        //     //     "<li>{}\n<img src=\"data:image/png;base64,{}\"/>\n</li>",
        //     //     asset_msg, img_base64
        //     // ));
        // }
        // assets_list_html.push_str(&format!("<li>{}</li>\n", asset_msg));
    }
    // assets_list_html.push_str("</ul>\n");
    Ok((assets_list_plain, assets_list_html))
}
