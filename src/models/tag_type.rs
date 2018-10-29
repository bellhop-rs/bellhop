use crate::errors::*;
use crate::schema::tag_types;

use super::asset_type::AssetType;

use diesel::prelude::*;

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
    pub fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<TagType>> {
        use self::tag_types::dsl::*;

        let tag_type: Option<TagType> = tag_types
            .filter(id.eq(by_id))
            .get_result(c)
            .optional()
            .chain_err(|| "failed to find tag_type by id")?;

        Ok(tag_type)
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
