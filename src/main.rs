extern crate chrono;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


mod vault;
use vault::*;

fn main() {
    let record = Record::new("facebook", "web");
    let j = serde_json::to_string(&record);
}
