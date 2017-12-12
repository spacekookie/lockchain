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
extern crate aesni;
extern crate blake2;
extern crate generic_array;


mod vault;
use vault::*;

use vault::Payload::Text;

/// This is a small example on how to use the lockchain API
///
/// This is by no means stable :')
fn main() {

    /* Create a new vault at a path, name and primary password */
    let mut vault = Vault::new(
        "Personal",
        "/home/spacekookie/Desktop",
        "my password is cheese",
    ).unwrap();

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
