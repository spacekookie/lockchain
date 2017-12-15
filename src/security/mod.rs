//! Cryptography module for lockchain
//!
//! A crypto engine is attached to a vault and provides easy to use
//! and high-level encryption and decryption functions.

// mod aes;

pub mod hash;
pub mod random;
pub mod engine;
pub mod encoding;


const DEFAULT_KEYLENGTH: usize = 32;
