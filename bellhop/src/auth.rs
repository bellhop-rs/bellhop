use crate::db::Db;
use crate::models::user::User;

use rocket::Rocket;
use rocket::request::Request;

use std::fmt;
use std::error::Error as StdError;

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

pub trait Auth: fmt::Debug {
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket
    }

    fn authenticate(&self, _conn: &Db, _req: &Request) -> Result<Option<User>, Error>;
}
