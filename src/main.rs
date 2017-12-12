//! This will become the lockchain library crate at some point
//!
//! For now it's a hybrid between a library and a Gtk+ UI

extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate base64;
extern crate rand;
extern crate aesni;
extern crate blake2;
extern crate generic_array;


mod vault;
use vault::*;

fn main() {

    let vault = vault::Vault::new("default", "/home/spacekookie/Desktop", "my password is cheese");

    // let record = Record::new("facebook", "web");
    // let j = serde_json::to_string(&record).unwrap();

    // /* Encrypt the data */
    // let crypto = crypto::CryptoEngine::new("My password is cheese with honey", "");
    // let encrypted = crypto.encrypt(&j);

    // /* Encode it as base64 */
    // let mut encoded = String::new();
    // let string = unsafe { String::from_utf8_unchecked(encrypted.clone()) };
    // base64::encode_config_buf(string.as_bytes(), base64::STANDARD, &mut encoded);

    // /* Then decode it and compare */
    // let decoded = base64::decode(&encoded).unwrap();
    // println!("Decoded == Encrypted: {}", decoded == encrypted);

    // /* Then decrypt it and compare */
    // let decrypted = crypto.decrypt(&decoded);
    // let recovered: Record = serde_json::from_str(&decrypted).unwrap();
    // println!("Recovered == Record: {:?}", recovered == record);
}
