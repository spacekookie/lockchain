extern crate lockchain_core as lockchain;
extern crate lockchain_files as files;
extern crate lockchain_http as http;

extern crate clap;

use files::*;
use http::*;
use lockchain::traits::*;
use lockchain::EncryptedBody;

fn foo() -> DataVault<EncryptedBody> {
    DataVault::new("name", "location")
}

fn main() {
    let server = create_server(foo());
    server.run();

    println!("After the server died!");
}
