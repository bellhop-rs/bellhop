//! Every `Asset` belongs to a family of assets represented by `AssetType`.

use crate::db::Db as PubDb;
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

/// The insertable companion of `AssetType`.
///
/// ## Example
///
/// ```no_run
/// use bellhop::db::Db;
/// use bellhop::models::asset_type::CreateAssetType;
///
/// // The `db` argument could come from implementing `bellhop::hooks::Hook`, or
/// // as a Rocket request guard.
/// fn some_function(db: &Db) {
///     let new_user = CreateAssetType::builder()
///         .name("Charlie Region")
///         .build()
///         .insert(db)
///         .unwrap();
/// }
/// ```
#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "asset_types"]
pub struct CreateAssetType {
    name: String,
}

impl CreateAssetType {
    /// The name of the `AssetType` to be created.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Insert the `AssetType` into the database and return it.
    ///
    /// See the struct documentation for an example.
    pub fn insert(&self, c: &PubDb) -> Result<AssetType> {
        use self::asset_types::dsl::*;

        diesel::insert_into(asset_types)
            .values(self)
            .get_result(c.db())
            .chain_err(|| "unable to insert asset type")
    }
}
