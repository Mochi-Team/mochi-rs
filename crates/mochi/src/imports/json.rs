use super::core::PtrRef;
use super::error::{Result, MochiError};

#[link(wasm_import_module = "json")]
// #[link(name = "swift-bindings", kind = "static")]
extern "C" {
    fn json_parse(bytes: i32, size: i32) -> i32;
}

pub type JsonValue = PtrRef;

pub fn parse<T: AsRef<[u8]>>(buf: T) -> Result<JsonValue> {
    let buf = buf.as_ref();
    let ptr: i32 = unsafe { json_parse(buf.as_ptr() as i32, buf.len() as i32) };
    match ptr {
        -1 => Err(MochiError::JsonParseError),
        _ => Ok(PtrRef::new(ptr))
    }
}