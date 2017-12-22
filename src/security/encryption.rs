//! High level utility wrapper module around crypto calls
//!

use super::aes::AES;
use super::encoding;
use super::keys;

use vault::ErrorType;

use serde_json;
use serde::{Serialize, Deserialize};

use generic_array::GenericArray;
use std::str::from_utf8_unchecked;


pub trait Blaaaaaa<T: Serialize + Deserialize<'static>> {
    fn encrypt(&self, data: &T) -> String;
    fn decrypt(&self, data: &String) -> T;
}


/// Wraps high-level utilities
pub struct CryptoHandler {
    core: AES,
}

impl<T: Serialize + Deserialize<'static>> Blaaaaaa<T> for CryptoHandler {
    fn encrypt(&self, data: &T) -> String {
        let encoded = serde_json::to_string(&data).unwrap();
        let vec = str_to_vec(&encoded);

        /*  ✨ M A G I C ✨  */
        let encrypted = self.core.encrypt(&vec);
        let base64 = encoding::base64_encode(&encrypted);

        return base64.to_owned();
    }

    fn decrypt(&self, data: &String) -> T {
        let decoded = encoding::base64_decode(data);
        let decrypted = self.core.decrypt(&decoded);

        let data: T = serde_json::from_str(&decrypted).unwrap();
        return data;
    }
}

impl CryptoHandler {
    pub fn new() -> CryptoHandler {
        let k = keys::generate_key();

        return CryptoHandler { core: AES::new(&k) };
    }
}


/// Convert a vector of u8 into a utf-8 string
fn vec_to_str(vec: &[u8]) -> String {
    return unsafe { String::from(from_utf8_unchecked(vec)) };
}

/// Convert a utf-8 string to a vector of u8
fn str_to_vec(string: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for b in string.as_bytes() {
        vec.push(*b);
    }
    return vec;
}


/// Generic encryption utility which takes any serialisable data
/// and returns a base64 encoded ciphertext
///
pub fn encrypt<T: Serialize>(handle: &CryptoHandler, data: T) -> String {
    let encoded = serde_json::to_string(&data).unwrap();
    let vec = str_to_vec(&encoded);

    /*  ✨ M A G I C ✨  */
    let encrypted = handle.core.encrypt(&vec);
    let base64 = encoding::base64_encode(&encrypted);

    return base64.to_owned();
}


/// Generic decryption utility which takes a base64 encoded ciphertext and
/// returns any Deserializable Rust struct
///
pub fn decrypt<T: Deserialize<'static>>(
    handle: &CryptoHandler,
    encrypted: &String,
) -> Result<T, ErrorType> {
    let decoded = encoding::base64_decode(encrypted);
    let decrypted = handle.core.decrypt(&decoded);

    let data: T = serde_json::from_str(&decrypted).unwrap();
    return Ok(data);
}