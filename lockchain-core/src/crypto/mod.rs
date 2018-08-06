//! Shared cryptographic primitives and utilities
//! 
//! 


/// We re-export keybob's API here
mod keys {
    pub use keybob::{Key, KeyType};
    use traits::AutoEncoder;

    impl AutoEncoder for Key {}
    impl AutoEncoder for KeyType {}
}

mod data;
pub mod encoding;
pub mod random;
pub mod hashing;

pub use self::data::PackedData;
pub use self::keys::{Key, KeyType};
pub use users::KeyStore;