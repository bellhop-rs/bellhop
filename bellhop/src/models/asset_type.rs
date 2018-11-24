use crate::errors::*;
use crate::schema::asset_types;

use super::tag_type::TagType;

use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq, Eq)]
pub struct AssetType {
    id: i32,
    name: String,
}

impl AssetType {
    pub fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<AssetType>> {
        use self::asset_types::dsl::*;

        let mut asset_type = asset_types
            .filter(id.eq(by_id))
            .limit(1)
            .load::<AssetType>(c)
            .chain_err(|| "failed to find asset_type by id")?;

        Ok(asset_type.pop())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fetch_tag_types(&self, c: &PgConnection) -> Result<Vec<TagType>> {
        use crate::schema::tag_types::dsl::*;

        tag_types
            .filter(asset_type_id.eq(self.id))
            .load(c)
            .chain_err(|| "failed to fetch tag types")
    }
}
