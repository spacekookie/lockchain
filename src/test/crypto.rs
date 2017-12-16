//! Testing crypto functions

use record::{Record, Payload, Version, Header};
use security::engine::CryptoEngine;
use serde_json;

use security::aes::AES;
use security::keys;


#[test]
fn header() {
    let h = Header::new("name".to_owned(), "category".to_owned());
    let serial = serde_json::to_string(&h).unwrap();

    let c = CryptoEngine::new("my password is cheese", "");
    let encrypted = c.encrypt(&serial);
    let decrypted = c.decrypt(&encrypted);
    let deserial: Header = serde_json::from_str(&decrypted).unwrap();

    assert_eq!(h, deserial);
}

#[test]
fn version() {
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
fn record() {
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
fn test_new_crypto() {
    let mut r = Record::new("name", "category");
    r.add_tag("tag");
    r.set_data("username", Payload::Text("jane".to_owned()));
    r.set_data("password", Payload::Text("car battery horse staple".to_owned()));

    let k = keys::generate_key();
    let encrypted = AES::encrypt(&r, &k);
    let decrypted: Record = AES::decrypt(&encrypted, &k);

    assert_eq!(r, decrypted);
}