//! A module that handles key generation and key loading
//!



use std::fs::File;
use std::ffi::OsStr;
use std::io::prelude::*;

pub const KEY_LENGTH: usize = 16;

/// A helper function to easily load a key into memory
pub fn load_key(path: &OsStr) -> String {

    let mut key = String::new();
    let mut key_file = File::open(path).unwrap();
    key_file.read_to_string(&mut key).expect(
        "Failed to load primary key file!",
    );

    // assert!(key.len() == KEY_LENGTH);
    return key;
}