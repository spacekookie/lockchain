//! This will become the lockchain library crate at some point
//!
//! For now it's a hybrid between a library and a Gtk+ UI

extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate rand;
extern crate blake2;
extern crate miscreant;


pub mod record;
pub mod vault;
mod security;
mod test;

use vault::*;
use record::Payload::Text;


fn main() {

    let rec = record::Record::new("name", "category");

    let encrypted = security::crypto::encrypt(&rec);
    println!("Encrypted: {}", encrypted);

    // security::bla();

    // create_and_populate();
    // load();
}


fn load() {

    let vault = Vault::load(
        "Personal",
        "/home/spacekookie/Desktop",
        "my password is cheese",
    );
    println!("{:?}", vault.records);

}

fn create_and_populate() {

    /* Create a new vault at a path, name and primary password */
    let mut vault = match Vault::new(
        "Personal",
        "/home/spacekookie/Desktop",
        "my password is cheese",
    ) {
        Ok(s) => s,
        Err(e) => panic!("Somehow failed to create the vault because {:?}", e),
    };

    /* Add a record with some tags */
    vault.add_record("mastodon", "web", vec!["social", "network"]);

    /* Add a few data fields to the body */
    vault.add_data(
        "mastodon",
        "url",
        Text(String::from("https://mastodon.social")),
    );
    vault.add_data("mastodon", "user", Text(String::from("spacekookie")));
    vault.add_data(
        "mastodon",
        "password",
        Text(String::from("My password is molten cheese")),
    );

    /* Sync the changes to disk */
    vault.sync();
}