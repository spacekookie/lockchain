//! High level utility wrapper module around crypto calls
//!

use super::aes::AES;
use super::encoding;
use super::keys::Key;

use serde_json;
use serde::Serialize;
use serde::de::DeserializeOwned;


pub trait Encryptor<'a, T: Serialize + DeserializeOwned> {
    fn encrypt(&self, data: &T) -> String;
    fn decrypt(&self, data: String) -> T;
}


/// Wraps high-level utilities
pub struct CryptoCtx {
    core: AES,
}

impl<'a, T: Serialize + DeserializeOwned> Encryptor<'a, T> for CryptoCtx {

    /// Generic encryption function for any higher level type that can be Serialised
    /// 
    /// Returns a bse64 encoded string of ciphertext
    fn encrypt(&self, data: &T) -> String {
        let encoded = serde_json::to_string(&data).unwrap();
        let vec = str_to_vec(&encoded);

        /*  ✨ M A G I C ✨  */
        let encrypted = self.core.encrypt(&vec);
        let base64 = encoding::base64_encode(&encrypted);
        return base64.to_owned();
    }

    /// Generic decryption function for any higher level type that can be Deserialised
    /// 
    /// Takes a base64 encoded string as data
    fn decrypt(&self, data: String) -> T {
        let decoded: Vec<u8> = encoding::base64_decode(&data);
        let decrypted: String = self.core.decrypt(&decoded);

        let data: T = serde_json::from_str(&decrypted).unwrap();
        return data;
    }
}

impl CryptoCtx {

    /// Create a new key and crypto context from scratch
    pub fn new() -> CryptoCtx {
        let k = Key::new();
        return CryptoCtx { core: AES::new(&k) };
    }

    /// Create a new context with an existing key
    pub fn existing(key: &Key) -> CryptoCtx {
        return CryptoCtx { core: AES::new(key) };
    }

    /// Get the currently in-use key
    pub fn get_active_key(&self) -> &Key {
        return &self.core.key;
    }
}


/// Convert a utf-8 string to a vector of u8
fn str_to_vec(string: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for b in string.as_bytes() {
        vec.push(*b);
    }
    return vec;
}
