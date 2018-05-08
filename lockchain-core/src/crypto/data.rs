use traits::AutoEncoder;

/// Represents some encrypted packed data
/// 
/// Includes nonce, vault iv and blob. This abstraction
/// is important to be able to send encrypted records across
/// a network.
#[derive(Serialize, Deserialize)]
pub struct PackedData {
    pub nonce: Vec<u8>,
    pub iv: Vec<u8>,
    pub data: Vec<u8>,
}

impl AutoEncoder for PackedData {}