//! Traits and types for implementing authentication plugins.
//!
//! ## Examples
//!
//! There are a couple example crates that implement the `Auth` trait:
//!  * `bellhop-auth-header` is a very simple example that demonstrates
//!    authenticating a user based on a header. It also includes an example of
//!    registering new users.
//!  * `bellhop-auth-dummy` is a more involved example that includes new HTTP
//!    endpoints and a login page.

use crate::db::Db;
use crate::models::user::User;

use rocket::request::Request;
use rocket::Rocket;

use std::error::Error as StdError;
use std::fmt;

/// The kinds of errors that can be returned from `Auth` functions.
#[derive(Debug)]
pub enum ErrorKind {
    /// A custom error string.
    Msg(String),
}

impl ErrorKind {
    /// Create an [`ErrorKind`] with the given text message.
    pub fn msg<S: Into<String>>(s: S) -> Self {
        ErrorKind::Msg(s.into())
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Msg(ref x) => write!(f, "{}", x),
        }
    }
}

/// The error type that can be returned from `Auth` functions.
#[derive(Debug)]
pub struct Error(pub ErrorKind, Option<Box<StdError + Send>>);

impl Error {
    /// Return a closure that creates a new [`Error`] from the given [`ErrorKind`].
    /// Particularly useful with `map_err`:
    ///
    /// ```
    /// use bellhop::auth::{Error, ErrorKind};
    ///
    /// let value: i32 = "55"
    ///         .parse()
    ///         .map_err(Error::for_kind(ErrorKind::msg("unable to parse")))
    ///         .unwrap();
    /// ```
    pub fn for_kind<E>(kind: ErrorKind) -> impl FnOnce(E) -> Self
    where
        E: 'static + Send + StdError,
    {
        |error: E| Self::new(kind, error)
    }

    /// Create a new [`Error`] with the given kind and cause.
    pub fn new<E>(kind: ErrorKind, cause: E) -> Self
    where
        E: 'static + Send + StdError,
    {
        Error(kind, Some(Box::new(cause)))
    }

    /// Shortcut for creating an [`Error`] with [`ErrorKind::Msg`] and no cause.
    pub fn with_msg<S: Into<String>>(s: S) -> Self {
        Error(ErrorKind::Msg(s.into()), None)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        self.1.as_ref().map(|x| Box::as_ref(x) as &StdError)
    }
}

/// Trait for plugins that provide authentication.
///
/// See module documentation for more information.
pub trait Auth: fmt::Debug {
    /// Perform Rocket related setup, like attaching routes and fairings,
    /// reading configuration values, etc.
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket
    }

    /// Return the [`models::user::User`] that is authenticated in the given
    /// `Request`.
    ///
    /// A return value of `Ok(None)` means that there is no authenticated user
    /// according to this plugin. Authentication will be retried with the next
    /// plugin.
    ///
    /// On the other hand, a return value of `Err(...)` will abort the process,
    /// and no user authentication will be retried.
    fn authenticate(&self, _conn: &Db, _req: &Request) -> Result<Option<User>, Error>;
}
