use traits::AutoEncoder;

/// Representation of encrypted data as an enecoded format
/// 
/// Includes all cryptographic state primitives that are
/// required to send the data over a network and decrypt on
/// the other side of a pipe.
#[derive(Serialize, Deserialize)]
pub struct PackedData {
    pub nonce: Vec<u8>,
    pub iv: Vec<u8>,
    pub data: Vec<u8>,
}

impl AutoEncoder for PackedData {}