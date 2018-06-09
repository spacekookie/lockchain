extern crate lockchain_core;
extern crate lockchain_files;
extern crate lockchain_http;

extern crate clap;

fn main() {
    use lockchain_core::{traits::*, EncryptedBody};
    use lockchain_http::create_server;
    use lockchain_files::DataVault;

    let server = create_server(
        "localhost",
        "8080",
        DataVault::<EncryptedBody>::new("name", "location"),
    );
    server.run();

    println!("After the server died!");
}
