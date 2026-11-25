use serde_json::{json, Value};
use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn omml_abi_version() -> u32 {
    1
}

/// Input buffers are borrowed until return. Each non-null response is owned
/// by the caller and must be passed to omml_free_json_v1 exactly once.
#[no_mangle]
pub unsafe extern "C" fn omml_roundtrip_json_v1(
    root: *const u8,
    root_len: usize,
    xml: *const u8,
    xml_len: usize,
) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        if root.is_null() || xml.is_null() {
            return failure(json!({"category":"invalid_argument","message":"null input buffer"}));
        }
        let root = std::slice::from_raw_parts(root, root_len);
        let xml = std::slice::from_raw_parts(xml, xml_len);
        let root = match std::str::from_utf8(root) {
            Ok(value) => value,
            Err(_) => {
                return failure(
                    json!({"category":"invalid_argument","message":"model type must be UTF-8"}),
                )
            }
        };
        match roundtrip(root, xml) {
            Ok(value) => json!({"abi_version":1,"ok":true,"result":value}),
            Err(error) => failure(error),
        }
    })
    .unwrap_or_else(|_| {
        failure(json!({"category":"runtime","message":"native operation panicked"}))
    });
    CString::new(result.to_string()).map_or(std::ptr::null_mut(), CString::into_raw)
}

/// The document roots go through `document`, which writes the bytes the Ruby
/// gem writes; every other root uses the generated codecs.
fn roundtrip(root: &str, xml: &[u8]) -> Result<Value, Value> {
    use crate::document::OmmlRoot;
    if root != "OMath" && root != "OMathPara" {
        return crate::dispatch::roundtrip(root, xml);
    }
    let contract = |error: Box<lutaml_rs::contracts::ContractError>| json!({"category":"contract","contract":error});
    let document = crate::document::parse(xml).map_err(contract)?;
    let model = match (&document.root, root) {
        (OmmlRoot::OMath(model), "OMath") => serde_json::to_value(model),
        (OmmlRoot::OMathPara(model), "OMathPara") => serde_json::to_value(model),
        _ => {
            return Err(json!({"category":"invalid_argument","message":"input root differs from the requested model type"}))
        }
    }
    .map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
    let xml = document.to_xml().map_err(contract)?;
    Ok(json!({"model":model,"xml":xml}))
}

fn failure(error: Value) -> Value {
    json!({"abi_version":1,"ok":false,"error":error})
}

#[no_mangle]
pub unsafe extern "C" fn omml_free_json_v1(reply: *mut c_char) {
    if !reply.is_null() {
        drop(CString::from_raw(reply));
    }
}
