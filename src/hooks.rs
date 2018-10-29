use crate::errors::*;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::jenkins_hook::JenkinsHook;
use crate::models::lease::Lease;

use diesel::prelude::*;

use reqwest::{Client, Url};

use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(i16)]
pub enum HookPoint {
    Leased = 0,
    Returned = 1,
    Evicted = 2,
}

impl fmt::Display for HookPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookPoint::Leased => write!(f, "leased"),
            HookPoint::Returned => write!(f, "returned"),
            HookPoint::Evicted => write!(f, "evicted"),
        }
    }
}

impl From<i16> for HookPoint {
    fn from(o: i16) -> HookPoint {
        match o {
            0 => HookPoint::Leased,
            1 => HookPoint::Returned,
            2 => HookPoint::Evicted,
            _ => panic!("unknown hook point"),
        }
    }
}

pub fn run_hooks(db: &PgConnection, lease: Lease, asset: Asset, hook: HookPoint) -> Result<()> {
    use crate::schema::asset_types::dsl as at;

    let asset_type = AssetType::by_id(db, asset.type_id())?.chain_err(|| "missing asset_type")?;

    run_jenkins_hooks(db, lease, asset, asset_type, hook)?;

    Ok(())
}

fn run_jenkins_hooks(
    db: &PgConnection,
    lease: Lease,
    asset: Asset,
    asset_type: AssetType,
    by_hook_at: HookPoint,
) -> Result<()> {
    use crate::schema::jenkins_hooks::dsl::*;

    println!("banana");

    let hook: Option<JenkinsHook> = JenkinsHook::belonging_to(&asset_type)
        .filter(hook_at.eq(by_hook_at as i16))
        .get_result(db)
        .optional()
        .chain_err(|| "unable to get jenkins hooks from asset type")?;

    let hook = match hook {
        Some(x) => x,
        None => return Ok(()),
    };

    #[derive(Debug, Serialize)]
    struct Kv {
        name: String,
        value: String,
    }

    #[derive(Debug, Serialize)]
    struct Body {
        parameter: Vec<Kv>,
    }

    let body = Body {
        parameter: vec![Kv {
            name: "hook_at".to_owned(),
            value: by_hook_at.to_string(),
        }],
    };

    let client = Client::new();
    let resp = client
        .post(hook.url())
        .basic_auth(hook.username(), Some(hook.token()))
        .json(&body)
        .send();

    println!("POST: {:?}", &resp);
    println!("POST RESP: {:?}", resp.unwrap().text());

    Ok(())
}
