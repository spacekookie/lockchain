//! Simple encoding submodule
//! 

/// Encode a byte string as base64 symbols
pub fn base64(data: &str) -> String {

}

/// Simply encode a byte-string as hexadecimal symbols
pub fn hex(data: &str) -> String {
    let mut s = String::new();
    for &byte in &buf {
        write!(&mut s, "{:X}", byte).expect("Unable to HEX encode!");
    }

    return s;
}