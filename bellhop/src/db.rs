//! General database related types and functions.

use crate::internal::db::Db as InternalDb;

use diesel::backend::Backend;
use diesel::prelude::*;

use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use std::fmt;
use std::ops::Deref;

enum Rv<'a> {
    Ref(&'a PgConnection),
    Val(InternalDb),
}

impl<'a> fmt::Debug for Rv<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rv::Ref(_) => write!(f, "Rv::Ref(PgConnection)"),
            Rv::Val(ref v) => write!(f, "Rv::Val({:?})", v),
        }
    }
}

impl<'a> Deref for Rv<'a> {
    type Target = PgConnection;

    fn deref(&self) -> &PgConnection {
        match self {
            Rv::Ref(v) => v,
            Rv::Val(v) => &*v,
        }
    }
}

/// Opaque wrapper around a database connection.
///
/// Useful for model functions like [`models::user::User::by_email`].
// Unlike [`internal::db::Db`], this doesn't implement `Deref` and therefore
// doesn't leak the type of database connection.
#[derive(Debug)]
pub struct Db<'a>(Rv<'a>);

impl From<InternalDb> for Db<'static> {
    fn from(o: InternalDb) -> Self {
        Db(Rv::Val(o))
    }
}

impl<'a> From<&'a InternalDb> for Db<'a> {
    fn from(o: &'a InternalDb) -> Self {
        Db(Rv::Ref(&*o))
    }
}

impl<'a> From<&'a PgConnection> for Db<'a> {
    fn from(o: &'a PgConnection) -> Self {
        Db(Rv::Ref(o))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Db<'static> {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match req.guard::<InternalDb>() {
            Outcome::Success(s) => Outcome::Success(s.into()),
            Outcome::Forward(f) => Outcome::Forward(f),
            Outcome::Failure(e) => Outcome::Failure(e),
        }
    }
}

impl<'a> Db<'a> {
    pub(crate) fn db(&self) -> &diesel::PgConnection {
        &self.0
    }

    /// Return a reference to the underlying database type.
    ///
    /// You'll need this function if you want to create custom `diesel` queries
    /// that aren't wrapped by functions on the model structs.
    pub fn raw(&self) -> &impl Connection<Backend = impl Backend<RawValue = [u8]>> {
        &*self.0
    }
}
