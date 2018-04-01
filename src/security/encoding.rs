//! Simple encoding submodule
//! 

use std::fmt::Write;
use std::str::from_utf8_unchecked;
use base64;

/// Encode a piece of arbitary data into a bse64 string
pub fn base64_encode(data: &Vec<u8>) -> String {
    return base64::encode(data);
}

/// Decode a base64 string into arbitrary data
pub fn base64_decode(data: &String) -> Vec<u8> {
    return base64::decode(data).unwrap();
}

/// Simply encode a byte-string as hexadecimal symbols
#[allow(unused)]
pub fn encode_hex(data: &str) -> String {
    let mut s = String::new();
    for &byte in data.as_bytes() {
        write!(&mut s, "{:X}", byte).expect("Unable to HEX encode!");
    }

    return s;
}
