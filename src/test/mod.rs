//! Unit testing module for lockchain
//! 
//! Purpose of this module is to properly test all the functions involved 
//! in lockchain, since even a small bug can completely break crypto
//! 
#![allow(unused)]

use record::Record;
use serde_json;

#[test]
fn serialise_deserialise() {
    let r = Record::new("name", "category");
    let serial = serde_json::to_string(&r).unwrap();
    let deserial: Record = serde_json::from_str(&serial).unwrap();
    
    assert_eq!(r, deserial);
}