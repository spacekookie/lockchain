//! A utility keystore module for the lockchain ecosystem

use traits::{AutoEncoder, Base64AutoEncoder};
use {crypto::Key, meta::MetaDomain};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct KeyStore {
    keys: HashMap<String, Key>,
}

impl KeyStore {
    pub fn add_key(&mut self, user: &str, key: Key) {
        self.keys.insert(user.into(), key);
    }

    pub fn revoke_key(&mut self, user: &str) {
        self.keys.remove(user);
    }
}

impl AutoEncoder for KeyStore {}

impl From<MetaDomain> for KeyStore {
    fn from(d: MetaDomain) -> Self {
        Self {
            keys: d
                .all()
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        match v {
                            ::Payload::Text(s) => Key::decode(&String::from_base64(s)).unwrap(),
                            _ => unreachable!(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<KeyStore> for MetaDomain {
    fn from(ks: KeyStore) -> Self {
        MetaDomain::new("keystore").fill(
            ks.keys
                .iter()
                .map(|(name, key)| {
                    (
                        name.clone(),
                        ::Payload::Text(key.encode().unwrap().to_base64()),
                    )
                })
                .collect(),
        )
    }
}
