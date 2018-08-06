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
mod utils;
pub use utils::*;
pub mod keystore;
pub mod store;

pub use self::data::PackedData;
pub use self::keystore::KeyStore;
pub use self::keys::{Key, KeyType};
