//! Command for interacting with Bellhop's asset types.
use bellhop_client::apis::client::APIClient;
use bellhop_client::models::CreateAssetType;

use crate::config::Config;

use structopt::StructOpt;

use super::{Api, CmdError};

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
            Cmd::Show(ref show) => self.show(client, show),
            Cmd::Delete(ref show) => self.delete(client, show),
            Cmd::Create(ref create) => self.create(client, create),
        }
    }

    fn list(&self, client: APIClient) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset_types = api.list_asset_types()?;

        let types = serde_json::to_string_pretty(&asset_types).unwrap();
        println!("{}", types);

        Ok(())
    }

    fn delete(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset_type = api.delete_asset_type(show.id)?;

        let at = serde_json::to_string_pretty(&asset_type).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn show(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let asset_type = api.show_asset_type(show.id)?;

        let at = serde_json::to_string_pretty(&asset_type).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn create(&self, client: APIClient, show: &Create) -> Result<(), CmdError> {
        let api = client.default_api();

        let create_asset_type = CreateAssetType::new(show.name.clone(), show.plural_name.clone());

        let asset_type = api.create_asset_type(create_asset_type)?;

        let at = serde_json::to_string_pretty(&asset_type).unwrap();
        println!("{}", at);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "list", about = "Prints a list of asset types")]
    List,

    #[structopt(name = "show", about = "Print a single asset type")]
    Show(ById),

    #[structopt(
        name = "delete",
        about = "Deletes an asset type and all of its tags and assets"
    )]
    Delete(ById),

    #[structopt(name = "create", about = "Create a new asset type")]
    Create(Create),
}

#[derive(Debug, StructOpt)]
struct ById {
    #[structopt(help = "The unique identifier for the asset type")]
    id: i32,
}

#[derive(Debug, StructOpt)]
struct Create {
    #[structopt(long = "name", help = "Name of the asset type to be created")]
    name: String,

    #[structopt(
        long = "plural-name",
        help = "Spelling of the name, when referring to many"
    )]
    plural_name: String,
}
