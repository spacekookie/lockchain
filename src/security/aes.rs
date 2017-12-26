//! Primitive AES encryption module
//! 
//! This module has no concept of higher-level types. It only deals with vectors of raw data
//! and strings. It pads data wherever neccessary with symbols that are stripped out when 
//! converting it to higher level types.
//! 
//! This module is by no means perfect and shouldn't be considered incredibly secure.
//! The API can remain the same as features underneath are exchanged and hardnened.
//! 


use aesni::{Aes128, BlockCipher};
use generic_array::GenericArray;
use std::str::from_utf8_unchecked;

use super::keys::{KEY_LENGTH, Key};

/// Low-level wrapper around the AES block encrypt functions
pub struct AES {
    ctx: Aes128,
    pub key: Key,
}

impl AES {
    /// Create a new AES context from a key context
    pub fn new(key: &Key) -> AES {
        return AES {
            ctx: Aes128::new_varkey(&key.data).unwrap(),
            key: key.clone(),
        };
    }

    /// Encrypt a generic vector of data into another vector
    pub fn encrypt(&self, d: &Vec<u8>) -> Vec<u8> {
        let mut encrypted: Vec<u8> = Vec::new();
        let mut start: usize = 0;
        let mut stop: usize = KEY_LENGTH;

        /* Pad the data */
        let padded = AES::pad(&d);
        let padded_slice = padded.as_slice();

        loop {
            let slice = &padded_slice[start..stop];

            /* Encrypt the slice in place */
            let mut block = GenericArray::clone_from_slice(slice);
            self.ctx.encrypt_block(&mut block);

            for byte in block {
                encrypted.push(byte);
            }

            start = stop;
            stop += KEY_LENGTH;
            if padded.len() < stop {
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

    /* Some utility functions below */

    fn pad(data: &Vec<u8>) -> Vec<u8> {
        let mut padded = data.clone();

        if padded.len() % KEY_LENGTH == 0 {
            return padded;
        }

        let to_pad = data.len() + (data.len() % KEY_LENGTH);
        for _ in 1..to_pad {
            padded.push(' ' as u8);
        }

        return padded;
    }

    /// Convert a vector of u8 into a utf-8 string
    fn vec_to_str(vec: &[u8]) -> String {
        return unsafe { String::from(from_utf8_unchecked(vec)) };
    }
}
