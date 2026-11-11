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
            Err(_) => return failure(json!({"category":"invalid_argument","message":"model type must be UTF-8"})),
        };
        match crate::dispatch::roundtrip(root, xml) {
            Ok(value) => json!({"abi_version":1,"ok":true,"result":value}),
            Err(error) => failure(error),
        }
    }).unwrap_or_else(|_| failure(json!({"category":"runtime","message":"native operation panicked"})));
    CString::new(result.to_string()).map_or(std::ptr::null_mut(), CString::into_raw)
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
