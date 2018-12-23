use crate::db::Db as PubDb;
use crate::errors::*;
use crate::schema::tags;

use diesel::prelude::*;

use super::asset::Asset;
use super::tag_type::TagType;

#[derive(Debug, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[primary_key(asset_id, tag_type_id)]
#[belongs_to(Asset)]
#[belongs_to(TagType)]
pub struct Tag {
    asset_id: i32,
    tag_type_id: i32,

    value: String,
}

impl Tag {
    pub fn asset_id(&self) -> i32 {
        self.asset_id
    }

    pub fn tag_type_id(&self) -> i32 {
        self.tag_type_id
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// The insertable companion of `Tag`.
///
/// ## Example
///
/// ```no_compile
/// // TODO: When tag is made pub, change to no_run
///
/// use bellhop::db::Db;
/// use bellhop::models::tag::CreateTag;
///
/// // The `db` argument could come from implementing `bellhop::hooks::Hook`, or
/// // as a Rocket request guard.
/// fn some_function(db: &Db) {
///     let new_tag = CreateTag::builder()
///         .value("Color")
///         .asset_id(3)
///         .tag_type_id(30)
///         .build()
///         .insert(db)
///         .unwrap();
/// }
/// ```
#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "tags"]
pub struct CreateTag {
    tag_type_id: i32,
    asset_id: i32,
    value: String,
}

impl CreateTag {
    /// The value of the `Tag` to be created.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Insert the `Tag` into the database and return it.
    ///
    /// See the struct documentation for an example.
    pub fn insert(&self, c: &PubDb) -> Result<Tag> {
        use self::tags::dsl::*;

        diesel::insert_into(tags)
            .values(self)
            .get_result(c.db())
            .chain_err(|| "unable to insert tag")
    }
}

/// Similar to `CreateTag`, but doesn't include `asset_id`.
#[derive(Debug, Deserialize, TypedBuilder, FromForm)]
pub struct CreateOwnedTag {
    tag_type_id: i32,
    value: String,
}

impl CreateOwnedTag {
    pub fn into_create_tag(self, asset_id: i32) -> CreateTag {
        CreateTag {
            asset_id,
            tag_type_id: self.tag_type_id,
            value: self.value,
        }
    }
}
