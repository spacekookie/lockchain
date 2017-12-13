//! Simple encoding submodule
//! 

use std::fmt::Write;
use std::str::from_utf8_unchecked;
use base64;


/// Takes a utf-8 string of raw binary data and converts itto base64 encoded form
pub fn encode_base64(data: &str) -> String {
    return base64::encode(data.as_bytes());
}

/// Takes a base64 string and converts it to raw binary data
pub fn decode_base64(base64: &str) -> String {
    let vec = base64::decode(base64).unwrap();
    let decoded = unsafe { from_utf8_unchecked(&vec) };
    return String::from(decoded);
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