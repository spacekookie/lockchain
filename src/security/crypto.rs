//!


use miscreant::aead::{Aes128PmacSiv, Aes128Siv, Aes256PmacSiv, Aes256Siv, Algorithm};
use security::{keys::Key, random, encoding};
use serde::Serialize;
use serde_json;

struct Encryptor {

}


pub fn encrypt<T: Serialize>(data: &T) -> String {
    let encoded: String = serde_json::to_string(&data).unwrap();

    let key: Key = Key::new();
    let mut aes: Aes256Siv = Aes256Siv::new(&key.to_slice());
    
    let nonce = random::bytes(64);
    let ad = random::bytes(64);

    let encrypted = aes.seal(nonce.as_slice(), ad.as_slice(), encoded.as_bytes());
    return encoding::base64_encode(&encrypted);
}
