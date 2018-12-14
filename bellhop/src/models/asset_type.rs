//! Every `Asset` belongs to a family of assets represented by `AssetType`.

use crate::errors::*;
use crate::schema::asset_types;

use diesel::prelude::*;

/// An `AssetType` is the family an `Asset` belongs to.
#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq, Eq)]
pub struct AssetType {
    id: i32,
    name: String,
}

impl AssetType {
    pub(crate) fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<AssetType>> {
        use self::asset_types::dsl::*;

        let mut asset_type = asset_types
            .filter(id.eq(by_id))
            .limit(1)
            .load::<AssetType>(c)
            .chain_err(|| "failed to find asset_type by id")?;

        Ok(asset_type.pop())
    }

    /// The primary key of this `AssetType`.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// The human-readable name of this `AssetType`.
    pub fn name(&self) -> &str {
        &self.name
    }
}
