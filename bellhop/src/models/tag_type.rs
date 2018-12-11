use crate::schema::tag_types;

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
