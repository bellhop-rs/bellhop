//! An `Asset` is a resource that can be loaned and returned.
//!
//! Examples of assets could include lab computers, smartphones, or test
//! credentials.

use crate::errors::*;
use crate::schema::assets;

use super::asset_type::AssetType;
use super::lease::Lease;
use super::user::User;

use diesel::prelude::*;

/// An `Asset` is a resource that can be loaned and returned.
///
/// `Asset`s are the _raison d'être_ for Bellhop. This struct represents the
/// things people want to share and borrow.
#[derive(Debug, Clone, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[belongs_to(AssetType, foreign_key = "type_id")]
#[belongs_to(Lease)]
pub struct Asset {
    id: i32,
    type_id: i32,
    lease_id: Option<i32>,

    name: String,
}

impl Asset {
    pub(crate) fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<Asset>> {
        use self::assets::dsl::*;

        let mut asset = assets
            .filter(id.eq(by_id))
            .limit(1)
            .load::<Asset>(c)
            .chain_err(|| "failed to find asset by id")?;

        Ok(asset.pop())
    }

    pub(crate) fn fetch_lease_owner(&self, c: &PgConnection) -> Result<Option<(Lease, User)>> {
        use crate::schema::leases::dsl::*;
        use crate::schema::users::dsl as u;

        if let Some(by_lease_id) = self.lease_id {
            leases
                .inner_join(u::users)
                .filter(id.eq(by_lease_id))
                .get_result(c)
                .optional()
                .chain_err(|| "failed to fetch tag types")
        } else {
            Ok(None)
        }
    }

    /// The primary key of this `Asset`.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// The primary key of this `Asset`'s `AssetType`.
    pub fn type_id(&self) -> i32 {
        self.type_id
    }

    /// The primary key of this `Asset`'s `Lease`, or `None` if it hasn't been
    /// leased yet.
    pub fn lease_id(&self) -> Option<i32> {
        self.lease_id
    }

    /// The human-readable name of this `Asset`.
    pub fn name(&self) -> &str {
        &self.name
    }
}
