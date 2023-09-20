use anyhow::{anyhow, Result};
use fancy_regex::Regex;
use halo2_regex::vrm::{js_caller::format_regex_str, DecomposedRegexConfig};
use itertools::Itertools;
use neon::prelude::*;


pub fn extract_substr_idxes(
    input_str: &str,
    regex_config: &DecomposedRegexConfig,
) -> Result<Vec<usize>> {
    if input_str.len() > regex_config.max_byte_size {
        return Err(anyhow!(
            "The input string length is {}, but the max length is {}",
            input_str.len(),
            regex_config.max_byte_size
        ));
    }
    let mut entire_regex = String::new();
    for part in regex_config.parts.iter() {
        entire_regex += part.regex_def.as_str();
    }
    let entire_regex = Regex::new(&entire_regex)?;
    let mut start = entire_regex.find(input_str)?.ok_or_else(|| {
        anyhow!(
            "Substring of the entire regex is not found in {}",
            &input_str
        )
    })?.start();

    let mut public_idxes = vec![];
    // let mut last_regex_str = String::new();
    // let part_regex_defs = regex_config.parts.iter().map(|part| part.regex_def.as_str()).collect_vec();
    for part_idx in 0..regex_config.parts.len() {
        // last_regex_str = last_regex_str + regex_config.parts[part_idx].regex_def.as_str();
        let regex = Regex::new(&format_regex_str(&regex_config.parts[part_idx].regex_def.as_str())?)?;
        let found = regex.find_from_pos(&input_str,start)?.ok_or_else(|| {
            anyhow!(
                "Substring of {} is not found in {}",
                regex,
                &input_str[start..]
            )
        })?;
        if found.start() >= found.end() {
            return Err(anyhow!(
                "Substring of {} in {} must not be empty",
                regex,
                &input_str[start..]
            ));
        }
        if regex_config.parts[part_idx].is_public {
            public_idxes.push(start);
        }
        start = found.end();
    }

    // for (idx, part) in regex_config.parts.iter().enumerate() {
    //     let regex = Regex::new(&format_regex_str(&part.regex_def)?)?;
    //     let found = regex.find_from_pos(&input_str,start)?.ok_or_else(|| {
    //         anyhow!(
    //             "Substring of {} is not found in {}",
    //             regex,
    //             &input_str[start..]
    //         )
    //     })?;
    //     if found.start() >= found.end() {
    //         return Err(anyhow!(
    //             "Substring of {} in {} must not be empty",
    //             regex,
    //             &input_str[start..]
    //         ));
    //     }
    //     if (part.is_public) {
    //         public_idxes.push(start);
    //     }
    //     start = found.end();
    // }

    Ok(public_idxes)
}

// pub fn extract_substr_idxes(
//     input_str: &str,
//     regex_config: &DecomposedRegexConfig,
// ) -> Result<Vec<usize>> {
//     if input_str.len() > regex_config.max_byte_size {
//         return Err(anyhow!(
//             "The input string length is {}, but the max length is {}",
//             input_str.len(),
//             regex_config.max_byte_size
//         ));
//     }
//     let mut public_idxes = vec![];
//     let mut start = 0;
//     for (idx, part) in regex_config.parts.iter().enumerate() {
//         let regex = Regex::new(&format_regex_str(&part.regex_def)?)?;
//         let found = regex.find_from_pos(&input_str,start)?.ok_or_else(|| {
//             anyhow!(
//                 "Substring of {} is not found in {}",
//                 regex,
//                 &input_str[start..]
//             )
//         })?;
//         if found.start() >= found.end() {
//             return Err(anyhow!(
//                 "Substring of {} in {} must not be empty",
//                 regex,
//                 &input_str[start..]
//             ));
//         }
//         if (part.is_public) {
//             public_idxes.push(start);
//         }
//         start = found.end();
//     }

//     Ok(public_idxes)
// }

pub fn extract_substr_idxes_node(
    mut cx: FunctionContext
) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let regex_config_str = cx.argument::<JsString>(1)?.value(&mut cx);
    let regex_config = match  serde_json::from_str::<DecomposedRegexConfig>(&regex_config_str){
        Ok(regex_config) => regex_config,
        Err(e) => return cx.throw_error(e.to_string())
    };
    let substr_idxes = match extract_substr_idxes(&input_str, &regex_config){
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string())
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, substr_idx) in substr_idxes.iter().enumerate() {
        let js_substr_idx = cx.number(*substr_idx as f64);
        js_array.set(&mut cx, i as u32, js_substr_idx)?;
    }
    Ok(js_array)
}

#[cfg(test)]
mod test {
    use halo2_regex::vrm::RegexPartConfig;

    use super::*;

    #[test]
    fn test_email_domain_valid() {
        let email_addr_regex = DecomposedRegexConfig {
            max_byte_size: 256,
            parts: vec![
                RegexPartConfig {
                    is_public: false,
                    regex_def: "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|0|1|2|3|4|5|6|7|8|9|.|_|%|\\+|-|=)+".to_string(),
                    max_size: 64,
                    solidity: None
                },
                RegexPartConfig {
                    is_public: false,
                    regex_def: "@".to_string(),
                    max_size: 1,
                    solidity: None
                },
                RegexPartConfig {
                    is_public: true, 
                    regex_def: "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|0|1|2|3|4|5|6|7|8|9|\\.|-)+".to_string(),
                    max_size: 255,
                    solidity: None
                }
            ]
        };
        let input_str = "suegamisora@gmail.com";
        let idxes = extract_substr_idxes(input_str, &email_addr_regex).unwrap();
        assert_eq!(idxes, vec![12]);
    }


    #[test]
    fn test_email_addr_in_subject_valid() {
        let email_addr_regex = DecomposedRegexConfig {
            max_byte_size: 256,
            parts: vec![
                RegexPartConfig {
                    is_public: true,
                    regex_def: "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|0|1|2|3|4|5|6|7|8|9|\\.|_|%|\\+|-|=)+".to_string(),
                    max_size: 64,
                    solidity: None
                },
                RegexPartConfig {
                    is_public: true,
                    regex_def: "@".to_string(),
                    max_size: 1,
                    solidity: None
                },
                RegexPartConfig {
                    is_public: true, 
                    regex_def: "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|0|1|2|3|4|5|6|7|8|9|\\.|-)+".to_string(),
                    max_size: 255,
                    solidity: None
                }
            ]
        };
        let input_str = "This is sent for suegamisora@gmail.com";
        let idxes = extract_substr_idxes(input_str, &email_addr_regex).unwrap();
        assert_eq!(idxes, vec![17,28,29]);
    }

    #[test]
    fn test_code_in_subject_valid() {
        let code_regex = DecomposedRegexConfig {
            max_byte_size: 1024,
            parts: vec![
                RegexPartConfig {
                    is_public: false,
                    regex_def: r"CODE:0x".to_string(),
                    max_size: 7,
                    solidity: None
                },
                RegexPartConfig {
                    is_public: true,
                    regex_def: "(0|1|2|3|4|5|6|7|8|9|a|b|c|d|e|f)+".to_string(),
                    max_size: 6,
                    solidity: None
                }
            ]
        };
        let input_str = "subject: Email Wallet CODE:0x123abc";
        let idxes = extract_substr_idxes(input_str, &code_regex).unwrap();
        assert_eq!(idxes, vec![29]);
    }
}
