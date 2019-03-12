//! Command for interacting with Bellhop's assets.
use bellhop_client::apis::client::APIClient;
use bellhop_client::models::CreateAsset;

use crate::config::Config;

use structopt::StructOpt;

use super::{Api, CmdError};

#[derive(Debug, StructOpt)]
pub struct Assets {
    #[structopt(flatten)]
    api: Api,

    #[structopt(subcommand)]
    cmd: Cmd,
}

impl Assets {
    pub fn execute(&self, cfg: &Config) -> Result<(), CmdError> {
        let client = crate::client::build(&cfg.remote, self.api.insecure)?;

        match self.cmd {
            Cmd::List => self.list(client),
            Cmd::Show(ref show) => self.show(client, show),
            Cmd::Delete(ref show) => self.delete(client, show),
            Cmd::Create(ref create) => self.create(client, create),
        }
    }

    fn list(&self, client: APIClient) -> Result<(), CmdError> {
        let api = client.default_api();

        let assets = api.list_assets()?;

        let types = serde_json::to_string_pretty(&assets).unwrap();
        println!("{}", types);

        Ok(())
    }

    fn delete(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset = api.delete_asset(show.id)?;

        let at = serde_json::to_string_pretty(&asset).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn show(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset = api.show_asset(show.id)?;

        let at = serde_json::to_string_pretty(&asset).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn create(&self, client: APIClient, create: &Create) -> Result<(), CmdError> {
        let api = client.default_api();

        let create_asset = CreateAsset::new(create.asset_type, create.name.clone());

        let asset = api.create_asset(create_asset)?;

        let at = serde_json::to_string_pretty(&asset).unwrap();
        println!("{}", at);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "list", about = "Prints a list of assets")]
    List,

    #[structopt(name = "show", about = "Print a single asset")]
    Show(ById),

    #[structopt(name = "delete", about = "Deletes an asset and all of its tags")]
    Delete(ById),

    #[structopt(name = "create", about = "Create a new asset")]
    Create(Create),
}

#[derive(Debug, StructOpt)]
struct ById {
    #[structopt(help = "The unique identifier for the asset")]
    id: i32,
}

#[derive(Debug, StructOpt)]
struct Create {
    #[structopt(long = "name", help = "Name of the asset to be created")]
    name: String,

    #[structopt(
        long = "asset-type",
        help = "The unique identifier for the asset type this asset should belong to"
    )]
    asset_type: i32,
}
