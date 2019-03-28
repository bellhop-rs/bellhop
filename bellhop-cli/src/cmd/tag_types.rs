//! Command for interacting with Bellhop's tag types.
use bellhop_client::apis::client::APIClient;
use bellhop_client::models::CreateTagType;

use crate::config::Config;

use structopt::StructOpt;

use super::{Api, CmdError};

#[derive(Debug, StructOpt)]
pub struct TagTypes {
    #[structopt(flatten)]
    api: Api,

    #[structopt(
        name = "asset-type",
        help = "The unique identifier for the tag type's asset type",
        long = "asset-type"
    )]
    asset_type: i32,

    #[structopt(subcommand)]
    cmd: Cmd,
}

impl TagTypes {
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

        let tag_types = api.list_tag_types(self.asset_type)?;

        let types = serde_json::to_string_pretty(&tag_types).unwrap();
        println!("{}", types);

        Ok(())
    }

    fn delete(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let tag_type = api.delete_tag_type(self.asset_type, show.id)?;

        let at = serde_json::to_string_pretty(&tag_type).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn show(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let tag_type = api.show_tag_type(self.asset_type, show.id)?;

        let at = serde_json::to_string_pretty(&tag_type).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn create(&self, client: APIClient, create: &Create) -> Result<(), CmdError> {
        let api = client.default_api();

        let create_tag_type =
            CreateTagType::new(create.name.clone(), create.detail_only, create.rightness);

        let tag_type = api.create_tag_type(self.asset_type, create_tag_type)?;

        let at = serde_json::to_string_pretty(&tag_type).unwrap();
        println!("{}", at);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "list", about = "Prints a list of tag types")]
    List,

    #[structopt(name = "show", about = "Print a single tag type")]
    Show(ById),

    #[structopt(name = "delete", about = "Deletes a tag type")]
    Delete(ById),

    #[structopt(name = "create", about = "Create a new tag type")]
    Create(Create),
}

#[derive(Debug, StructOpt)]
struct ById {
    #[structopt(help = "The unique identifier for the tag type")]
    id: i32,
}

#[derive(Debug, StructOpt)]
struct Create {
    #[structopt(long = "name", help = "Name of the tag type to be created")]
    name: String,

    #[structopt(long = "detail-only", help = "Only show tag on the detail view")]
    detail_only: bool,

    #[structopt(
        long = "rightness",
        help = "How far to the right the tag should be sorted"
    )]
    rightness: i32,
}
