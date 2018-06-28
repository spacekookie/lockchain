//! Shared cryptographic primitives for the lockchain ecosystem
//!
//! This is a secure storage vault after all, we need some
//! shared crypto helpers for all the other crates :)

mod data;
mod utils;

/// We re-export keybob's API here
mod keys {
    use traits::AutoEncoder;
    pub use keybob::{Key, KeyType};

    impl AutoEncoder for Key {}
    impl AutoEncoder for KeyType {}
}

pub use self::data::PackedData;
pub use self::keys::{Key, KeyType};
pub use self::utils::*;
