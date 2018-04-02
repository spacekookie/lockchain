//!

use miscreant::aead::{Aes256Siv, Algorithm};
use security::{keys::{Key, KEY_LENGTH}, utils::{Encoding, Hashing, Random}};
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use std::error::Error;

/// The main encryption context
pub struct CryptoEngine {
    ctx: Aes256Siv,
    key: Key,
    iv: Vec<u8>,
}

/// Represents some packed data – includes nonce and blob
#[derive(Serialize, Deserialize)]
struct PackedData {
    nonce: Vec<u8>,
    data: Vec<u8>,
}

impl CryptoEngine {
    /// Create a new encryption context with a key
    pub fn new(key: Key) -> CryptoEngine {
        return CryptoEngine {
            ctx: Aes256Siv::new(&key.to_slice()),
            key: key,
            iv: Random::bytes(KEY_LENGTH),
        };
    }

    /// Load an existing encryption context into scope
    pub fn load(key: Key, iv: Vec<u8>) -> CryptoEngine {
        return CryptoEngine {
            ctx: Aes256Siv::new(&key.to_slice()),
            key: key,
            iv: iv,
        };
    }

    /// Encrypt a piece of data, returns a packed and encoded string
    pub fn encrypt<T: Serialize>(&mut self, data: &T) -> Result<String, Box<Error>> {
        let serial = serde_json::to_string(&data)?;
        let nonce = Random::bytes(64);
        let iv = &self.iv.as_slice();
        let data = &serial.as_bytes();

        let encrypted = self.ctx.seal(nonce.as_slice(), iv, data);
        let packed = PackedData {
            nonce: nonce,
            data: encrypted,
        };

        let enc_packed = serde_json::to_string(&packed)?;
        return Ok(Encoding::base64_encode(&enc_packed.into_bytes()));
    }

    /// Decrypt a ciphertext string into a type object
    pub fn decrypt<T: DeserializeOwned>(&mut self, cipher: String) -> Result<T, Box<Error>> {
        let dec_packed = String::from_utf8(Encoding::base64_decode(&cipher))?;
        let p: PackedData = serde_json::from_str(&dec_packed)?;

        let iv = &self.iv.as_slice();
        let decrypted = self.ctx.open(p.nonce.as_slice(), iv, p.data.as_slice())?;
        let decr_str = String::from_utf8(decrypted)?;

        let t: T = serde_json::from_str(&decr_str)?;
        return Ok(t);
    }
}
