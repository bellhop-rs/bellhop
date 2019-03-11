//! `bellhop-cli` is a command line tool for working with Bellhop's HTTP API.
//!
//! ## Usage Examples
//!
//! ### Creating a Default Config
//!
//! This command creates a configuration file that needs to be edited to point
//! at your Bellhop instance.
//!
//! ```bash
//! $ bellhop-cli config create
//! ```
//!
//! ### Listing Asset Types
//!
//! ```bash
//! $ bellhop-cli asset-types list
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;

mod client;
mod cmd;
mod config;

use structopt::StructOpt;

quick_main!(run);

fn run() -> Result<(), cmd::CmdError> {
    let opt = cmd::Opt::from_args();
    opt.execute()?;

    Ok(())
}
