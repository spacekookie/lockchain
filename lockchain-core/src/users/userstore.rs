//! Merging `KeyStore` and `Userstore` into the same concept

use super::rights::Access;
use std::collections::HashMap;
use traits::AutoEncoder;

/// A thin user UserStore
///
/// It's implementation can manage multiple keys per user, of various
/// types and constrained for limited access rights.
#[derive(Serialize, Deserialize)]
pub struct UserStore {
    store: HashMap<String, StoreUser>,
}

/// Internal store user structure
#[derive(Serialize, Deserialize)]
pub struct StoreUser {
    name: String,
    keys: HashMap<Access, Vec<u8>>,
}

impl AutoEncoder for UserStore {}

impl UserStore {
    /// Create a new, empty UserStore
    ///
    /// This is most likely *not* what you want. Instead, transform
    /// a `MetaData` object into a UserStore.
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    /// Adds a new user to the store, with a root-key
    pub fn add_user(&mut self, name: String, key: Vec<u8>) {
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
    pub fn add_key(&mut self, user: String, k: Vec<u8>, access: Access) {
        if !self.store.contains_key(&user) {
            return;
        }

        self.store.get_mut(&user).unwrap().keys.insert(access, k);
    }

    pub fn get_root_key(&self, user: &str) -> Option<&Vec<u8>> {
        self.store
            .get(user)
            .map_or(None, |u| u.keys.get(&Access::Root))
    }

    pub fn get_key(&self, user: &str, access: Access) -> Option<&Vec<u8>> {
        self.store.get(user).map_or(None, |u| u.keys.get(&access))
    }
}
