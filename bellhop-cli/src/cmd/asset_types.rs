use bellhop_client::apis::client::APIClient;

use crate::config::Config;

use super::{Api, CmdError};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AssetTypes {
    #[structopt(flatten)]
    api: Api,

    #[structopt(subcommand)]
    cmd: Cmd,
}

impl AssetTypes {
    pub fn execute(&self, cfg: &Config) -> Result<(), CmdError> {
        let client = crate::client::build(&cfg.remote, self.api.insecure)?;

        match self.cmd {
            Cmd::List => self.list(client),
        }
    }

    fn list(&self, client: APIClient) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset_types = api.list_asset_types()?;

        let types = serde_json::to_string_pretty(&asset_types).unwrap();
        println!("{}", types);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "list", about = "Prints a list of asset types")]
    List,
}
