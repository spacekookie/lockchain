//! Simple AES module to do encryption
//!
//! Handles a lot of convertion automagically

use aesni::{Aes128, BlockCipher};
use generic_array::GenericArray;
use std::str::from_utf8_unchecked;

use record::Record;
use serde_json;

use super::keys::{KEY_LENGTH, Key};
use super::encoding;


/// Low-level wrapper around the AES block encrypt functions
pub struct AES {
    ctx: Aes128,
}

impl AES {
    
    /// Create a new AES context from a key context
    pub fn new(key: &Key) -> AES {
        return AES { ctx: Aes128::new_varkey(&key.data).unwrap() };
    }

    /// Encrypt a generic vector of data into another vector
    pub fn encrypt(&self, data: &Vec<u8>) -> Vec<u8> {
        let mut encrypted: Vec<u8> = Vec::new();
        let mut start: usize = 0;
        let mut stop: usize = KEY_LENGTH;

        let data_slice = data.as_slice();

        loop {
            let slice = &data_slice[start..stop];

            /* Encrypt the slice in place */
            let mut block = GenericArray::clone_from_slice(slice);
            self.ctx.encrypt_block(&mut block);

            for byte in block {
                encrypted.push(byte);
            }

            start = stop;
            stop += KEY_LENGTH;
            if encrypted.len() < stop {
                break;
            }
        }

        return encrypted;
    }

    pub fn decrypt(&self, vec: &Vec<u8>) -> String {
        let mut decrypted = String::new();
        let mut start: usize = 0;
        let mut stop: usize = KEY_LENGTH;

        loop {
            let slice = &vec[start..stop];
            let mut block = GenericArray::clone_from_slice(slice);

            /* Encrypt block and push to collection */
            self.ctx.decrypt_block(&mut block);
            decrypted.push_str(&AES::vec_to_str(&block));

            start = stop;
            stop += KEY_LENGTH;
            if vec.len() < stop {
                break;
            }
        }

        return decrypted;
    }

    // Easy to use encryption function
    //
    // Takes a record and a key, then returns a base64 encoded encrypted string
    // pub fn encrypt2(record: &Record, key: &Key) -> String {
    //     let aes = Aes128::new_varkey(&key.data).unwrap();
    //     let encoded = serde_json::to_string(&record).unwrap();

    //     let encrypted = AES::encrypt_string(&encoded, &aes);
    //     let base64 = encoding::base64_encode(&encrypted);

    //     return base64;
    // }

    // Easy to use decryption function
    //
    // Takes a base64 encoded string and key, then returns a record object
    // pub fn decrypt(data: &String, key: &Key) -> Record {
    //     let aes = Aes128::new_varkey(&key.data).unwrap();
    //     let decoded = encoding::base64_decode(data);

    //     let decrypted = AES::decrypt_vector(&decoded, &aes);
    //     let record: Record = serde_json::from_str(&decrypted).unwrap();

    //     return record;
    // }


    /* Some utility functions below */

    fn pad_string(data: &str) -> String {
        if data.len() % KEY_LENGTH == 0 {
            return String::from(data);
        }

        return format!(
            "{: <width$}",
            data,
            width = data.len() + (data.len() % KEY_LENGTH)
        );
    }

    /// Convert a vector of u8 into a utf-8 string
    fn vec_to_str(vec: &[u8]) -> String {
        return unsafe { String::from(from_utf8_unchecked(vec)) };
    }


    // fn encrypt_string(utf_8: &str, ctx: &Aes128) -> Vec<u8> {
    //     let to_encrypt = AES::pad_string(utf_8);
    //     let mut encrypted: Vec<u8> = Vec::new();
    //     let mut start: usize = 0;
    //     let mut stop: usize = KEY_LENGTH;

    //     loop {
    //         let slice = to_encrypt[start..stop].as_bytes();

    //         /* Encrypt the slice in place */
    //         let mut block = GenericArray::clone_from_slice(slice);
    //         ctx.encrypt_block(&mut block);

    //         for byte in block {
    //             encrypted.push(byte);
    //         }

    //         start = stop;
    //         stop += KEY_LENGTH;
    //         if to_encrypt.len() < stop {
    //             break;
    //         }
    //     }

    //     return encrypted;
    // }

    // fn decrypt_vector(vec: &Vec<u8>, ctx: &Aes128) -> String {
    //     let mut decrypted = String::new();
    //     let mut start: usize = 0;
    //     let mut stop: usize = KEY_LENGTH;

    //     loop {
    //         let slice = &vec[start..stop];
    //         let mut block = GenericArray::clone_from_slice(slice);

    //         /* Encrypt block and push to collection */
    //         ctx.decrypt_block(&mut block);
    //         decrypted.push_str(&AES::vec_to_str(&block));

    //         start = stop;
    //         stop += KEY_LENGTH;
    //         if vec.len() < stop {
    //             break;
    //         }
    //     }

    //     return decrypted;
    // }
}