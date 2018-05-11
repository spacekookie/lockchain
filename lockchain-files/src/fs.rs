//! Utility module which handles filesystem writes

use lcc::traits::AutoEncoder;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

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
    Checksum,
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
    pub fn fetch<T: AutoEncoder>(&self, types: FileType) -> Result<Vec<T>, io::Error> {
        Ok(fs::read_dir(match types {
            FileType::Record => self.root.join("records"),
            _ => self.root.join("."),
        })?.into_iter()
            .filter_map(|r| r.ok())
            .filter(|f| match f.file_type() {
                Ok(vf) => vf.is_file(),
                _ => false,
            })
            .map(|de| de.path())
            .filter_map(|p| p.into_os_string().into_string().ok())
            .filter_map(|s| File::open(s).ok())
            .filter_map(|mut f| f.get_string().ok())
            .filter_map(|s| T::decode(&s).ok())
            .collect())
    }

    pub fn pull<T: AutoEncoder>(&self, types: FileType, id: &str) -> Result<T, io::Error> {
        unimplemented!()
    }

    pub fn sync<T: AutoEncoder>(&self, types: FileType) -> Result<(), io::Error> {
        unimplemented!()
    }
}

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