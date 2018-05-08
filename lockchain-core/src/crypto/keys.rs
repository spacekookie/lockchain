//! A module that handles key generation and key loading

use super::utils::{hashing, random};

/// A shared key length parameter for all cryptographic operations
/// 
/// This is *not* ideal and should be replaced with something better
/// at some point in the future
pub const KEY_LENGTH: usize = 64;

/// A wrapper to represent a key for encryption
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Key {
    pub data: Vec<u8>,
}

impl Key {

    /// Create a new key from scratch
    pub fn generate() -> Key {
        let data = random::bytes(KEY_LENGTH);
        return Key { data: data };
    }

    /// Use a password as a key
    pub fn from_password(password: &str, salt: &str) -> Key {
        let hashed = hashing::blake2(password, salt);
        let mut vec: Vec<u8> = Vec::new();
        for b in &hashed {
            vec.push(b.clone());
        }
        return Key { data: vec };
    }

    /// Used to get the raw data from this key, as a slice copy
    pub fn to_slice(&self) -> [u8; KEY_LENGTH] {
        let mut slice: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
        slice.clone_from_slice(&self.data);
        return slice;
    }
}
