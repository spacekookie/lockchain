//! Small utility module for file operations

use std::fs::File;
use std::io::{self, Read};

pub fn check_config() {}

/// A utility trait to read the conents from a file in
/// a single line.
pub trait FileToString {
    /// Read the file contents into a string without any
    /// error handling.
    fn get_string(&mut self) -> Result<String, io::Error>;
}

impl FileToString for File {
    fn get_string(&mut self) -> Result<String, io::Error> {
        let mut s = String::new();
        return match self.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        };
    }
}
