use crate::models::asset_type::AssetType;
use crate::schema::asset_types;

use rocket_contrib::databases::diesel;

use diesel::prelude::*;

use crate::errors::*;

#[database("bellhop")]
pub struct Db(diesel::PgConnection);

pub fn get_all_types(c: &PgConnection) -> Result<Vec<AssetType>> {
    use self::asset_types::dsl::*;

    let all_asset_types: Vec<AssetType> = asset_types
        .load::<AssetType>(c)
        .chain_err(|| "failed to find user by email")?;

    Ok(all_asset_types)
}