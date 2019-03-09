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
