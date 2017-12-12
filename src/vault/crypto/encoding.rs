//! Simple encoding submodule
//! 

use std::fmt::Write;
use base64;

/// Encode a byte string as base64 symbols
pub fn base64(data: &Vec<u8>) -> String {
    let mut encoded = String::new();
    let string = unsafe { String::from_utf8_unchecked(data.clone()) };
    base64::encode_config_buf(string.as_bytes(), base64::STANDARD, &mut encoded);
    return encoded;
}

/// Simply encode a byte-string as hexadecimal symbols
pub fn hex(data: &str) -> String {
    let mut s = String::new();
    for &byte in data.as_bytes() {
        write!(&mut s, "{:X}", byte).expect("Unable to HEX encode!");
    }

    return s;
}