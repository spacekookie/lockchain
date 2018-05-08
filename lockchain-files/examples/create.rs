extern crate lockchain_core as lcc;
extern crate lockchain_files as files;

use lcc::{Record, EncryptedBody};
use lcc::traits::Vault;
use files::DataVault;
use std::env;

fn main() {
    if env::args().len() == 3 {
        let path = env::args().nth(1).unwrap();
        let name = env::args().nth(2).unwrap();

        let vault: DataVault<EncryptedBody> = DataVault::new(&name, &path);
    } else {
        eprintln!("Usage: create <path> <name> [FLAGS] (there are no flags)")
    }
}