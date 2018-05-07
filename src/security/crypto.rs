//! Comprehensive encryption submodule which handles serialising and de-serialising

use miscreant::aead::{Aes256Siv, Algorithm};
use security::{keys::{Key, KEY_LENGTH},
               utils::{encoding, random}};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use std::io::prelude::*;
use std::{error::Error, fs::File};

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
    iv: Vec<u8>,
    data: Vec<u8>,
}

impl CryptoEngine {
    /// Create a new CryptoEngine from a key
    ///
    /// This generates a new IV which is then used for all
    /// cryptographic transactions in a vault context
    pub fn generate(key: Key) -> CryptoEngine {
        return CryptoEngine {
            ctx: Aes256Siv::new(&key.to_slice()),
            key: key,
            iv: random::bytes(KEY_LENGTH),
        };
    }

    /// Load an existing encryption context into scope
    ///
    /// Takes an encrypted stream of a key file that wraps a key object
    /// in a packed encryption object.
    pub fn load(path: &str, pw: &str, salt: &str) -> Result<CryptoEngine, Box<Error>> {
        let mut file = File::open(path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let decoded = String::from_utf8(encoding::base64_decode(&file_content))?;
        let packed: PackedData = serde_json::from_str(&decoded)?;

        /* Decrypt key */
        let decrypted_key: Key = CryptoEngine {
            ctx: Aes256Siv::new(&packed.data.as_slice()),
            key: Key::from_password(pw, salt),
            iv: packed.iv.clone(),
        }.decrypt(String::from_utf8(packed.data)?)?;

        return Ok(CryptoEngine {
            ctx: Aes256Siv::new(&decrypted_key.to_slice()),
            key: decrypted_key,
            iv: packed.iv,
        });
    }

    /// Save the current key, encrypted to disk
    pub fn save(&mut self, path: &str, pw: &str, salt: &str) -> Result<(), Box<Error>> {
        /* Encrypt key */
        let mut tmp = CryptoEngine {
            ctx: Aes256Siv::new(&self.key.data.as_slice()),
            key: Key::from_password(pw, salt),
            iv: self.iv.clone(),
        };

        let encrypted_key = tmp.encrypt(&self.key)?;

        let mut file = File::create(path)?;
        file.write_all(encrypted_key.as_bytes())?;
        return Ok(());
    }

    /// Encrypt a piece of data, returns a packed and encoded string
    pub fn encrypt<T: Serialize>(&mut self, data: &T) -> Result<String, Box<Error>> {
        let serial = serde_json::to_string(&data)?;
        let nonce = random::bytes(64);
        let iv = &self.iv.as_slice();
        let data = &serial.as_bytes();

        let encrypted = self.ctx.seal(nonce.as_slice(), iv, data);
        let packed = PackedData {
            iv: self.iv.clone(),
            data: encrypted,
            nonce: nonce,
        };

        let enc_packed = serde_json::to_string(&packed)?;
        return Ok(encoding::base64_encode(&enc_packed.into_bytes()));
    }

    /// Decrypt a ciphertext string into a type object
    pub fn decrypt<T: DeserializeOwned>(&mut self, cipher: String) -> Result<T, Box<Error>> {
        let dec_packed = String::from_utf8(encoding::base64_decode(&cipher))?;
        let p: PackedData = serde_json::from_str(&dec_packed)?;

        let iv = &self.iv.as_slice();
        let decrypted = self.ctx.open(p.nonce.as_slice(), iv, p.data.as_slice())?;
        let decr_str = String::from_utf8(decrypted)?;

        let t: T = serde_json::from_str(&decr_str)?;
        return Ok(t);
    }
}
