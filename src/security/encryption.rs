//! High level utility wrapper module around crypto calls
//! 

use super::aes::AES;
use super::keys;
use record::Record;
use serde_json;


// TODO: Use this implementation
pub trait Encryption {
    fn encrypt(&self, data: &Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, data: &Vec<u8>) -> Vec<u8>;
}


/// Wraps high-level utilities
pub struct CryptoHandler {
    core: AES,
    // _type: T,
}

impl CryptoHandler {

    pub fn new() {
        let k = keys::generate_key();

        let me = CryptoHandler {
            core: AES::new(&k)
        };
    }

    pub fn generic(&self) {
        
    }

}