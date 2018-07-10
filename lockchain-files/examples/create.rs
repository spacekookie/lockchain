extern crate lockchain_core as lcc;
extern crate lockchain_files as files;

use files::DataVault;
use lcc::traits::Vault;
use lcc::users::{User, UserStore};
use lcc::{EncryptedBody, Payload, Record};
use std::env;

fn main() {
    if env::args().len() == 3 {
        let path = env::args().nth(1).unwrap();
        let name = env::args().nth(2).unwrap();

        let mut vault: DataVault<EncryptedBody> = DataVault::new(&name, &path);
        let mut store = match vault.meta_pull_domain("userstore") {
            Some(m) => m.clone().into(),
            _ => UserStore::default(),
        };

        /* Some users of our vault have the same password :S */
        store.add(User::register("alice", "password"));
        store.add(User::register("bob", "password"));
        store.add(User::register("carol", "password"));
        store.add(User::register("darius", "password"));
        store.add(User::register("elena", "password"));
        store.add(User::register("farah", "password"));

        vault.meta_push_domain(store.into());
        vault.sync();
    } else {
        eprintln!("Usage: create <path> <name> [FLAGS] (there are no flags)")
    }
}
