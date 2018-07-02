//! Shared cryptographic primitives and utilities
//! 
//! 

mod data;

/// We re-export keybob's API here
mod keys {
    pub use keybob::{Key, KeyType};
    use traits::AutoEncoder;

    impl AutoEncoder for Key {}
    impl AutoEncoder for KeyType {}
}

pub mod passwords;
pub mod encoding;
pub mod hashing;
pub mod random;

pub use self::data::PackedData;
pub use self::keys::{Key, KeyType};
