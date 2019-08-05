use crate::errors::*;
use crate::models::asset_type::AssetType;
use crate::schema::asset_types;

use rocket_contrib::databases::diesel;

use diesel::prelude::*;

use std::fmt;

#[database("bellhop")]
pub struct Db(diesel::PgConnection);

impl DbPool {
    pub fn get(&self) -> Option<Db> {
        self.0.get().ok().map(Db)
    }
}

impl Clone for DbPool {
    fn clone(&self) -> Self {
        DbPool(self.0.clone())
    }
}

impl fmt::Debug for Db {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Db(PgConnection)")
    }
}

pub fn get_all_types(c: &PgConnection) -> Result<Vec<AssetType>> {
    use self::asset_types::dsl::*;

    let all_asset_types: Vec<AssetType> = asset_types
        .load::<AssetType>(c)
        .chain_err(|| "failed to find user by email")?;

    Ok(all_asset_types)
}
