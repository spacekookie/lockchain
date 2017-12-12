//! Crypto module for lockchain
//!
//!

use aesni::{Aes128, BlockCipher};
use generic_array::GenericArray;
use std;

use rand::{thread_rng, Rng};

pub mod hashing;
pub mod encoding;

const KEYLENGTH: usize = 16;


/// The crypto engine which holds the key and AES context
///
pub struct CryptoEngine {
    encrypted_key: Option<String>,
    aes: Aes128,
    iv: String,
}


impl CryptoEngine {

    /// Generate a new random key which is encrypted with the password
    pub fn new(password: &str, _: &str) -> CryptoEngine {

        /* Generate some random key */
        let mut random_data = [0u8; 13579];
        thread_rng().fill_bytes(&mut random_data);

        /* Move key around */
        let mut secret_key = [0u8; KEYLENGTH];
        for i in 0..KEYLENGTH {
            secret_key[i] = random_data[i];
        }

        /* Encrypt secret_key with password */
        let k = hashing::blake2_16(password, "");
        let tmp = CryptoEngine {
            encrypted_key: None,
            aes: Aes128::new_varkey(&k).unwrap(),
            iv: String::from("unused"),
        };

        let encryted_key_formatted = unsafe { std::str::from_utf8_unchecked(&secret_key) };
        let encrypted_key = tmp.encrypt(encryted_key_formatted);
        let string = unsafe { String::from_utf8_unchecked(encrypted_key.clone()) };
        let encrypted_key_encoded = encoding::hex(&string);

        /* Then actually create an engine and return it */
        let me = CryptoEngine {
            encrypted_key: Some(encrypted_key_encoded),
            aes: Aes128::new_varkey(&secret_key).unwrap(),
            iv: String::from("unused"),
        };

        return me;
    }

    /// Load an existing vault with it's encrypted key and password
    pub fn load_existing(encrypted_key: &str, password: &str) -> CryptoEngine {
        
        /* Decrypt key with password */
        let k = hashing::blake2_16(password, "");
        let tmp = CryptoEngine {
            encrypted_key: Some(String::from(encrypted_key)),
            aes: Aes128::new_varkey(&k).unwrap(),
            iv: String::from("unused"),
        };

        let floobar = tmp.encrypted_key.clone().unwrap();
        let key_as_bytes = floobar.as_bytes();

        let mut key_vector: Vec<u8> = Vec::new();
        for byte in key_as_bytes {
            key_vector.push(*byte);
        }
        let decrypted = tmp.decrypt(&key_vector);
        
        /* Then initialise a new crypto engine with the newly decrypted key */
        let me = CryptoEngine {
            encrypted_key: Some(String::from(encrypted_key)),
            aes: Aes128::new_varkey(&decrypted.as_bytes()).unwrap(),
            iv: String::from("unused")
        };

        return me;
    }

    /// Get the encrypted key that was used for a vault
    pub fn dump_encrypted_key(&self) -> Option<String> {
        return self.encrypted_key.clone();
    }

    pub fn encrypt(&self, data: &str) -> Vec<u8> {
        let to_encrypt = self.pad_data(data);

        let mut encrypted: Vec<u8> = Vec::new();
        let mut start: usize = 0;
        let mut stop: usize = KEYLENGTH;

        loop {
            let slice = to_encrypt[start..stop].as_bytes();

            /* Encrypt the slice in place */
            let mut block = GenericArray::clone_from_slice(slice);
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

        return encrypted;
    }

    pub fn decrypt(&self, data: &Vec<u8>) -> String {
        let mut decryted = String::new();
        let sliced = data.as_slice();

        let mut start: usize = 0;
        let mut stop: usize = KEYLENGTH;

        loop {
            let slice = &sliced[start..stop];
            let mut block = GenericArray::clone_from_slice(slice);
            self.aes.decrypt_block(&mut block);

            match std::str::from_utf8(&block) {
                Ok(string) => decryted.push_str(string),
                Err(err) => panic!("Failed to decode: {}", err),
            }

            start = stop;
            stop += KEYLENGTH;
            if sliced.len() < stop {
                break;
            }
        }

        return decryted;
    }

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
