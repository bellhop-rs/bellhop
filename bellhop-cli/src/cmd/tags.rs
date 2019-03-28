//! Command for interacting with Bellhop's tags.
use bellhop_client::apis::client::APIClient;
use bellhop_client::models::CreateTag;

use crate::config::Config;

use structopt::StructOpt;

use super::{Api, CmdError};

#[derive(Debug, StructOpt)]
pub struct Tags {
    #[structopt(flatten)]
    api: Api,

    #[structopt(
        name = "asset",
        help = "The unique identifier for the tag's asset",
        long = "asset"
    )]
    asset: i32,

    #[structopt(subcommand)]
    cmd: Cmd,
}

impl Tags {
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

        let tags = api.list_tags(self.asset)?;

        let types = serde_json::to_string_pretty(&tags).unwrap();
        println!("{}", types);

        Ok(())
    }

    fn delete(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let tag = api.delete_tag(self.asset, show.tag_type)?;

        let at = serde_json::to_string_pretty(&tag).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn show(&self, client: APIClient, show: &ById) -> Result<(), CmdError> {
        let api = client.default_api();

        let tag = api.show_tag(self.asset, show.tag_type)?;

        let at = serde_json::to_string_pretty(&tag).unwrap();
        println!("{}", at);

        Ok(())
    }

    fn create(&self, client: APIClient, create: &Create) -> Result<(), CmdError> {
        let api = client.default_api();

        let create_tag = CreateTag::new(create.value.clone(), create.tag_type);

        let tag = api.create_tag(self.asset, create_tag)?;

        let at = serde_json::to_string_pretty(&tag).unwrap();
        println!("{}", at);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "list", about = "Prints a list of tags")]
    List,

    #[structopt(name = "show", about = "Print a single tag")]
    Show(ById),

    #[structopt(name = "delete", about = "Deletes a tag")]
    Delete(ById),

    #[structopt(name = "create", about = "Create a new tag")]
    Create(Create),
}

#[derive(Debug, StructOpt)]
struct ById {
    #[structopt(
        name = "tag-type",
        long = "tag-type",
        help = "The unique identifier for the tag's type"
    )]
    tag_type: i32,
}

#[derive(Debug, StructOpt)]
struct Create {
    #[structopt(long = "value", help = "Value of the tag to be created")]
    value: String,

    #[structopt(
        long = "tag-type",
        help = "The unique identifier for the tag type this tag belongs to"
    )]
    tag_type: i32,
}
