//! Utility module which handles filesystem writes

use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use lcc::traits::AutoEncoder;

use lcc::Record;

pub struct Filesystem {
    name: String,
    path: String,
    root: PathBuf,
}

pub enum FileType<T> {
    Record(T),
    Metadata(T),
    Checksum(T)
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
    pub fn fetch<T: AutoEncoder>(types: FileType<T>) -> Vec<T> {
        unimplemented!()
    }
}
