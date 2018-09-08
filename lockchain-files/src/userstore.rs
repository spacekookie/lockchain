//! This module maps the lockchain internal `UserStore` into a
//! structure that can be saved to disk.

use lcc::users::UserStore;

#[derive(Debug)]
pub struct UserStoreMapper {
    inner: UserStore,
}

impl UserStoreMapper {
    pub fn new(store: UserStore) -> Store {
        Self { inner: store }
    }
}
