//! Utilities for building an `APIClient` with the given authentication and
//! base path.

use bellhop_client::apis::client::APIClient;
use bellhop_client::apis::configuration::{ApiKey, Configuration};

use crate::config::{ClientCertificate, Config, Identity, Remote};

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

fn client_certificate(crt: &ClientCertificate) -> Result<reqwest::Identity, ClientError> {
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

    match remote.identity {
        Some(Identity::ClientCertificate(ref crt)) => {
            let reqwest_identity = client_certificate(crt)?;
            builder = builder.identity(reqwest_identity);
        }
        _ => (),
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

    match remote.identity {
        Some(Identity::Header(ref header)) => {
            let api_key = ApiKey {
                prefix: None,
                key: header.value.clone(),
            };

            configuration.api_key = Some(api_key);
        }
        _ => (),
    }

    Ok(APIClient::new(configuration))
}
