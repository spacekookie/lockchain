//! Crypto engine implementation
//! 
//! 

use super::DEFAULT_KEYLENGTH;
use super::encoding;
use super::random;
use super::hash;

use aesni::{Aes256, BlockCipher};
use generic_array::GenericArray;
use std::str::from_utf8_unchecked;

/// The crypto engine which holds the key and AES context
///
pub struct CryptoEngine {
    encrypted_key: Option<String>,
    aes: Aes256,
}


impl CryptoEngine {

    /// Generate a new random key which is encrypted with the password
    pub fn new(password: &str, _: &str) -> CryptoEngine {

        /* Generate a random key */
        let secret_key = random::bytes(DEFAULT_KEYLENGTH);

        /* Encrypt secret_key with password */
        let k = hash::blake2_16(password, "");
        let tmp = CryptoEngine {
            encrypted_key: None,
            aes: Aes256::new_varkey(&k).unwrap(),
        };

        /* Encrypt and encode the secret key */
        let string = CryptoEngine::vec_to_str(&secret_key);
        let encrypted = tmp.encrypt(&string);
        let encoded = encoding::encode_base64(&encrypted);

        /* Then actually create an engine and return it */
        let me = CryptoEngine {
            encrypted_key: Some(encoded),
            aes: Aes256::new_varkey(&secret_key).unwrap(),
        };

        return me;
    }

    /// Load an existing vault with it's encrypted key and password
    pub fn load_existing(encrypted_key: &str, password: &str) -> CryptoEngine {

        /* Decrypt key with password */
        let k = hash::blake2_16(password, "");
        let tmp = CryptoEngine {
            encrypted_key: Some(String::from(encrypted_key)),
            aes: Aes256::new_varkey(&k).unwrap(),
        };

        /* Decode and decrypt key */
        let decoded = encoding::decode_base64(&encrypted_key);
        let decrypted = tmp.decrypt(&decoded);

        /* Then initialise a new crypto engine with the newly decrypted key */
        let me = CryptoEngine {
            encrypted_key: Some(String::from(encrypted_key)),
            aes: Aes256::new_varkey(&decrypted.as_bytes()).unwrap(),
        };

        return me;
    }

    /// Get the encrypted key that was used for a vault
    pub fn dump_encrypted_key(&self) -> Option<String> {
        return self.encrypted_key.clone();
    }

    /// Takes a simple utf-8 encoded string and encrypts it
    ///
    /// Outputs a base64 encoded string
    pub fn encrypt(&self, utf_8: &str) -> String {
        let to_encrypt = self.pad_data(&utf_8);

        let mut encrypted: Vec<u8> = Vec::new();
        let mut start: usize = 0;
        let mut stop: usize = 16;

        loop {
            let slice = to_encrypt[start..stop].as_bytes();

            /* Encrypt the slice in place */
            let mut block = GenericArray::clone_from_slice(slice);
            self.aes.encrypt_block(&mut block);

            for byte in block {
                encrypted.push(byte);
            }

            start = stop;
            stop += 16;
            if to_encrypt.len() < stop {
                break;
            }
        }

        return encoding::encode_base64(&CryptoEngine::vec_to_str(&encrypted));
    }


    /// Takes a base64 encoded, encrypted string and decrypts it
    ///
    /// Outputs a simple utf-8 string
    pub fn decrypt(&self, base64: &str) -> String {
        let mut decryted = String::new();
        let data = encoding::decode_base64(base64);
        let sliced = CryptoEngine::str_to_vec(&data);

        let mut start: usize = 0;
        let mut stop: usize = 16;

        loop {
            let slice = &sliced[start..stop];
            let mut block = GenericArray::clone_from_slice(slice);

            /* Encrypt block and push to collection */
            self.aes.decrypt_block(&mut block);
            decryted.push_str(&CryptoEngine::vec_to_str(&block));

            start = stop;
            stop += 16;
            if sliced.len() < stop {
                break;
            }
        }

        return decryted;
    }

    /// Convert a vector of u8 into a utf-8 string
    fn vec_to_str(vec: &[u8]) -> String {
        return unsafe { String::from(from_utf8_unchecked(vec)) };
    }

    /// Convert a utf-8 string to a vector of u8
    fn str_to_vec(string: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for b in string.as_bytes() {
            vec.push(*b);
        }
        return vec;
    }

    /// Pad a string to the block-size of the cipher
    ///
    /// This is a rather bad function and should be replaced with random
    /// data padding soon. But it works for now, I guess
    fn pad_data(&self, data: &str) -> String {

        if data.len() % DEFAULT_KEYLENGTH == 0 {
            return String::from(data);
        }

        return format!(
            "{: <width$}",
            data,
            width = data.len() + (data.len() % DEFAULT_KEYLENGTH)
        );
    }
}
