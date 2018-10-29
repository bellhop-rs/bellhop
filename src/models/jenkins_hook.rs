use crate::hooks::HookPoint;
use crate::schema::jenkins_hooks;

use super::asset_type::AssetType;

#[derive(Debug, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[belongs_to(AssetType)]
pub struct JenkinsHook {
    id: i32,
    asset_type_id: i32,

    hook_at: i16,
    username: String,
    token: String,
    url: String,
}

impl JenkinsHook {
    pub fn asset_type_id(&self) -> i32 {
        self.asset_type_id
    }

    pub fn hook_at(&self) -> HookPoint {
        HookPoint::from(self.hook_at)
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
