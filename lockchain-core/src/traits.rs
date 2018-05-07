//! Common vault traits for plugin-crates
//! 
//! The core of this crate has no functionality and is dependant
//! on other libraries to fill those holes. To make this easer
//! (and sometimes possible), we defined a few common behaviours
//! in traits to expand on in implementations specific to the
//! library.
//! 
//! Each trait is documented in more detail and provides default
//! implementations with `unimplemented!` macros to make 
//! compilation work without external crates but not calling
//! functions at runtime.

use record::{Header, Body};

/// A simple trait that allows libraries to hook into the
/// `body()` and `record()` hooks for vault records.
pub trait LoadRecord {
    fn header() -> Header {
        unimplemented!()
    }

    fn body() -> Body {
        unimplemented!()
    }
}


/// This is a trait which needs to be implemented by any
/// backend which hopes to do encryption on data.
pub trait Encryption {
    fn encrypt(&mut self) -> Vec<u8> {
        unimplemented!()
    }

    fn decrypt(_: Vec<u8>) -> Box<Self> {
        unimplemented!()
    }
}

/// A trait that abstracts file or record loading for
/// any backend which wants to implement storage functions
pub trait Loading {
    fn load(_path: &str) -> Box<Self> {
        unimplemented!()
    }

    fn save(&mut self, _path: &str) {
        unimplemented!()
    }
}