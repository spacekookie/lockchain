//! Crypto module for lockchain
//!
//!

use aesni::{Aes128, BlockCipher};

pub mod hashing;
pub mod encoding;

struct CryptoEngine {
    key: [u8; 16],
    aes: Aes128,
    iv: String,
}

impl CryptoEngine {
    pub fn new(password: &str, salt: &str) -> CryptoEngine {

        /* Make password to hash */
        let k = hashing::blake2_16(password, "");

        return CryptoEngine {
            key: k.clone(),
            aes: match Aes128::new_varkey(&k) {
                Ok(_aes) => _aes,
                Err(e) => panic!(e),
            },
            iv: String::from("lockchain"),
        };
    }
}



pub fn aes256_encrypt() {
    let password = "1234567890123456";
    // GenericArray::from_slice(password.as_bytes()); // [0u8; 16]

}

// pub fn aes256_decrypt(bh: &mut test::Bencher) {
//     let cipher = Aes256::new(&Default::default());
//     let mut input = Default::default();

//     bh.iter(|| {
//         cipher.decrypt_block(&mut input);
//         test::black_box(&input);
//     });
//     bh.bytes = input.len() as u64;
// }
