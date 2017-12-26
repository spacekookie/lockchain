//! A module that handles key generation and key loading
//!



use std::fs::File;
use std::ffi::OsStr;
use std::io::prelude::*;

use super::random;
use super::encoding;
use super::hash;

pub const KEY_LENGTH: usize = 16;


/// A wrapper to represent a key for encryption
#[derive(Clone, Serialize, Deserialize)]
pub struct Key {
    pub data: [u8; KEY_LENGTH],
}


/// A helper function to easily load a key into memory
pub fn load_key(path: &OsStr) -> Key {

    let mut key = String::new();
    let mut key_file = File::open(path).unwrap();
    key_file.read_to_string(&mut key).expect(
        "Failed to load primary key file!",
    );

    let vec = encoding::base64_decode(&key);
    let mut k: [u8; 16] = [0; 16];
    k.clone_from_slice(&vec);

    return Key { data: k };
}


pub fn password_to_key(password: &str) -> Key {
    let hashed = hash::blake2_16(password, "");
    return Key { data: hashed };
}

pub fn generate_key() -> Key {
    let key = random::bytes(KEY_LENGTH);

    let mut k: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
    k.clone_from_slice(&key);

    return Key { data: k };
}
