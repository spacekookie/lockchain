use super::rights::Access;
use super::secrets::SecretType;
use crypto::Key;
use std::collections::HashMap;

/// A thin user keystore
///
/// It's implementation can manage multiple keys per user, of various
/// types and constrained for limited access rights.
pub struct KeyStore {
    store: HashMap<String, StoreUser>,
}

struct StoreUser {
    name: String,
    keys: HashMap<Access, Key>,
}

impl KeyStore {
    /// Create a new, empty keystore
    ///
    /// This is most likely *not* what you want. Instead, transform
    /// a `MetaData` object into a keystore.
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add_user(&mut self) {}

    pub fn rm_user(&mut self) {}

    pub fn add_key(&mut self, user: String, k: Key, access: Access) {}

    pub fn get_key(&self, user: String, access: Access) -> &Key {
        unimplemented!()
    }
}
