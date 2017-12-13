//! A version of a record
//!
//! A set of version can be flattened to represent the latest set
//! of changes of a record

use super::Payload;
use std::collections::BTreeMap;

/// An operation that was applied to a version
///
/// An operation is either an insert or a delete.
/// It also carries a string key and a payload value inside
/// a tuple. These are then summed together as a vector.
///
/// This means that if data contradicts itself in the same
/// version the later edit (call) will override the previous
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Insert(String, Payload),
    Delete(String),
}

use self::Operation::{Insert, Delete};

/// Represents a series of operations done in sequence
/// that are applied to a record to preserve history of state
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    version: u64,
    ops: Vec<Operation>,
}

impl Version {

    /// Create a simple new version
    pub fn new(ver: u64) -> Version {
        return Version {
            version: ver,
            ops: Vec::new(),
        };
    }

    /// Take a version full of operations and flatten it to  a single
    /// binary search tree that can be included into an embedded record
    ///
    /// Non-mutable on the version itself
    pub fn flatten(&self) -> BTreeMap<String, Payload> {
        let mut map = BTreeMap::new();

        /* For all operations, process them in order */
        for op in &self.ops {

            /* Match the operation */
            match op {
                &Insert(ref key, ref payload) => {

                    /* Match the return to log errors */
                    match map.insert(key.clone(), payload.clone()) {
                        None => {}
                        _ => println!("Overriding value {}", key),
                    }
                }
                &Delete(ref key) => {

                    /* Match the return to log errors */
                    match map.remove(key) {
                        None => println!("Failed to apply deletion: key doesn't exists!"),
                        _ => {}
                    }
                }
            }
        }

        /* Return the map */
        return map;
    }

    /// A utility function which merges two versions onto &self
    /// 
    /// - If a key is present in `other`, `self.key` is overwritten
    /// - If a key is missing in `other`, `self.key` is deleted
    /// 
    pub fn merge(&mut self, other: &Version) {

    }
}
