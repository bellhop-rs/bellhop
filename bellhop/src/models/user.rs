use crate::db::Db;
use crate::errors::*;
use crate::schema::users;
use crate::internal::auth::Auths;

use diesel::prelude::*;

use rocket::request::{self, Request, FromRequest, State};
use rocket::http::Status;
use rocket::Outcome;

#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i32,
    email: String,
}

impl User {
    pub fn by_email(c: &PgConnection, by_email: &str) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(email.eq(by_email))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by email")?;

        Ok(user.pop())
    }

    pub fn by_id<B, Conn>(c: &Conn, by_id: i32) -> Result<Option<User>>
    where
        Conn: Connection<Backend = B>,
        B: diesel::backend::Backend<RawValue = [u8]>,
    {
        use self::users::dsl::*;

        let mut user = users
            .filter(id.eq(by_id))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by id")?;

        Ok(user.pop())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    // TODO: Maybe switch to crate::errors::Error to get better debugging?
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let auths = match request.guard::<State<Auths>>() {
            Outcome::Success(x) => x,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        let db = match request.guard::<Db>() {
            Outcome::Success(x) => x,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        for auth in auths.0.iter() {
            let maybe_user = match auth.authenticate(&db, request) {
                Ok(u) => u,
                Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
            };

            if let Some(user) = maybe_user {
                return Outcome::Success(user);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "users"]
pub(crate) struct CreateUser {
    email: String,
}

impl CreateUser {
    pub fn email(&self) -> &str {
        &self.email
    }
}
