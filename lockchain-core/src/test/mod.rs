//! Unit testing module for lockchain
//! 
//! Purpose of this module is to properly test all the functions involved 
//! in lockchain, since even a small bug can completely break crypto
//! 
#![allow(unused)]

mod crypto;
mod serialize;

use std::fs;
use std::path::Path;
use record::Payload;
use vault::Vault;

// #[test]
fn storage_lifecycle() {
    let mut v: Vault = Vault::new("lockchain_testing", "/tmp/", "password").unwrap();
    v.add_record("name", "category", vec!["test"]);
    v.add_data("name", "key", Payload::Text("value".to_owned()));
    v.sync();

    let v2: Vault = Vault::load("lockchain_testing", "/tmp/", "password");
    fs::remove_dir_all(Path::new("/tmp/lockchain_testing.vault/")).unwrap();

    // assert_eq!(v.records, v2.records);
}