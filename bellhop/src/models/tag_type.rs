use crate::db::Db as PubDb;
use crate::errors::*;
use crate::schema::tag_types;

use diesel::prelude::*;

use super::asset_type::AssetType;

#[derive(Debug, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[belongs_to(AssetType)]
pub struct TagType {
    id: i32,
    asset_type_id: i32,

    name: String,
    detail_only: bool,
    rightness: i32,
}

impl TagType {
    pub(crate) fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<TagType>> {
        use self::tag_types::dsl::*;

        tag_types
            .filter(id.eq(by_id))
            .limit(1)
            .get_result::<TagType>(c)
            .optional()
            .chain_err(|| "failed to find asset_type by id")
    }

    pub fn detail_only(&self) -> bool {
        self.detail_only
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn asset_type_id(&self) -> i32 {
        self.asset_type_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// The larger this value, the more to the right this tag appears in the
    /// type detail view.
    pub fn rightness(&self) -> i32 {
        self.rightness
    }
}

/// The insertable companion of `TagType`.
///
/// ## Example
///
/// ```no_compile
/// // TODO: When tag_type is made pub, change to no_run
///
/// use bellhop::db::Db;
/// use bellhop::models::tag_type::CreateTagType;
///
/// // The `db` argument could come from implementing `bellhop::hooks::Hook`, or
/// // as a Rocket request guard.
/// fn some_function(db: &Db) {
///     let new_tag_type = CreateTagType::builder()
///         .name("Color")
///         .asset_type_id(3)
///         .build()
///         .insert(db)
///         .unwrap();
/// }
/// ```
#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "tag_types"]
pub struct CreateTagType {
    asset_type_id: i32,
    name: String,

    #[serde(default)]
    #[default]
    detail_only: bool,

    #[serde(default)]
    #[default]
    rightness: i32,
}

impl CreateTagType {
    /// The name of the `TagType` to be created.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Insert the `TagType` into the database and return it.
    ///
    /// See the struct documentation for an example.
    pub fn insert(&self, c: &PubDb) -> Result<TagType> {
        use self::tag_types::dsl::*;

        diesel::insert_into(tag_types)
            .values(self)
            .get_result(c.db())
            .chain_err(|| "unable to insert tag type")
    }
}

/// Similar to `CreateTagType`, but doesn't include `asset_type_id`.
#[derive(Debug, Deserialize, TypedBuilder, FromForm)]
pub struct CreateOwnedTagType {
    name: String,

    #[serde(default)]
    #[default]
    detail_only: bool,

    #[serde(default)]
    #[default]
    rightness: i32,
}

impl CreateOwnedTagType {
    pub fn into_create_tag_type(self, asset_type_id: i32) -> CreateTagType {
        CreateTagType {
            asset_type_id,
            name: self.name,
            detail_only: self.detail_only,
            rightness: self.rightness,
        }
    }
}
