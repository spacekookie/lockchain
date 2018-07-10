//! User registry in a vault is done via the metadata store
//! 
//! In addition to that the `lockchain-core ` provides some simple
//! utilities to manage Users and UserStore objects, mapping them
//! onto metadata stores.
//! 
//! In this example we will define a function that takes a generic
//! Vault implementation backend (because lockchain-core doesn't
//! provide a concrete way of doing this) and registering a user 
//! into it.
//! 
//! Please note you can't actually _run_ this code example, because
//! no concrete type can be known. The exact same example (with a type)
//! can however be found in `lockchain-files`


extern crate lockchain_core as lockchain;
use lockchain::users::{User, UserStore, Access, Role};
use lockchain::traits::Vault;
use lockchain::EncryptedBody;

fn main() {
    // register(your_vault_here, "spacekookie", "password");
}

/// This function takes a generic Vault which MUST implement
/// the EncryptedBody backend. This would normally be the case
/// for the `DataVault` provided by `lockchain-files`
fn register<V: Vault<EncryptedBody>>(vault: &mut V, username: &str, password: &str) {
    let me = User::register(username, password);
}


