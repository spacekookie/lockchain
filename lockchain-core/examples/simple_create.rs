extern crate lockchain_core as lockchain;
use lockchain::*;

fn main() {
    let mut v = Vault::new(
        "some_vault",
        "/home/spacekookie",
        "my password is super cool",
    ).unwrap();

    v.add_record("icarus", "Keys", Vec::new());
    v.add_data(
        "icarus",
        "key",
        Payload::Text("My key data here .........................................".to_owned()),
    );
    v.sync();
}
