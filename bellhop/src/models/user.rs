//! The most basic user database model you could ever hope to find.
//!
//! Most of the functionality you'd expect from a user comes from plugin crates
//! like `bellhop-auth-header` or `bellhop-auth-dummy`.

use crate::db::Db as PubDb;
use crate::errors::*;
use crate::internal::auth::Auths;
use crate::internal::db::Db;
use crate::schema::users;

use diesel::prelude::*;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request, State};
use rocket::Outcome;

/// A `User` is Bellhop's representation of a person or API client.
#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i32,
    email: String,
}

impl User {
    /// Find a `User` by its email address.
    pub fn by_email(c: &PubDb, by_email: &str) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(email.eq(by_email))
            .limit(1)
            .load::<User>(c.db())
            .chain_err(|| "failed to find user by email")?;

        Ok(user.pop())
    }

    /// Find a `User` by its primary key.
    pub fn by_id(c: &PubDb, by_id: i32) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(id.eq(by_id))
            .limit(1)
            .load::<User>(c.db())
            .chain_err(|| "failed to find user by id")?;

        Ok(user.pop())
    }

    /// The primary key of this `User`.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// The email address of this `User`.
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
            let maybe_user = match auth.authenticate(&(&db).into(), request) {
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

/// The insertable companion of `User`.
///
/// ## Example
///
/// ```no_run
/// use bellhop::db::Db;
/// use bellhop::models::user::CreateUser;
///
/// // The `db` argument could come from implementing `bellhop::hooks::Hook`, or
/// // as a Rocket request guard.
/// fn some_function(db: &Db) {
///     let new_user = CreateUser::builder()
///         .email("ew@example.com")
///         .build()
///         .insert(db)
///         .unwrap();
/// }
/// ```
#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "users"]
pub struct CreateUser {
    email: String,
}

impl CreateUser {
    /// The email address of the `User` to be created.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Insert the `User` into the database and return it.
    ///
    /// See the struct documentation for an example.
    pub fn insert(&self, c: &PubDb) -> Result<User> {
        use self::users::dsl::*;

        diesel::insert_into(users)
            .values(self)
            .get_result(c.db())
            .chain_err(|| "unable to insert user")
    }
}
