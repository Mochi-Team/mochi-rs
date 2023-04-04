extern crate alloc;

use alloc::{string::String, vec::Vec};
use crate::convert::ToWasm;

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum Result<T> {
    Ok(T),
    Err(MochiError)
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MochiError {
    Network
}

#[link(wasm_import_module = "mochi_imports")]
extern "C" {
    // Media
    fn create_media(
        id: i32,
        id_len: i32,
        title: i32,
        title_len: i32
    ) -> i32;
}

#[derive(Debug, Clone)]
pub struct Media {
    pub id: String,
    pub title: String
}

impl ToWasm for Media {
    type Value = i32;

    fn to_wasm(self) -> Self::Value {
        unsafe { 
            create_media(
                self.id.as_ptr() as i32, 
                self.id.len() as i32, 
                self.title.as_ptr() as i32, 
                self.title.len() as i32
            )
        }
    }
}

impl<T> ToWasm for Result<T> where T: ToWasm<Value = i32> {
    type Value = i32;

    fn to_wasm(self) -> Self::Value {
        match self {
            Result::Ok(val) => T::to_wasm(val),
            Result::Err(err) => err as i32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paging<T> {
    pub items: Vec<T>,
    pub current_page: String,
    pub next_page: Option<String>,
}