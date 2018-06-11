extern crate lockchain_core as lcc;
extern crate lockchain_files as files;

use files::DataVault;
use lcc::traits::Vault;
use lcc::{EncryptedBody, Payload, Record};
use std::env;

fn main() {
    if env::args().len() == 3 {
        let path = env::args().nth(1).unwrap();
        let name = env::args().nth(2).unwrap();

        let mut vault: DataVault<EncryptedBody> = DataVault::new(&name, &path);
        vault.meta_add_domain("userstore").unwrap();
        vault
            .meta_set(
                "userstore",
                "spacekookie",
                Payload::Text("<access token here>".into()),
            )
            .unwrap();
        vault.sync();
    } else {
        eprintln!("Usage: create <path> <name> [FLAGS] (there are no flags)")
    }
}
