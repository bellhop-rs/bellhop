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

#[derive(Debug, Associations, Serialize, Queryable, Identifiable, PartialEq, Eq)]
#[belongs_to(User)]
pub struct Lease {
    id: i32,
    user_id: i32,

    last_notified: Option<DateTime<Utc>>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn last_notified(&self) -> Option<DateTime<Utc>> {
        self.last_notified
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }
}

#[derive(Debug, Deserialize, Insertable, TypedBuilder)]
#[table_name = "leases"]
pub struct CreateLease {
    user_id: i32,

    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl CreateLease {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, FromForm)]
pub(crate) struct CreateLeaseForm {
    end_time: DateField,
}

impl CreateLeaseForm {
    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time.0
    }

    pub fn into_create_lease(self, user_id: i32) -> CreateLease {
        CreateLease::builder()
            .user_id(user_id)
            .start_time(Utc::now())
            .end_time(self.end_time.0)
            .build()
    }
}
