extern crate lockchain_core as lockchain;
extern crate lockchain_http;

fn main() {
    use lockchain::EncryptedBody;
    use lockchain_http::start_server;
    
    start_server::<EncryptedBody>("localhost", "8888");
}