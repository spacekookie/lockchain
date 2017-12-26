//! High level utility wrapper module around crypto calls
//!

use super::aes::AES;
use super::encoding;
use super::keys;

use record::Record;

use serde_json;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::str::from_utf8_unchecked;


pub trait Encryptor<'a, T: Serialize + DeserializeOwned> {
    fn encrypt(&self, data: &T) -> String;
    fn decrypt(&self, data: String) -> T;
}


/// Wraps high-level utilities
pub struct CryptoHandler {
    core: AES,
}

impl<'a, T: Serialize + DeserializeOwned> Encryptor<'a, T> for CryptoHandler {
    fn encrypt(&self, data: &T) -> String {
        let encoded = serde_json::to_string(&data).unwrap();
        let vec = str_to_vec(&encoded);

        /*  ✨ M A G I C ✨  */
        let encrypted = self.core.encrypt(&vec);
        let base64 = encoding::base64_encode(&encrypted);

        return base64.to_owned();
    }

    fn decrypt(&self, data: String) -> T {
        let decoded: Vec<u8> = encoding::base64_decode(&data);
        let decrypted: String = self.core.decrypt(&decoded);

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
