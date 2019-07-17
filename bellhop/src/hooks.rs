//! Traits and types for implementing hook plugins.
//!
//! ## Examples
//!
//! There are a couple example crates that implement the `Hook` trait:
//!  * `bellhop-hook-email` is a very simple example that demonstrates sending
//!    an email when the sheriff does its rounds.
//!  * `bellhop-hook-jenkins` is a more involved example that starts a Jenkins
//!    job.

use crate::db::Db;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::Lease;

use rocket::Rocket;

use std::error::Error as StdError;
use std::fmt;

/// The kinds of errors that can be returned from `Hook` functions.
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

/// The error type that can be returned from `Hook` functions.
#[derive(Debug)]
pub struct Error(pub ErrorKind, Option<Box<dyn StdError + Send>>);

impl Error {
    /// Return a closure that creates a new [`Error`] from the given [`ErrorKind`].
    /// Particularly useful with `map_err`:
    ///
    /// ```
    /// use bellhop::hooks::{Error, ErrorKind};
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
    fn cause(&self) -> Option<&dyn StdError> {
        self.1.as_ref().map(|x| Box::as_ref(x) as &dyn StdError)
    }
}

/// Data that is provided to `Hook` functions.
#[derive(Debug, Clone)]
pub struct Data<'a> {
    asset_type: &'a AssetType,
    asset: &'a Asset,
    lease: &'a Lease,
}

impl<'a> Data<'a> {
    pub(crate) fn new(lease: &'a Lease, asset: &'a Asset, asset_type: &'a AssetType) -> Self {
        Self {
            lease,
            asset,
            asset_type,
        }
    }

    /// The `AssetType` associated with the `Asset` that generated this event.
    pub fn asset_type(&self) -> &AssetType {
        self.asset_type
    }

    /// `Asset` associated with this event.
    pub fn asset(&self) -> &Asset {
        self.asset
    }

    /// `Lease` associated with this event.
    ///
    /// This object may already be deleted from the database by the time the
    /// hook is invoked.
    pub fn lease(&self) -> &Lease {
        self.lease
    }
}

/// Trait for plugins that want notifications when `Lease` events are generated.
pub trait Hook: fmt::Debug {
    /// Perform Rocket related setup, like attaching routes and fairings,
    /// reading configuration values, etc.
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket
    }

    /// Called for each hook when a lease is created.
    fn leased(&self, _conn: &Db, _data: Data) -> Result<(), Error> {
        Ok(())
    }

    /// Called for each hook when a lease is returned before it expires.
    fn returned(&self, _conn: &Db, _data: Data) -> Result<(), Error> {
        Ok(())
    }

    /// Called for each hook after a lease has been deleted.
    fn evicted(&self, _conn: &Db, _data: Data) -> Result<(), Error> {
        Ok(())
    }

    /// Called for each hook when the eviction notice should be sent.
    fn warned(&self, _conn: &Db, _data: Data) -> Result<(), Error> {
        Ok(())
    }
}
