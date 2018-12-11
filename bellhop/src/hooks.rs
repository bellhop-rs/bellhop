use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::Lease;
use crate::db::Db;

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
}

impl ErrorKind {
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

#[derive(Debug)]
pub struct Error(pub ErrorKind, Option<Box<StdError + Send>>);

impl Error {
    pub fn for_kind<E>(kind: ErrorKind) -> impl FnOnce(E) -> Self
    where
        E: 'static + Send + StdError,
    {
        |error: E| Self::new(kind, error)
    }

    pub fn new<E>(kind: ErrorKind, cause: E) -> Self
    where
        E: 'static + Send + StdError,
    {
        Error(kind, Some(Box::new(cause)))
    }

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

    pub fn asset_type(&self) -> &AssetType {
        self.asset_type
    }

    pub fn asset(&self) -> &Asset {
        self.asset
    }

    pub fn lease(&self) -> &Lease {
        self.lease
    }
}

pub trait Hook: fmt::Debug {
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
