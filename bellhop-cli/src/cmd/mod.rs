//! Root of the command line interface command "tree".
//!
//! Each command is implemented as a `structopt` sub-parser.
mod asset_types;
mod assets;
mod config;

use crate::config::Config as ConfigFile;

pub use self::error::*;

use std::path::PathBuf;

use structopt::StructOpt;

#[allow(deprecated)] // error-chain#254
mod error {
    use bellhop_client::apis::Error as BellhopError;

    error_chain! {
        types {
            CmdError, CmdErrorKind, ResultExt;
        }

        errors {
            Api {
                description("the api client encountered a problem")
            }
        }

        links {
            Config(crate::config::ConfigError, crate::config::ConfigErrorKind);
            Client(crate::client::ClientError, crate::client::ClientErrorKind);
        }
    }

    impl From<BellhopError> for CmdError {
        fn from(error: BellhopError) -> CmdError {
            match error {
                BellhopError::Io(e) => CmdError::with_chain(e, CmdErrorKind::Api),
                BellhopError::Reqwest(e) => CmdError::with_chain(e, CmdErrorKind::Api),
                BellhopError::Serde(e) => CmdError::with_chain(e, CmdErrorKind::Api),
            }
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Cmd,

    #[structopt(short = "c", help = "Specify an alternate configuration file")]
    config: Option<PathBuf>,
}

impl Opt {
    pub fn execute(&self) -> Result<(), CmdError> {
        if let Cmd::Config(ref cfg) = self.cmd {
            return cfg.execute(self.config.clone());
        }

        let cfg = match self.config {
            Some(ref c) => ConfigFile::load_from(c)?,
            None => ConfigFile::load()?,
        };

        match self.cmd {
            Cmd::Config(_) => unreachable!(),
            Cmd::AssetTypes(ref at) => at.execute(&cfg),
            Cmd::Assets(ref a) => a.execute(&cfg),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "command")]
pub enum Cmd {
    #[structopt(name = "config", about = "Create or view bellhop-cli's configuration")]
    Config(config::Config),
    #[structopt(name = "asset-types", about = "View or modify asset types")]
    AssetTypes(asset_types::AssetTypes),
    #[structopt(name = "assets", about = "View or modify assets")]
    Assets(assets::Assets),
}

#[derive(Debug, StructOpt)]
struct Api {
    #[structopt(
        short = "k",
        long = "insecure",
        help = "Allow connections to SSL sites without valid certificates"
    )]
    insecure: bool,
}
