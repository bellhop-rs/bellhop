//! Command for interacting with `bellhop-cli`'s configuration file. Not to be
//! confused with `crate::config` (which is the configuration file itself.)
use crate::config::Config as ConfigFile;

use std::path::{Path, PathBuf};

use structopt::StructOpt;

use super::CmdError;

use toml;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(subcommand)]
    cmd: Cmd,
}

impl Config {
    pub fn execute(&self, config_path: Option<PathBuf>) -> Result<(), CmdError> {
        match self.cmd {
            Cmd::Show => self.show(config_path.as_ref().map(|x| x.as_ref())),
            Cmd::Create => self.create(config_path),
        }
    }

    pub fn show(&self, config_path: Option<&Path>) -> Result<(), CmdError> {
        let cfg = match config_path {
            None => ConfigFile::load()?,
            Some(c) => ConfigFile::load_from(c)?,
        };

        let value = toml::Value::try_from(&cfg).unwrap();
        let serialized = toml::to_string_pretty(&value).unwrap();

        println!("{}", serialized);
        Ok(())
    }

    pub fn create(&self, config_path: Option<PathBuf>) -> Result<(), CmdError> {
        let path = ConfigFile::save_default(config_path)?;
        println!("created configuration file: `{}`", path.to_string_lossy());
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "show", about = "Print the current configuration")]
    Show,

    #[structopt(name = "create", about = "Write a default configuration file")]
    Create,
}
