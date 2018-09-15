//! Very simple file system primitives

#![allow(dead_code)]

/// A set of files that exist inside a `FileVault`
pub enum FileType {
    /// A data record file
    Record,
    /// A MetaDomain file
    Metadata,
    /// A simple checksum file
    Checksum,
    /// _The_ config file
    Config,
    #[doc(hidden)]
    __NonExhaustive,
}

/// Construct a file ending for a specific match result
macro_rules! file_ending {
    ($type:expr) => {
        match $type {
            FileType::Record => "record",
            FileType::Metadata => "meta",
            FileType::Checksum => "sum",
            FileType::Config => "cfg"
            _ => "dat",
        }
    };
}

pub fn write_file(tt: FileType) {}

pub fn read_file() {}
