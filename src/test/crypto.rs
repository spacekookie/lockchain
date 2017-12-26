//! Testing crypto functions

use record::{Record, Payload, Version, Header};
use serde_json;

use security::{CryptoCtx, Encryptor};
use security::keys;


#[test]
fn header() {
    let h = Header::new("name".to_owned(), "category".to_owned());
    let ctx = CryptoHandler::new();
    let encrypted = ctx.encrypt(&h);
    let decrypted: Header = ctx.decrypt(encrypted);

    assert_eq!(h, decrypted);
}

#[test]
fn version() {
    let mut v = Version::new(0);
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("username", Payload::Text("jane".to_owned()));
    v.insert("password", Payload::Text("car battery horse staple".to_owned()));
    let serial = serde_json::to_string(&v).unwrap();

    let ctx = CryptoHandler::new();
    let encrypted = ctx.encrypt(&v);
    let decrypted: Version = ctx.decrypt(encrypted);

    assert_eq!(v, decrypted);
}

#[test]
fn record() {
    let mut r = Record::new("name", "category");
    r.add_tag("tag");
    r.set_data("username", Payload::Text("jane".to_owned()));
    r.set_data("password", Payload::Text("car battery horse staple".to_owned()));

    let ctx = CryptoHandler::new();
    let encrypted = ctx.encrypt(&r);
    let decrypted: Record = ctx.decrypt(encrypted);

    assert_eq!(r, decrypted);
}
