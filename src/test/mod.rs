//! Unit testing module for lockchain
//! 
//! Purpose of this module is to properly test all the functions involved 
//! in lockchain, since even a small bug can completely break crypto
//! 
#![allow(unused)]

use record::{Record, Payload, Version, Header};
use crypto::engine::CryptoEngine;
use vault::Vault;
use serde_json;

#[test]
fn serialise_record_simple() {
    let r = Record::new("name", "category");
    let serial = serde_json::to_string(&r).unwrap();
    let deserial: Record = serde_json::from_str(&serial).unwrap();
    
    assert_eq!(r, deserial);
}


#[test]
fn serialise_record_data() {
    let mut r = Record::new("name", "category");
    r.add_tag("tag");
    r.set_data("username", Payload::Text("jane".to_owned()));
    r.set_data("password", Payload::Text("car battery horse staple".to_owned()));

    let serial = serde_json::to_string(&r).unwrap();
    let deserial: Record = serde_json::from_str(&serial).unwrap();
    
    assert_eq!(r, deserial);
}

#[test]
fn serialise_version() {
    let mut v = Version::new(0);
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("password", Payload::Text("car battery horse staple".to_owned()));
    
    let serial = serde_json::to_string(&v).unwrap();
    let deserial: Version = serde_json::from_str(&serial).unwrap();

    assert_eq!(v, deserial);
}

#[test]
fn encrypt_header() {
    let h = Header::new("name".to_owned(), "category".to_owned());
    let serial = serde_json::to_string(&h).unwrap();

    let c = CryptoEngine::new("my password is cheese", "");
    let encrypted = c.encrypt(&serial);
    let decrypted = c.decrypt(&encrypted);
    let deserial: Header = serde_json::from_str(&decrypted).unwrap();

    assert_eq!(h, deserial);
}

#[test]
fn encrypt_version() {
    let mut v = Version::new(0);
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("password", Payload::Text("car battery horse staple".to_owned()));
    let serial = serde_json::to_string(&v).unwrap();

    let c = CryptoEngine::new("my password is cheese", "");
    let encrypted = c.encrypt(&serial);
    let decrypted = c.decrypt(&encrypted);

    let deserial: Version = serde_json::from_str(&decrypted).unwrap();

    assert_eq!(v, deserial);
}

#[test]
fn encrypt_record() {
    let mut r = Record::new("name", "category");
    r.add_tag("tag");
    r.set_data("username", Payload::Text("jane".to_owned()));
    r.set_data("password", Payload::Text("car battery horse staple".to_owned()));
    let serial = serde_json::to_string(&r).unwrap();

    let c = CryptoEngine::new("my password is cheese", "");
    let encrypted = c.encrypt(&serial);
    let decrypted = c.decrypt(&encrypted);

    let deserial: Record = serde_json::from_str(&decrypted).unwrap();

    assert_eq!(r, deserial);
}

#[test]
fn storage_lifecycle() {
    let mut v: Vault = Vault::new("lockchain_testing", "/tmp/", "password").unwrap();
    v.add_record("name", "category", vec!["test"]);
    v.add_data("name", "key", Payload::Text("value".to_owned()));
    v.sync();

    let v2: Vault = Vault::load("lockchain_testing", "/tmp/", "password");
    assert_eq!(v.records, v2.records);
}