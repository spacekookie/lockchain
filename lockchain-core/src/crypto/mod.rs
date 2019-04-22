//! Shared cryptographic primitives and utilities
//!
//!

/// We re-export keybob's API here
mod keys {
    use crate::traits::{AutoEncoder, Encryptable};
    pub use keybob::{Key, KeyType};

    impl AutoEncoder for Key {}
    impl AutoEncoder for KeyType {}

    impl Encryptable for Key {}
    impl Encryptable for KeyType {}
}

mod data;
pub mod encoding;
pub mod hashing;
pub mod random;

pub use self::data::PackedData;
pub use self::keys::{Key, KeyType};
