use bellhop_client::apis::client::APIClient;
use bellhop_client::apis::configuration::Configuration;

use crate::config::{Config, Identity, Remote};

use reqwest::ClientBuilder;

pub use self::error::*;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[allow(deprecated)] // error-chain#254
mod error {
    use super::*;

    error_chain! {
        types {
            ClientError, ClientErrorKind, ResultExt;
        }
        links {
            Config(crate::config::ConfigError, crate::config::ConfigErrorKind);
        }
        foreign_links {
            Reqwest(reqwest::Error);
        }
        errors {
            Io(path: PathBuf) {
                description("input/output error")
                display("input/output failed on `{}`", path.to_string_lossy())
            }
        }
    }
}

fn read(buf: &mut Vec<u8>, path: &Path) -> Result<(), ClientError> {
    File::open(Config::resolve(path)?)
        .chain_err(|| ClientErrorKind::Io(path.to_path_buf()))?
        .read_to_end(buf)
        .chain_err(|| ClientErrorKind::Io(path.to_path_buf()))?;
    Ok(())
}

fn identity(identity: &Identity) -> Result<reqwest::Identity, ClientError> {
    let crt = match identity {
        Identity::ClientCertificate(crt) => crt,
    };

    let mut pem = vec![];
    read(&mut pem, &crt.certificate)?;
    if let Some(ref key) = crt.key {
        read(&mut pem, key)?;
    }

    let ident = reqwest::Identity::from_pem(&pem)?;

    Ok(ident)
}

fn certificate(path: &Path) -> Result<reqwest::Certificate, ClientError> {
    let mut pem = vec![];
    read(&mut pem, path)?;

    let ident = reqwest::Certificate::from_pem(&pem)?;

    Ok(ident)
}

pub fn build(remote: &Remote, insecure: bool) -> Result<APIClient, ClientError> {
    let mut builder = ClientBuilder::new();

    if let Some(ref ident) = remote.identity {
        let reqwest_identity = identity(ident)?;
        builder = builder.identity(reqwest_identity);
    }

    if insecure {
        builder = builder.danger_accept_invalid_certs(true);
    }

    for ca in remote.additional_root_certificates.iter() {
        builder = builder.add_root_certificate(certificate(ca)?);
    }

    let client = builder.build()?;

    let mut configuration = Configuration::new();
    configuration.base_path = remote.base_path.clone();
    configuration.client = client;

    Ok(APIClient::new(configuration))
}
