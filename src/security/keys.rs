//! A module that handles key generation and key loading
//!



use std::fs::File;
use std::io::prelude::*;

use super::random;
use super::hash;

pub const KEY_LENGTH: usize = 16;


/// A wrapper to represent a key for encryption
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Key {
    pub data: Vec<u8>,
}

impl Key {
    /// Create a new key from scratch
    pub fn new() -> Key {
        let data = random::bytes(KEY_LENGTH);
        return Key { data: data };
    }

    /// Use a password as a key
    pub fn from_password(password: &str, salt: &str) -> Key {
        let hashed = hash::blake2_16(password, salt);
        let vec: Vec<u8> = Vec::new();
        for b in &hashed {
            vec.push(b.clone());
        }
        return Key { data: vec };
    }

    /// Load an encrypted key from disk
    pub fn load(path: &String, password: &str) -> Key {
        let tmp_key = Key::from_password(password, "REPLACE WITH SALT");
        let ctx = CryptoCtx::existing(&tmp_key);

        /* Load encrypted from disk */
        let mut key = String::new();
        let mut key_file = File::open(path).unwrap();
        key_file.read_to_string(&mut key).expect(
            "Failed to load key file!",
        );

        let decrypted: Key = ctx.decrypt(key);
        return decrypted;
    }

    /// Save the current key, encrypted to disk
    pub fn save(&self, path: &String, password: &str) {
        let tmp_key = Key::from_password(password, "REPLACE WITH SALT");
        let ctx = CryptoCtx::existing(&tmp_key);

        let encrypted = ctx.encrypt(&self.clone());
        let key_file = File::create(path).unwrap();
        key_file.write_all(encrypted.as_bytes()).unwrap();
    }

    /// Used to get the raw data from this key, as a slice copy
    pub fn copy_slice(&self) -> [u8; KEY_LENGTH] {
        let mut slice: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
        slice.clone_from_slice(&self.data);
        return slice;
    }
}
