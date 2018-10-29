use crate::schema::tags;

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
