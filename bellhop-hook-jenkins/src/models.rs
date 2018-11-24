use crate::schema::jenkins_hooks;

use bellhop::models::asset_type::AssetType;

use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(i16)]
pub enum HookPoint {
    Leased = 0,
    Returned = 1,
    Evicted = 2,
}

impl fmt::Display for HookPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookPoint::Leased => write!(f, "leased"),
            HookPoint::Returned => write!(f, "returned"),
            HookPoint::Evicted => write!(f, "evicted"),
        }
    }
}

impl From<i16> for HookPoint {
    fn from(o: i16) -> HookPoint {
        match o {
            0 => HookPoint::Leased,
            1 => HookPoint::Returned,
            2 => HookPoint::Evicted,
            _ => panic!("unknown hook point"),
        }
    }
}

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
