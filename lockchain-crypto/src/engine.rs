//!

use lcc::{traits::{AutoEncoder, Encryptable, EncryptionHandler},
          EncryptedBody};
use miscreant::aead::{Aes256Siv, Algorithm};

use super::data::PackedData;
use super::databody::DataBody;
use super::{keys::{Key, KEY_LENGTH},
            utils::random};

use std::collections::BTreeMap;

impl Encryptable for DataBody {}

pub struct AesEngine {
    ctx: Aes256Siv,
    key: Key,
    iv: Vec<u8>,
}

impl AesEngine {
    /// Generate new key and encryption engine
    pub fn generate() -> Self {
        let key = Key::generate();
        Self {
            ctx: Aes256Siv::new(&key.to_slice()),
            key,
            iv: random::bytes(KEY_LENGTH),
        }
    }

    pub fn from_pw(pw: &str, salt: &str) -> Self {
        let key = Key::from_password(pw, salt);
        Self {
            ctx: Aes256Siv::new(&key.to_slice()),
            key,
            iv: random::bytes(KEY_LENGTH),
        }
    }
}

impl EncryptionHandler<DataBody> for AesEngine {
    fn encrypt(&mut self, item: DataBody) -> EncryptedBody {
        let ser = item.encode();
        let nonce = random::bytes(64);
        let iv = &self.iv.as_slice();
        let data = &ser.as_bytes();

        let encrypted = self.ctx.seal(nonce.as_slice(), iv, data);
        let data = PackedData {
            iv: self.iv.clone(),
            data: encrypted,
            nonce: nonce,
        }.encode();

        EncryptedBody { data }
    }

    fn decrypt(&mut self, item: EncryptedBody) -> Option<DataBody> {
        let packed = PackedData::decode(&item.data);
        let iv = &self.iv.as_slice();
        let decrypted = self.ctx
            .open(packed.nonce.as_slice(), iv, packed.data.as_slice())
            .ok()?;

        Some(DataBody::decode(&String::from_utf8(decrypted).ok()?))
    }
}
