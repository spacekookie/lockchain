//! Filesystem abstraction for various data types
//!
//! All operations return io::Result<()> to indicate errors
//! and functions that have multiple file endpoints will return
//! a folded error list to indicate which ops were successful
//! and which failed.
//!
//! There is also a `From<Vec<?>> for Result<?>` implementation
//! which will return either `Ok(())` or the first error in the list
//! of operations.

use lcc::traits::AutoEncoder;

use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::{
    fs::{self, File, OpenOptions as OO},
    path::PathBuf,
};

use utils::FileToString;

#[derive(Debug)]
pub struct Filesystem {
    pub name: String,
    pub path: String,
    pub root: PathBuf,
}

/// A switching enum to determine what type of file to load
#[allow(dead_code)]
pub enum FileType {
    /// A data record file
    Record,
    /// A MetaDomain file
    Metadata,
    /// A simple checksum file
    Checksum,
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
            _ => "dat",
        }
    };
}

impl Filesystem {
    /// Create a new filesystem representation
    ///
    /// This function does _not_ touch the disk!
    pub fn new(path: &str, name: &str) -> Self {
        let mut buffer = PathBuf::new();
        buffer.push(path);
        buffer.push(format!("{}.vault", name));

        Self {
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
    pub fn fetch<T: AutoEncoder>(&self, types: FileType) -> Result<Vec<T>, Box<Error>> {
        Ok(fs::read_dir(match types {
            FileType::Record => self.root.join("records"),
            FileType::Metadata => self.root.join("metadata"),
            _ => self.root.clone(),
        })?.into_iter()
        .filter_map(|r| r.ok())
        .filter(|f| match f.file_type() {
            Ok(vf) => vf.is_file(),
            _ => false,
        }).map(|de| de.path())
        .filter_map(|p| p.into_os_string().into_string().ok())
        .filter_map(|s| File::open(s).ok())
        .filter_map(|mut f| f.get_string().ok())
        .filter_map(|s| T::decode(&s).ok())
        .collect())
    }

    /// Retrieve a single record from the cached vault
    pub fn pull<T: AutoEncoder>(&self, types: FileType, id: &str) -> Result<T, Box<Error>> {
        Ok(T::decode(
            &File::open(self.root.join(&format!("{}.{}", id, file_ending!(types))))?
                .get_string()?,
        )?)
    }

    /// Respond to a sync request
    pub fn sync<T>(&self, data: &HashMap<String, T>, types: FileType) -> Result<(), Box<Error>>
    where
        T: AutoEncoder,
    {
        data.into_iter()
            .map(|(k, v)| (k, v.encode().ok()))
            .map(|(k, v)| {
                (
                    match types {
                        FileType::Record => self.root.join("records"),
                        FileType::Metadata => self.root.join("metadata"),
                        _ => self.root.join("."),
                    }.join(format!("{}.{}", k, file_ending!(types))),
                    v,
                )
            }).filter(|(_, v)| v.is_some())
            .map(|(k, v)| (k, v.unwrap()))
            .map(|(path, data): (PathBuf, String)| {
                (OO::new().create(true).write(true).open(path), data)
            }).filter(|(path, _)| path.is_ok())
            .map(|(file, data)| (file.unwrap(), data))
            .for_each(|(mut file, data)| {
                file.write_all(data.as_bytes())
                    .expect("Failed to write file!")
            });

        Ok(())
    }
}
