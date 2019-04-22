//! Implements an Aes256Siv encryption engine
//!
//! Can be initialised from scratch or with a pw/salt
//! combintaion which derives a key via the `keybob` crate.

use crate::lcc::crypto::{random, Key};
use crate::lcc::traits::{AutoEncoder, Encryptable, EncryptionHandler};
use crate::lcc::{EncryptedBody, PackedData};

use super::databody::DataBody;
use miscreant::{Aead, Aes256SivAead};

impl Encryptable for DataBody {}

pub struct AesEngine {
    ctx: Aes256SivAead,
    key: Key,
    iv: Vec<u8>,
}

impl AesEngine {
    /// Initialise an AesEngine and take ownership of a raw key
    pub fn new(key: Key) -> Self {
        assert!(key.len() == 64);

        Self {
            ctx: Aes256SivAead::new(&key.as_slice()),
            iv: random::bytes(64),
            key,
        }
    }

    fn encrypt_primitive(&mut self, data: &Vec<u8>) -> PackedData {
        let nonce = random::bytes(64);
        let iv = &self.iv.as_slice();
        let encrypted = self.ctx.seal(nonce.as_slice(), iv, data.as_slice());

        PackedData {
            iv: self.iv.clone(),
            data: encrypted,
            nonce: nonce,
        }
    }

    fn decrypt_primitive(&mut self, packed: &PackedData) -> Option<Vec<u8>> {
        let iv = &self.iv.as_slice();
        Some(
            self.ctx
                .open(packed.nonce.as_slice(), iv, packed.data.as_slice())
                .ok()?,
        )
    }
}

impl EncryptionHandler<DataBody> for AesEngine {
    fn encrypt(&mut self, item: DataBody) -> EncryptedBody {
        let ser = item.encode().unwrap();
        let data = self
            .encrypt_primitive(&ser.as_bytes().to_vec())
            .encode()
            .unwrap();
        EncryptedBody { data }
    }

    fn decrypt(&mut self, item: EncryptedBody) -> Option<DataBody> {
        let packed = PackedData::decode(&item.data).ok()?;
        Some(DataBody::decode(&String::from_utf8(self.decrypt_primitive(&packed)?).ok()?).ok()?)
    }
}
