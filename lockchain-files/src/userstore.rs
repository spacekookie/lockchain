//! This module maps the lockchain internal `UserStore` into a
//! structure that can be saved to disk.

use lcc::users::UserStore;

pub struct UserStoreMapper {
    inner: UserStore,
}

impl UserStoreMapper {
    pub fn new() -> Self {
        Self {
            inner: UserStore::new(),
        }
    }

    pub fn load(store: UserStore) -> Self {
        Self { inner: store }
    }
}
