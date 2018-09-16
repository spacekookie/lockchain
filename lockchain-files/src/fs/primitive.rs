//! Very simple file system primitives

#![allow(dead_code)]

use std::fs::OpenOptions;
use std::io::{Read, Result, Write};
use std::path::PathBuf;

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
            &FileType::Record => "record",
            &FileType::Metadata => "meta",
            &FileType::Checksum => "sum",
            &FileType::Config => "cfg",
            _ => "dat",
        }
    };
}

#[inline]
fn type_path(tt: FileType, root: &PathBuf) -> PathBuf {
    use self::FileType::*;
    let mut path = root.clone();
    match tt {
        Record => path.push("records"),
        Metadata => path.push("metadata"),
        Checksum => path.push("checksums"),
        _ => path.push("."),
    };

    path
}

pub fn write_file(tt: FileType, root: PathBuf, name: &str, contents: Vec<u8>) -> Result<()> {
    let file_name = format!("{}.{}", name, file_ending!(&tt));
    let mut path = type_path(tt, &root);
    path.push(file_name);

    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    file.write_all(&contents)?;

    Ok(())
}

pub(crate) fn read_file(tt: FileType, root: PathBuf, name: &str) -> Result<Vec<u8>> {
    let file_name = format!("{}.{}", name, file_ending!(&tt));
    let mut path = type_path(tt, &root);
    path.push(file_name);

    let mut file = OpenOptions::new().read(true).create(false).open(path)?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).map(|_| buffer)
}
