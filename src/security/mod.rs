//! Cryptography module for lockchain
//!
//! A crypto engine is attached to a vault and provides easy to use
//! and high-level encryption and decryption functions.

// Utility modules
pub mod encoding;
pub mod random;
pub mod hash;
pub mod keys;

// Core cryptography
pub mod aes;
pub mod encryption;
pub use self::encryption::*;
