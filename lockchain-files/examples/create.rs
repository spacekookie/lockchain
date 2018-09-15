extern crate lockchain_core as lcc;
extern crate lockchain_files as files;

use files::FileVault;
use lcc::traits::Vault;
use lcc::users::User;
use lcc::{EncryptedBody, Generator, Payload, Record, VaultType};
use std::env;

fn main() {
    if env::args().len() == 3 {
        let path = env::args().nth(1).unwrap();
        let name = env::args().nth(2).unwrap();

        let mut vault: FileVault<EncryptedBody> = Generator::new()
            .path(name, path)
            .user_type(VaultType::SoloUser {
                username: "spacekookie".into(),
                secret: "foobar3264".into(),
            }).finalise()
            .unwrap();
        vault.sync();

    // let vault: FileVault<EncryptedBody> = FileVault::new(&name, &path);

    // let mut store = match (
    //     vault.meta_pull_domain("userstore"),
    //     vault.meta_pull_domain("registry"),
    // ) {
    //     (Some(users), Some(registry)) => (users.clone(), registry.clone()).into(),
    //     _ => UserStore::default(),
    // };

    // /* Some users of our vault have the same password :S */
    // store.add(User::register("alice", "password"));
    // let token = store.get_token(vec!());

    // let (users, registry) = store.into();

    // vault.meta_push_domain(users);
    // vault.meta_push_domain(registry);
    // vault.sync();
    } else {
        eprintln!("Usage: create <path> <name> [FLAGS] (there are no flags)")
    }
}
