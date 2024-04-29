use neon::prelude::*;
pub use zk_regex_apis::extract_substrs::*;
pub use zk_regex_apis::padding::*;

pub fn pad_string_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let string = cx.argument::<JsString>(0)?.value(&mut cx);
    let padded_bytes_size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let padded_bytes = pad_string(&string, padded_bytes_size);
    let padded_array = JsArray::new(&mut cx, padded_bytes_size as u32);
    for (idx, byte) in padded_bytes.into_iter().enumerate() {
        let js_byte = cx.number(byte);
        padded_array.set(&mut cx, idx as u32, js_byte)?;
    }
    Ok(padded_array)
}

pub fn extract_substr_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let regex_config_str = cx.argument::<JsString>(1)?.value(&mut cx);
    let regex_config = match serde_json::from_str::<DecomposedRegexConfig>(&regex_config_str) {
        Ok(regex_config) => regex_config,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let substr_idxes = match extract_substr_idxes(&input_str, &regex_config) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_email_addr_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_email_addr_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_email_domain_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_email_domain_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_email_addr_with_name_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_email_addr_with_name_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_from_all_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_from_all_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_from_addr_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_from_addr_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_to_all_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);

    let substr_idxes = match extract_to_all_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };

    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);

    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);

        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;

        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;

        js_array.set(&mut cx, i as u32, start_end_array)?;
    }

    Ok(js_array)
}

pub fn extract_to_addr_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_to_addr_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_subject_all_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_subject_all_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_body_hash_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_body_hash_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_timestamp_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_timestamp_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}

pub fn extract_message_id_idxes_node(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let substr_idxes = match extract_message_id_idxes(&input_str) {
        Ok(substr_idxes) => substr_idxes,
        Err(e) => return cx.throw_error(e.to_string()),
    };
    let js_array = JsArray::new(&mut cx, substr_idxes.len() as u32);
    for (i, (start_idx, end_idx)) in substr_idxes.iter().enumerate() {
        let start_end_array = JsArray::new(&mut cx, 2u32);
        let start_idx = cx.number(*start_idx as f64);
        start_end_array.set(&mut cx, 0, start_idx)?;
        let end_idx = cx.number(*end_idx as f64);
        start_end_array.set(&mut cx, 1, end_idx)?;
        js_array.set(&mut cx, i as u32, start_end_array)?;
    }
    Ok(js_array)
}
