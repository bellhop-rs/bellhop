//! Configuration for `bellhop-cli` itself.
//!
//! Includes the actual configuration structure, and functions for loading/storing
//! it.
//!
//! Uses the `directories` crate to put the configuration in the proper place
//! depending on the operating system.

use directories::ProjectDirs;

pub use self::error::*;
use serde_derive::{Deserialize, Serialize};

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use toml;

#[allow(deprecated)] // error-chain#254
mod error {
    use super::*;

    error_chain! {
        types {
            ConfigError, ConfigErrorKind, ResultExt;
        }
        foreign_links {
            Deserialize(toml::de::Error);
            Serialize(toml::ser::Error);
        }
        errors {
            Io(path: PathBuf) {
                description("input/output error")
                display("input/output failed on `{}`", path.to_string_lossy())
            }
            MissingHome {
                description("no home directory could be found")
                display("no home directory could be found")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub remote: Remote,
}

impl Config {
    fn dirs() -> Result<ProjectDirs, ConfigError> {
        let result = ProjectDirs::from("rs", "bellhop", "bellhop-cli")
            .chain_err(|| ConfigErrorKind::MissingHome)?;

        Ok(result)
    }

    fn config_path() -> Result<PathBuf, ConfigError> {
        let dirs = Self::dirs()?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    pub fn resolve<P: AsRef<Path>>(p: P) -> Result<PathBuf, ConfigError> {
        let dirs = Self::dirs()?;
        let resolved = dirs.config_dir();
        Ok(resolved.join(p))
    }

    fn default_bytes() -> &'static [u8] {
        include_bytes!("default_config.toml")
    }

    pub fn load() -> Result<Config, ConfigError> {
        Self::load_from(Self::config_path()?)
    }

    pub fn load_from<P: AsRef<Path>>(p: P) -> Result<Config, ConfigError> {
        let config_path = p.as_ref();
        let mut file =
            File::open(config_path).chain_err(|| ConfigErrorKind::Io(config_path.to_path_buf()))?;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .chain_err(|| ConfigErrorKind::Io(config_path.to_path_buf()))?;

        let config = toml::from_slice(&bytes)?;

        Ok(config)
    }

    pub fn save_default(config_path: Option<PathBuf>) -> Result<PathBuf, ConfigError> {
        let config_path = match config_path {
            None => Self::config_path()?,
            Some(x) => x,
        };

        if let Some(config_dir) = config_path.parent() {
            create_dir_all(config_dir).chain_err(|| ConfigErrorKind::Io(config_dir.to_owned()))?;
        }

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&config_path)
            .chain_err(|| ConfigErrorKind::Io(config_path.clone()))?;

        file.write_all(Self::default_bytes())
            .chain_err(|| ConfigErrorKind::Io(config_path.clone()))?;

        Ok(config_path)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Remote {
    pub base_path: String,

    #[serde(default)]
    pub identity: Option<Identity>,

    #[serde(default)]
    pub additional_root_certificates: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Identity {
    ClientCertificate(ClientCertificate),
    Header(Header),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCertificate {
    pub certificate: PathBuf,

    #[serde(default)]
    pub key: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn parse_default_config() {
        let bytes = Config::default_bytes();
        let config: Config = toml::from_slice(bytes).unwrap();

        assert_eq!(config.remote.base_path, "http://localhost/api/v0");
    }

    #[test]
    fn parse_example_config() {
        let txt = String::from_utf8(Config::default_bytes().to_vec()).unwrap();
        let re = Regex::new(r"(?m:^#)").unwrap();
        let uncommented = re.replace_all(&txt, "").to_owned();

        let config: Config = toml::from_str(&uncommented).unwrap();

        assert_eq!(
            config.remote.additional_root_certificates,
            vec![Path::new("/ca.pem")],
        );

        let identity = config.remote.identity.unwrap();

        let cert = match identity {
            Identity::ClientCertificate(c) => c,
            Identity::Header(_) => panic!("expected client certificate"),
        };

        assert_eq!(cert.certificate, PathBuf::from("/cert.pem"));
        assert_eq!(cert.key, Some("/key.pem".into()));
    }
}
