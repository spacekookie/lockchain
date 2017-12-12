//! Crypto module for lockchain
//!
//!

use aesni::{Aes128, BlockCipher};
use generic_array::GenericArray;

pub mod hashing;
pub mod encoding;

const KEYLENGTH: usize = 16;


/// The crypto engine which holds the key and AES context
/// 
pub struct CryptoEngine {
    key: [u8; KEYLENGTH],
    aes: Aes128,
    iv: String,
}


impl CryptoEngine {
    pub fn new(password: &str, salt: &str) -> CryptoEngine {

        /* Make password to hash */
        let k = hashing::blake2_16(password, "");

        let me = CryptoEngine {
            key: k.clone(),
            aes: match Aes128::new_varkey(&k) {
                Ok(_aes) => _aes,
                Err(e) => panic!(e),
            },
            iv: String::from("lockchain"),
        };

        return me;
    }

    pub fn encrypt(&self, data: &str) -> {
        let to_encrypt = self.pad_data(data);

        let mut encrypted: Vec<u8> = Vec::new();
        let mut start: usize = 0;
        let mut stop: usize = KEYLENGTH;
        loop {
            let slice = to_encrypt[start..stop].as_bytes();

            /* Encrypt the slice in place */
            let mut block = GenericArray::clone_from_slice(&slice);
            self.aes.encrypt_block(&mut block);

            for byte in block {
                encrypted.push(byte);
            }

            start = stop;
            stop += KEYLENGTH;
            if to_encrypt.len() < stop {
                break;
            }
        }

        
    }

    pub fn decrypt(&self, data: &str) {}

    /// Pad a string to the block-size of the cipher
    ///
    /// This is a rather bad function and should be replaced with random
    /// data padding soon. But it works for now, I guess
    fn pad_data(&self, data: &str) -> String {

        if data.len() % KEYLENGTH == 0 {
            return String::from(data);
        }

        return format!(
            "{: <width$}",
            data,
            width = data.len() + (data.len() % KEYLENGTH)
        );
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
