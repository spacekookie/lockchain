//! Cryptography module for lockchain
//!
//! A crypto engine is attached to a vault and provides easy to use
//! and high-level encryption and decryption functions.

pub mod hash;
pub mod random;
pub mod engine;
pub mod encoding;

const DEFAULT_KEYLENGTH: usize = 16;
