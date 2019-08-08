//! A `Lease` is a duration of time that a `User` owns an `Asset`.
use crate::db::Db as PubDb;
use crate::errors::*;
use crate::schema::leases;

use super::user::User;

use chrono::prelude::*;

use diesel::prelude::*;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;
use std::str::FromStr;

/// A `Lease` is a duration of time that a `User` owns an `Asset`.
#[derive(Debug, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[belongs_to(User)]
pub struct Lease {
    id: i32,
    user_id: i32,

    last_notified: Option<DateTime<Utc>>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl Lease {
    pub(crate) fn by_id(c: &PgConnection, by_id: i32) -> Result<Option<Lease>> {
        use self::leases::dsl::*;

        let mut lease = leases
            .filter(id.eq(by_id))
            .limit(1)
            .load::<Lease>(c)
            .chain_err(|| "failed to find lease by id")?;

        Ok(lease.pop())
    }

    /// The primary key of this `Lease`.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// The primary key of this `Lease`'s owner.
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    /// When the last sheriff notification was sent for this `Lease`.
    pub fn last_notified(&self) -> Option<DateTime<Utc>> {
        self.last_notified
    }

    /// When this `Lease` was created.
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    /// When this `Lease` is expected to end.
    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time
    }
}

/// Insertable companion to [`Lease`].
///
/// ## Example
///
/// ```no_run
/// use bellhop::db::Db;
/// use bellhop::models::lease::CreateLease;
///
/// use chrono::prelude::*;
///
/// fn some_function(db: &Db) {
///     let lease = CreateLease::builder()
///         .user_id(1)
///         .start_time(Utc::now())
///         .end_time(Utc::now())
///         .build()
///         .insert(db)
///         .unwrap();
/// }
/// ```
#[derive(Debug, Deserialize, Insertable, TypedBuilder)]
#[table_name = "leases"]
pub struct CreateLease {
    user_id: i32,

    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl CreateLease {
    /// The primary key for the new `Lease`'s owner.
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    /// When the new `Lease` will come into effect.
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    /// When the new `Lease` will end.
    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time
    }

    /// Insert the `Lease` into the database and return it.
    pub fn insert(&self, c: &PubDb) -> Result<Lease> {
        use self::leases::dsl::*;

        diesel::insert_into(leases)
            .values(self)
            .get_result(c.db())
            .chain_err(|| "unable to insert lease")
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct DateField(pub DateTime<Utc>);

impl Deref for DateField {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl DerefMut for DateField {
    fn deref_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
    }
}

impl<'v> FromFormValue<'v> for DateField {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> StdResult<Self, &'v RawStr> {
        let txt = String::from_form_value(form_value)?;
        let inner = DateTime::from_str(&txt).map_err(|_| form_value)?;

        Ok(DateField(inner))
    }
}

#[derive(Debug, FromForm, Deserialize)]
pub(crate) struct CreateLeaseForm {
    end_time: Option<DateField>,
}

impl CreateLeaseForm {
    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time.map(|x| x.0)
    }

    pub fn into_create_lease(self, user_id: i32) -> CreateLease {
        CreateLease::builder()
            .user_id(user_id)
            .start_time(Utc::now())
            .end_time(self.end_time.map(|x| x.0))
            .build()
    }
}
