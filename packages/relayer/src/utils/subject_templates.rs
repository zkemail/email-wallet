#![allow(clippy::upper_case_acronyms)]

use crate::*;

use ethers::abi::{self, Token};
use ethers::types::{Address, Bytes, I256, U256};
use regex::Regex;

#[derive(Debug, Clone)]
pub enum TemplateValue {
    TokenAmount {
        token_name: String,
        amount: String,
    },
    Amount(String),
    String(String),
    Uint(U256),
    Int(I256),
    Address(Address),
    Recipient {
        is_email: bool,
        email_addr: Option<String>,
        eth_addr: Option<Address>,
    },
    Fixed(String),
}

impl TemplateValue {
    #[named]
    pub fn abi_encode(&self, amount_decimal_size: Option<u8>) -> Result<Bytes> {
        match self {
            Self::TokenAmount { token_name, amount } => {
                let amount_u256 = Self::amount_to_uint(amount, amount_decimal_size.unwrap());
                info!(LOG, "amount_u256: {}", amount_u256; "func" => function_name!());
                Ok(Bytes::from(abi::encode(&[
                    Token::Uint(amount_u256),
                    Token::String(token_name.clone()),
                ])))
            }
            Self::Amount(amount) => {
                let amount_u256 = Self::amount_to_uint(amount, amount_decimal_size.unwrap());
                Ok(Bytes::from(abi::encode(&[Token::Uint(amount_u256)])))
            }
            Self::String(string) => Ok(Bytes::from(abi::encode(&[Token::String(string.clone())]))),
            Self::Uint(uint) => Ok(Bytes::from(abi::encode(&[Token::Uint(*uint)]))),
            Self::Int(int) => Ok(Bytes::from(abi::encode(&[Token::Int(int.into_raw())]))),
            Self::Address(address) => Ok(Bytes::from(abi::encode(&[Token::Address(*address)]))),
            Self::Recipient {
                is_email,
                email_addr,
                eth_addr,
            } => {
                if *is_email {
                    Ok(Bytes::default())
                } else {
                    Ok(Bytes::from(abi::encode(&[Token::Address(
                        eth_addr.unwrap(),
                    )])))
                }
            }
            Self::Fixed(string) => Err(anyhow!("Fixed value must not be passed to abi_encode")),
        }
    }

    pub fn amount_to_uint(amount_str: &str, decimal_size: u8) -> U256 {
        let decimal_size = decimal_size as usize;
        let dot = Regex::new("\\.").unwrap().find(amount_str);
        let (before_dot_str, mut after_dot_str) = match dot {
            Some(dot_match) => (
                amount_str[0..dot_match.start()].to_string(),
                amount_str[dot_match.end()..].to_string(),
            ),
            None => (amount_str.to_string(), "".to_string()),
        };
        assert!(after_dot_str.len() <= decimal_size);
        let num_leading_zeros = decimal_size - after_dot_str.len();
        after_dot_str.push_str(&"0".repeat(num_leading_zeros));
        U256::from_dec_str(&(before_dot_str + &after_dot_str))
            .expect("composed amount string is not valid decimal")
    }
}

pub async fn extract_command_from_subject(
    subject: &str,
    chain_client: &Arc<ChainClient>,
    wallet_salt: &WalletSalt,
) -> Result<(String, usize)> {
    for word in subject.split_whitespace() {
        let position = subject.find(word).unwrap();
        if word == SEND_COMMAND
            || word == EXECUTE_COMMAND
            || word == INSTALL_COMMAND
            || word == UNINSTALL_COMMAND
            || word == EXIT_COMMAND
            || word == DKIM_COMMAND
        {
            return Ok((word.to_string(), position));
        } else if chain_client
            .query_user_extension_for_command(wallet_salt, word)
            .await?
            != Address::zero()
        {
            return Ok((word.to_string(), position));
        }
    }
    Err(anyhow!("No command found"))
}

pub fn extract_template_vals_send(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![
        SEND_COMMAND.to_string(),
        "{tokenAmount}".to_string(),
        "to".to_string(),
        "{recipient}".to_string(),
    ];
    extract_template_vals(input, templates)
}

pub fn extract_template_vals_execute(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![EXECUTE_COMMAND.to_string(), "{string}".to_string()];
    let vals = extract_template_vals(input, templates)?;
    if let TemplateValue::String(hex) = &vals[0] {
        let hex_match = Regex::new("0x[0-9a-fA-F]+")
            .unwrap()
            .find(hex)
            .ok_or(anyhow!(
                "No hex found in the string of execute command subject"
            ))?;
        if hex_match.start() != 0 || hex_match.end() != hex.len() {
            return Err(anyhow!("Hex must be the whole word"));
        }
    } else {
        return Err(anyhow!("No string found in the execute command subject"));
    }
    Ok(vals)
}

pub fn extract_template_vals_install(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![
        INSTALL_COMMAND.to_string(),
        "extension".to_string(),
        "{string}".to_string(),
    ];
    extract_template_vals(input, templates)
}

pub fn extract_template_vals_uninstall(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![
        UNINSTALL_COMMAND.to_string(),
        "extension".to_string(),
        "{string}".to_string(),
    ];
    extract_template_vals(input, templates)
}

pub fn extract_template_vals_exit(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![
        EXIT_COMMAND.to_string(),
        "Email".to_string(),
        "Wallet".to_string(),
        "Change".to_string(),
        "Ownership".to_string(),
        "to".to_string(),
        "{address}".to_string(),
    ];
    extract_template_vals(input, templates)
}

pub fn extract_template_vals_dkim(input: &str) -> Result<Vec<TemplateValue>> {
    let templates = vec![
        DKIM_COMMAND.to_string(),
        "registry".to_string(),
        "set".to_string(),
        "to".to_string(),
        "{address}".to_string(),
    ];
    extract_template_vals(input, templates)
}

pub fn extract_template_vals_and_idx(
    input: &str,
    templates_array: Vec<Vec<String>>,
) -> Result<(Option<usize>, Vec<TemplateValue>)> {
    for (idx, templates) in templates_array.into_iter().enumerate() {
        let template_vals = extract_template_vals(input, templates);
        match template_vals {
            Ok(vals) => {
                return Ok((Some(idx), vals));
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok((None, Vec::new()))
}

fn extract_template_vals(input: &str, templates: Vec<String>) -> Result<Vec<TemplateValue>> {
    let input_decomposed: Vec<&str> = input.split(' ').collect();
    let mut template_vals = Vec::new();
    let mut input_idx = 0;
    for template in templates.iter() {
        match template.as_str() {
            "{tokenAmount}" => {
                let amount_match = Regex::new(AMOUNT_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No amount found"))?;
                if amount_match.start() != 0
                    || amount_match.end() != input_decomposed[input_idx].len()
                {
                    return Err(anyhow!("Amount must be the whole word"));
                }
                let amount = amount_match.as_str().to_string();
                let token_name_match = Regex::new(TOKEN_NAME_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx + 1])
                    .ok_or(anyhow!("No token name found"))?;
                if token_name_match.start() != 0
                    || token_name_match.end() != input_decomposed[input_idx + 1].len()
                {
                    return Err(anyhow!("Token name must be the whole word"));
                }
                let token_name = token_name_match.as_str().to_string();
                template_vals.push(TemplateValue::TokenAmount { token_name, amount });
                input_idx += 2;
            }
            "{amount}" => {
                let amount_match = Regex::new(AMOUNT_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No amount found"))?;
                if amount_match.start() != 0
                    || amount_match.end() != input_decomposed[input_idx].len()
                {
                    return Err(anyhow!("Amount must be the whole word"));
                }
                let amount = amount_match.as_str().to_string();
                template_vals.push(TemplateValue::Amount(amount));
                input_idx += 1;
            }
            "{string}" => {
                let string_match = Regex::new(STRING_RGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No string found"))?;
                if string_match.start() != 0
                    || string_match.end() != input_decomposed[input_idx].len()
                {
                    return Err(anyhow!("String must be the whole word"));
                }
                let string = string_match.as_str().to_string();
                template_vals.push(TemplateValue::String(string));
                input_idx += 1;
            }
            "{uint}" => {
                let uint_match = Regex::new(UINT_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No uint found"))?;
                if uint_match.start() != 0 || uint_match.end() != input_decomposed[input_idx].len()
                {
                    return Err(anyhow!("Uint must be the whole word"));
                }
                let uint = U256::from_dec_str(uint_match.as_str()).unwrap();
                template_vals.push(TemplateValue::Uint(uint));
                input_idx += 1;
            }
            "{int}" => {
                let int_match = Regex::new(INT_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No int found"))?;
                if int_match.start() != 0 || int_match.end() != input_decomposed[input_idx].len() {
                    return Err(anyhow!("Int must be the whole word"));
                }
                let int_str = int_match.as_str();
                let int = I256::from_dec_str(int_match.as_str()).unwrap();
                template_vals.push(TemplateValue::Int(int));
                input_idx += 1;
            }
            "{address}" => {
                let address_match = Regex::new(ETH_ADDR_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx])
                    .ok_or(anyhow!("No address found"))?;
                if address_match.start() != 0
                    || address_match.end() != input_decomposed[input_idx].len()
                {
                    return Err(anyhow!("Address must be the whole word"));
                }
                let address = address_match.as_str().parse::<Address>().unwrap();
                template_vals.push(TemplateValue::Address(address));
                input_idx += 1;
            }
            "{recipient}" => {
                let email_addr_match = Regex::new(EMAIL_ADDR_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx]);
                let eth_addr_match = Regex::new(ETH_ADDR_REGEX)
                    .unwrap()
                    .find(input_decomposed[input_idx]);
                let is_email = if let Some(email_addr_match) = email_addr_match {
                    if email_addr_match.start() != 0
                        || email_addr_match.end() != input_decomposed[input_idx].len()
                    {
                        return Err(anyhow!("Email address must be the whole word"));
                    }
                    true
                } else if let Some(eth_addr_match) = eth_addr_match {
                    if eth_addr_match.start() != 0
                        || eth_addr_match.end() != input_decomposed[input_idx].len()
                    {
                        return Err(anyhow!("Eth address must be the whole word"));
                    }
                    false
                } else {
                    return Err(anyhow!("No recipient found"));
                };
                let (email_addr, eth_addr) = if is_email {
                    (Some(email_addr_match.unwrap().as_str().to_string()), None)
                } else {
                    (
                        None,
                        Some(eth_addr_match.unwrap().as_str().parse::<Address>().unwrap()),
                    )
                };
                template_vals.push(TemplateValue::Recipient {
                    is_email,
                    email_addr,
                    eth_addr,
                });
                input_idx += 1;
            }
            _ => {
                input_idx += 1;
            }
        }
    }
    if input_idx != input_decomposed.len() {
        return Err(anyhow!("Input is not fully consumed"));
    }
    Ok(template_vals)
}

// Generated by Github Copilot!
pub fn uint_to_decimal_string(amount: u128, decimal: usize) -> String {
    // Convert amount to string in wei format (no decimals)
    let amount_str = amount.to_string();
    let amount_length = amount_str.len();

    // Create result vector with max length
    // If less than 18 decimals, then 2 extra for "0.", otherwise one extra for "."
    let mut result = vec![
        '0';
        if amount_length > decimal {
            amount_length + 1
        } else {
            decimal + 2
        }
    ];
    let result_length = result.len();

    // Difference between result and amount array index when copying
    // If more than 18, then 1 index diff for ".", otherwise actual diff in length
    let mut delta = if amount_length > decimal {
        1
    } else {
        result_length - amount_length
    };

    // Boolean to indicate if we found a non-zero digit when scanning from last to first index
    let mut found_non_zero_decimal = false;

    let mut actual_result_len = 0;

    // In each iteration we fill one index of result array (starting from end)
    for i in (0..result_length).rev() {
        // Check if we have reached the index where we need to add decimal point
        if i == result_length - decimal - 1 {
            // No need to add "." if there was no value in decimal places
            if found_non_zero_decimal {
                result[i] = '.';
                actual_result_len += 1;
            }
            // Set delta to 0, as we have already added decimal point (only for amount_length > 18)
            delta = 0;
        }
        // If amountLength < 18 and we have copied everything, fill zeros
        else if amount_length <= decimal && i < result_length - amount_length {
            result[i] = '0';
            actual_result_len += 1;
        }
        // If non-zero decimal is found, or decimal point inserted (delta == 0), copy from amount array
        else if found_non_zero_decimal || delta == 0 {
            result[i] = amount_str.chars().nth(i - delta).unwrap();
            actual_result_len += 1;
        }
        // If we find non-zero decimal for the first time (trailing zeros are skipped)
        else if amount_str.chars().nth(i - delta).unwrap() != '0' {
            result[i] = amount_str.chars().nth(i - delta).unwrap();
            actual_result_len += 1;
            found_non_zero_decimal = true;
        }
    }

    // Create final result string with correct length
    let compact_result: String = result.into_iter().take(actual_result_len).collect();

    compact_result
}
