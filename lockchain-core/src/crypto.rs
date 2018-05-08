//! Shared cryptographic primitives for the lockchain ecosystem
//! 
//! This is a secure storage vault after all, we need some
//! shared crypto helpers for all the other crates :)

mod keys;
mod utils;

pub use self::keys::{Key, KEY_LENGTH};
pub use self::utils::*;