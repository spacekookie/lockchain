//! A simple data layout

use lcc::traits::AutoEncoder;

/// Represents some packed data – includes nonce and blob
#[derive(Serialize, Deserialize)]
pub struct PackedData {
    pub nonce: Vec<u8>,
    pub iv: Vec<u8>,
    pub data: Vec<u8>,
}

impl AutoEncoder for PackedData {}