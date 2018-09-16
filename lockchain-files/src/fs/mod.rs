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

use lcc::traits::{AutoEncoder, Body};

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use utils::FileToString;
use FileVault;

mod primitive;
use self::primitive::*;
use userstore::DiskMirror;

#[derive(Debug)]
pub struct Filesystem {
    pub name: String,
    pub path: String,
    pub root: PathBuf,
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
    pub fn scaffold(&self) -> Result<(), io::Error> {
        fs::create_dir_all(&self.root)?;
        fs::create_dir(&self.root.join("records"))?;
        fs::create_dir(&self.root.join("metadata"))?;
        fs::create_dir(&self.root.join("checksums"))?;
        Ok(())
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
        // Ok(T::decode(
        //     &File::open(self.root.join(&format!("{}.{}", id, file_ending!(types))))?
        //         .get_string()?,
        // )?)
        unimplemented!()
    }

    pub fn sync_vault<T: Body>(&self, vault: &FileVault<T>) -> Result<(), io::Error> {
        vault.config.save(&self.root)?;
        primitive::write_file(
            FileType::Metadata,
            &self.root,
            "userstore",
            vault.users.to_disk(),
        )?;

        Ok(())
    }

    /// Respond to a sync request
    #[deprecated]
    pub fn sync<T>(&self, data: &HashMap<String, T>, types: FileType) -> Result<(), Box<Error>>
    where
        T: AutoEncoder,
    {
        unimplemented!()
        // data.into_iter()
        //     .map(|(k, v)| (k, v.encode().ok()))
        //     .map(|(k, v)| {
        //         (
        //             match types {
        //                 FileType::Record => self.root.join("records"),
        //                 FileType::Metadata => self.root.join("metadata"),
        //                 _ => self.root.join("."),
        //             }.join(format!("{}.{}", k, file_ending!(types))),
        //             v,
        //         )
        //     }).filter(|(_, v)| v.is_some())
        //     .map(|(k, v)| (k, v.unwrap()))
        //     .map(|(path, data): (PathBuf, String)| {
        //         (OO::new().create(true).write(true).open(path), data)
        //     }).filter(|(path, _)| path.is_ok())
        //     .map(|(file, data)| (file.unwrap(), data))
        //     .for_each(|(mut file, data)| {
        //         file.write_all(data.as_bytes())
        //             .expect("Failed to write file!")
        //     });

        // Ok(())
    }

    /************* Private utility functions*************/
}
