//! Simple encoding submodule
//! 

use std::fmt::Write;

/// Encode a byte string as base64 symbols
pub fn base64(data: &str) -> String {
    return String::new();
}

/// Simply encode a byte-string as hexadecimal symbols
pub fn hex(data: &str) -> String {
    let mut s = String::new();
    for &byte in data.as_bytes() {
        write!(&mut s, "{:X}", byte).expect("Unable to HEX encode!");
    }

    return s;
}