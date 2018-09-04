use std::error::Error;
use std::fmt;
use std::time::SystemTime;
use std::{fs::File, path::PathBuf, io};

use semver::Version;
use toml;
use utils::FileToString;

/// A set of errors around `lockchain-files` configs
#[derive(Debug)]
pub enum ConfigError {
    IncompatibleVersion(String, String),
    ConfigCorrupted,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ConfigError::*;
        write!(
            f,
            "{}",
            match self {
                IncompatibleVersion(f, l) => {
                    format!("Version '{}' is incompatible with library '{}'", f, l)
                }
                ConfigCorrupted => "Configuration file was corrupted!".into(),
            }
        )
    }
}

impl Error for ConfigError {}

/// The configuration describing a file vault
#[derive(Serialize, Deserialize)]
pub struct VaultConfig {
    /// A semver conforming version string
    pub version: String,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

impl VaultConfig {
    pub fn new() -> Self {
        Self {
            version: "0.1".into(),
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        }
    }

    pub fn save(&self) -> Result<(), io::Error> {

        Ok(())
    }

    /// Attempts to load a configuration – returning detailed errors
    pub fn load(vault: &PathBuf) -> Result<Self, ConfigError> {
        let mut cfg_path = vault.clone();
        cfg_path.push("config.toml");

        let cfg: VaultConfig = match File::open(cfg_path.as_path()) {
            Ok(mut f) => match f.get_string() {
                Ok(s) => match toml::from_str(&s) {
                    Ok(c) => c,
                    Err(_) => return Err(ConfigError::ConfigCorrupted),
                },
                Err(_) => return Err(ConfigError::ConfigCorrupted),
            },
            Err(_) => return Err(ConfigError::ConfigCorrupted),
        };

        let version = match Version::parse(&cfg.version) {
            Ok(v) => v,
            Err(_) => return Err(ConfigError::ConfigCorrupted),
        };

        if version != Version::parse("0.1").unwrap() {
            Err(ConfigError::IncompatibleVersion(cfg.version, "0.1".into()))
        } else {
            Ok(cfg)
        }
    }
}
