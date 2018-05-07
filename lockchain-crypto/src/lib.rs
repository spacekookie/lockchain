//!

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate lockchain_core as lcc;

use lcc::{Payload, EncryptedBody};
use lcc::traits::{Body, AutoEncoder, Base64AutoEncoder, Encryption};

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct DataBody {
    tree: BTreeMap<String, Payload>,
}

impl AutoEncoder for DataBody {}

impl Body for DataBody {
    fn get_field(&self, key: &str) -> Option<&Payload> {
        self.tree.get(key).as_ref()?
    }

    fn set_field(&mut self, key: &str, value: Payload) -> Option<()> {
        self.tree.insert(key, value);
        Some(())
    }

    fn flatten(&mut self) -> Option<()> {
        None
    }
}

impl Encryption for DataBody {
    fn encrypt(&mut self) -> EncryptedBody {
        EncryptedBody {
            cipher: self.encode()
        }
    }
}