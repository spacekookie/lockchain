//! A clear-text representation of a record body in memory
//! 
//! This form is created by the `lockchain-crypto` crate and
//! should only exist in ephemeral form. All actions are first
//! encrypted before being written back to a persistence
//! medium.

use lcc::traits::{AutoEncoder, Body};
use lcc::Payload;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct DataBody {
    tree: BTreeMap<String, Payload>,
}

impl DataBody {
    pub fn new() -> Self {
        DataBody {
            tree: BTreeMap::new(),
        }
    }
}

impl AutoEncoder for DataBody {}

impl Body for DataBody {
    fn get_field(&self, key: &str) -> Option<&Payload> {
        self.tree.get(key)
    }

    fn set_field(&mut self, key: &str, value: Payload) -> Option<()> {
        self.tree.insert(key.to_owned(), value)?;
        Some(())
    }

    fn flatten(&mut self) -> Option<()> {
        None
    }
}
