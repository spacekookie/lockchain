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
    /// Adds a new user to the store, with a root-key
    pub fn add_user(&mut self, name: String, key: Key) {
        let mut user = StoreUser {
            name: name.clone(),
            keys: HashMap::new(),
        };
        user.keys.insert(Access::Root, key);
        self.store.insert(name, user);
    }
    /// Delete a user from this store
    pub fn del_user(&mut self, name: &str) {
        self.store.remove(name);
    }
    /// Add a key to an existing user
    pub fn add_key(&mut self, user: String, k: Key, access: Access) {
        if !self.store.contains_key(&user) {
            return;
        }

        self.store.get_mut(&user).unwrap().keys.insert(access, k);
    }

    pub fn get_key(&self, user: String, access: Access) -> Option<&Key> {
        self.store.get(&user).map_or(None, |u| u.keys.get(&access))
    }
}
