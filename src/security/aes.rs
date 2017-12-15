//! Wrapper AES encryption, decryption functions
//! 
//! 


use aesni::{Aes128, BlockCipher};
use generic_array::GenericArray;
use std::str::from_utf8_unchecked;

use record::{Record, Header, Payload, Version};

pub struct AES {
    aes: Aes128,
}