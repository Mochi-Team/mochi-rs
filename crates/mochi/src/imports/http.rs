extern crate alloc;

use alloc::{vec::Vec, string::String};

use super::error::{Result, MochiError, PtrCastError};
use super::core::PtrRef;
use super::html::Node;
use super::json::{parse, JsonValue};

type ReqRef = i32;

#[link(wasm_import_module = "http")]
// #[link(name = "swift-bindings", kind = "static")]
extern "C" {
    #[link_name = "create"]
    fn request_create(method: RequestMethod) -> ReqRef;
    #[link_name = "send"]
    fn request_send(ptr: ReqRef);
    #[link_name = "close"]
    fn request_close(ptr: ReqRef);

    #[link_name = "set_url"]
    fn request_set_url(ptr: ReqRef, url_ptr: i32, url_len: i32);
    #[link_name = "set_header"]
    fn request_set_header(ptr: ReqRef, key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32);
    #[link_name = "set_body"]
    fn request_set_body(ptr: ReqRef, data_ptr: i32, data_len: i32);
    #[link_name = "set_method"]
    fn request_set_method(ptr: ReqRef, method: RequestMethod);

    #[link_name = "get_method"]
    fn request_get_method(ptr: ReqRef) -> RequestMethod;
    #[link_name = "get_url"]
    fn request_get_url(ptr: ReqRef) -> i32;
    #[link_name = "get_header"]
    fn request_get_header(ptr: ReqRef, key_ptr: i32, key_len: i32) -> i32;
    #[link_name = "get_status_code"]
    fn request_get_status_code(ptr: ReqRef) -> i32;
    #[link_name = "get_data_len"]
    fn request_get_data_len(ptr: ReqRef) -> i32;
    #[link_name = "get_data"]
    fn request_get_data(ptr: ReqRef, arr_ptr: i32, len: i32);
}

#[repr(C)]
#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

#[derive(Debug)]
pub struct Request {
    ptr: i32,
}

// By default, the method it uses is `GET`
impl Request {
    pub fn new(url: &str, method: RequestMethod) -> Self {
        unsafe {
            let ptr: i32 = request_create(method);
            request_set_url(
                ptr,
                url.as_ptr() as i32, 
                url.len() as i32
            );
            Self { ptr }
        }
    }

    #[inline]
    pub fn send(&self) {
        unsafe { request_send(self.ptr); }
    }

    #[inline]
    fn close(&self) {
        unsafe { request_close(self.ptr); }
    }

    pub fn set_url<T: AsRef<str>>(self, url: T) -> Self {
        let url = url.as_ref();
        unsafe {
            request_set_url(
                self.ptr, 
                url.as_ptr() as i32,
                url.len() as i32
            );
        };
        self
    }

    pub fn header<T: AsRef<str>>(self, key: T, value: T) -> Self {
        let key = key.as_ref();
        let value = value.as_ref();
        unsafe {
            request_set_header(
                self.ptr as i32, 
                key.as_ptr() as i32, 
                key.len() as i32, 
                value.as_ptr() as i32, 
                value.len() as i32
            )
        };
        self
    }

    pub fn body<T: AsRef<[u8]>>(self, data: T) -> Self {
        let data = data.as_ref();
        unsafe { 
            request_set_body(
                self.ptr, 
                data.as_ptr() as i32, 
                data.len() as i32
            ) 
        };
        self
    }

    pub fn set_method(self, method: RequestMethod) -> Self {
        unsafe {
            request_set_method(self.ptr, method)
        }
        self
    }

    #[inline]
    pub fn status_code(&self) -> i32 {
        unsafe {
            request_get_status_code(self.ptr)
        }        
    }

    pub fn get_method(&self) -> RequestMethod {
        unsafe {
            request_get_method(self.ptr)
        }
    }

    pub fn get_header<T: AsRef<str>>(&self, key: T) -> Result<String> {
        let key = key.as_ref();
        let value_ptr = unsafe {
            request_get_header(
                self.ptr, 
                key.as_ptr() as i32, 
                key.len() as i32
            )
        };
        PtrRef::new(value_ptr).as_string()
    }

    pub fn url(&self) -> String {
        let str_ptr = unsafe {
            request_get_url(self.ptr)
        };
        PtrRef::new(str_ptr).as_string().unwrap_or_default()
    }

    pub fn data(self) -> Vec<u8> {
        self.send();
        let size = unsafe { request_get_data_len(self.ptr) };
        let mut buf = Vec::with_capacity(size as usize);
        unsafe {
            request_get_data(self.ptr, buf.as_mut_ptr() as i32, size);
            buf.set_len(size as usize);
        }
        self.close();
        buf
    }

    pub fn string(self) -> Result<String> {
        match String::from_utf8(self.data()) {
            Ok(v) => Ok(v),
            Err(_) => Err(MochiError::from(PtrCastError::Utf8NotValid)),
        }
    }

    pub fn json(self) -> Result<JsonValue> {
        parse(self.data())
    }

    pub fn html(self) -> Result<Node> {
        let url = self.url();
        let data = self.data();

        if url.is_empty() {
            Node::new(data)
        } else {
            Node::new_with_uri(data, url)
        }
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        self.close();
    }
}