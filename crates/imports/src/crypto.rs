use alloc::{string::String, vec::Vec};

use crate::core::PtrRef;

#[link(wasm_import_module = "crypto")]
extern "C" {
    fn crypto_get_data_len(host_ptr: i32) -> i32;
    fn crypto_get_data(host_ptr: i32, buf_ptr: i32, buf_len: i32);

    fn crypto_base64_parse(
        value_ptr: i32,
        value_len: i32
    ) -> i32;

    fn crypto_base64_string(
        bytes_ptr: i32,
        bytes_len: i32
    ) -> i32;

    fn crypto_utf8_parse(
        value_ptr: i32, 
        value_len: i32
    ) -> i32;

    fn crypto_pbkdf2(
        hash_algorithm: i32,
        password_ptr: i32,
        password_len: i32,
        salt_ptr: i32,
        salt_len: i32,
        rounds: i32,
        key_len: i32
    ) -> i32;

    fn crypto_generate_random_bytes(count: i32) -> i32;

    fn crypto_aes_encrypt(
        msg_ptr: i32,
        msg_len: i32,
        key_ptr: i32,
        key_len: i32,
        iv_ptr: i32,
        iv_len: i32
    ) -> i32;

    fn crypto_aes_decrypt(
        encrypted_msg_ptr: i32,
        encrypted_msg_len: i32,
        key_ptr: i32,
        key_len: i32,
        iv_ptr: i32,
        iv_len: i32
    ) -> i32;

    fn crypto_md5_hash(
        input_ptr: i32,
        input_len: i32
    ) -> i32;
}


#[repr(C)]
pub enum CryptoPBKDFAlgorithm {
    HmacAlgSHA1 = 1,
    HmacAlgSHA224 = 2,
    HmacAlgSHA256 = 3,
    HmacAlgSHA384 = 4,
    HmacAlgSHA512 = 5
}

// This represents CryptoJS but uses Apple's CryptoKit and CommonCrypto to
// compute cryptographies
//
// This uses the CBC to follow CryptoJS's default decryption.
pub struct Crypto {}

// Parsers
impl Crypto {
    pub fn utf8_parse(value: &str) -> String {
        let host_data_ptr = unsafe {
            crypto_utf8_parse(
                value.as_ptr() as i32, 
                value.len() as i32
            )
        };

        String::from_utf8(data_to_vec(host_data_ptr)).unwrap_or_default()
    }

    pub fn base64_parse(value: &str) -> Vec<u8> {
        let host_data_ptr = unsafe {
            crypto_base64_parse(
                value.as_ptr() as i32, 
                value.len() as i32
            )
        };
        data_to_vec(host_data_ptr)
    }

    pub fn base64_string(bytes: &[u8]) -> String {
        let host_string_ptr = unsafe {
            crypto_base64_string(
                bytes.as_ptr() as i32, 
                bytes.len() as i32
            )
        };
        let ptr = PtrRef::new(host_string_ptr);
        ptr.as_string().unwrap_or_default()
    }
}

// AES
impl Crypto {
    pub fn aes_encrypt(
        msg: &[u8], 
        key: &[u8], 
        iv: &[u8]
    ) -> Vec<u8> {
        let data_ptr = unsafe {
            crypto_aes_encrypt(
                msg.as_ptr() as i32, 
                msg.len() as i32, 
                key.as_ptr() as i32, 
                key.len() as i32, 
                iv.as_ptr() as i32, 
                iv.len() as i32
            )
        };
        data_to_vec(data_ptr)
    }

    pub fn aes_decrypt(
        encrypted_msg: &[u8],
        key: &[u8],
        iv: &[u8]
    ) -> Vec<u8> {
        let data_ptr = unsafe {
            crypto_aes_decrypt(
                encrypted_msg.as_ptr() as i32, 
                encrypted_msg.len() as i32, 
                key.as_ptr() as i32, 
                key.len() as i32, 
                iv.as_ptr() as i32, 
                iv.len() as i32
            )
        };
        data_to_vec(data_ptr)
    }
}

// MD5
impl Crypto {
    pub fn md5_hash(
        input: &[u8]
    ) -> Vec<u8> {
        let host_ptr = unsafe {
            crypto_md5_hash(
                input.as_ptr() as i32, 
                input.len() as i32
            )
        };
        data_to_vec(host_ptr)
    }
}

impl Crypto {
    pub fn generate_pbkdf2(
        hash_algorithm: CryptoPBKDFAlgorithm,
        password: &str,
        salt: &[u8],
        rounds: i32,
        key_len: i32
    ) -> Vec<u8> {
        let host_ptr = unsafe {
            crypto_pbkdf2(
                hash_algorithm as i32, 
                password.as_ptr() as i32, 
                password.len() as i32, 
                salt.as_ptr() as i32, 
                salt.len() as i32, 
                rounds, 
                key_len
            )
        };
        data_to_vec(host_ptr)
    }

    pub fn generate_random_bytes(count: i32) -> Vec<u8> {
        let host_ptr = unsafe {
            crypto_generate_random_bytes(count)
        };
        data_to_vec(host_ptr)
    }
}

fn data_to_vec(host_data_ptr: i32) -> Vec<u8> {
    if host_data_ptr >= 0 {
        let data_size = unsafe { crypto_get_data_len(host_data_ptr) };
        let mut buf = Vec::with_capacity(data_size as usize);
        unsafe {
            crypto_get_data(
                host_data_ptr, 
                buf.as_mut_ptr() as i32, 
                data_size
            );
            buf.set_len(data_size as usize);
        }
        buf
    } else {
        Vec::new()
    }
}