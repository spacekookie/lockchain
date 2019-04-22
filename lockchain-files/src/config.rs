use std::{
    error::Error,
    fmt,
    fs::{File, OpenOptions as OO},
    io::{self, Write},
    path::PathBuf,
    time::SystemTime,
};

use crate::utils::FileToString;
use semver::Version;
use serde_yaml;

use crate::lcc::{errors::VaultError, VaultType};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    /// A semver conforming version string
    pub version: String,
    pub vault_type: ConfigType,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigType {
    SoloUser,
    Administrated,
    Unmanaged,
}

impl VaultConfig {
    pub fn new(vt: &VaultType) -> Result<Self, VaultError> {
        Ok(Self {
            version: "0.1".into(),
            vault_type: match vt {
                &VaultType::SoloUser { .. } => ConfigType::SoloUser,
                &VaultType::Administrated { .. } => ConfigType::Administrated,
                _ => ConfigType::Unmanaged,
            },
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        })
    }

    pub fn save(&self, vault: &PathBuf) -> Result<(), io::Error> {
        let mut cfg_path = vault.clone();
        cfg_path.push("vault.cfg");

        let t = serde_yaml::to_string(self).unwrap();
        let mut f = OO::new().create(true).write(true).open(cfg_path)?;
        f.write_all(t.as_bytes())?;
        Ok(())
    }

    /// Attempts to load a configuration – returning detailed errors
    pub fn load(vault: &PathBuf) -> Result<Self, ConfigError> {
        let mut cfg_path = vault.clone();
        cfg_path.push("vault.cfg");

        let cfg: VaultConfig = match File::open(cfg_path.as_path()) {
            Ok(mut f) => match f.get_string() {
                Ok(s) => match serde_yaml::from_str(&s) {
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
