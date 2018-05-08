//! Utility module which handles filesystem writes

use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use lcc::traits::AutoEncoder;

pub struct Filesystem {
    name: String,
    path: String,
    root: PathBuf,
}

/// A switching enum to determine what type of file to load
pub enum FileType {
    /// A data record file
    Record,
    /// A vault/ zser metadata file 
    Metadata,
    /// A simple checksum file
    Checksum
}

impl Filesystem {
    pub fn create(path: &str, name: &str) -> Filesystem {
        let mut buffer = PathBuf::new();
        buffer.push(path);
        buffer.push(format!("{}.vault", name));

        Filesystem {
            name: name.to_owned(),
            path: path.to_owned(),
            root: buffer,
        }
    }

    /// Create required directories
    pub fn scaffold(&self) -> Option<()> {
        fs::create_dir_all(&self.root).ok()?;
        fs::create_dir(&self.root.join("records")).ok()?;
        fs::create_dir(&self.root.join("metadata")).ok()?;
        fs::create_dir(&self.root.join("checksums")).ok()?;
        Some(())
    }

    /// Load all files of a certain type into a Vec<String>
    pub fn fetch<T: AutoEncoder>(&self, types: FileType) -> Vec<T> {
        unimplemented!()
    }

    /// Load a single file of a certain type
    pub fn pull<T: AutoEncoder>(&self, ftype: FileType, id: &str) -> T {
        unimplemented!()
    }
}
