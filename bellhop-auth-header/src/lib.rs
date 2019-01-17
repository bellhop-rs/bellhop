//! An implementation of [`bellhop::auth::Auth`] that authenticates a user based
//! on a header.
//!
//! ## Routes
//!
//! Provides no routes.
//!
//! ## Catchers
//!
//! Provides no catchers.
//!
//! ## Configuration
//!
//! There are two optional configuration options that can be specified in
//! `Rocket.toml`:
//!  - `auth_header`: The name of the header to pull the email address from
//!    (Default: `X-Bellhop-Email`.)
//!  - `auth_header_email_pattern': A regular expression with a named capture
//!    group for the email address (Default: `(?P<email>.*)`)
//!
//! ## Example
//!
//! ```no_run
//! use bellhop::Bellhop;
//! use bellhop_auth_header::Header;
//!
//! fn main() {
//!     Bellhop::default()
//!         .auth(Header)
//!         .start()
//! }
//! ```

#![deny(missing_docs)]

use bellhop::auth::*;
use bellhop::db::Db;
use bellhop::models::user::{CreateUser, User};

use regex::Regex;

use rocket::fairing::AdHoc;
use rocket::request::{Request, State};
use rocket::{Outcome, Rocket};

#[derive(Debug)]
struct AuthRegex {
    header_name: String,
    suffix: String,
    re: Regex,
}

/// An implementation of [`bellhop::auth::Auth`] that authenticates based on a
/// header.
///
/// See the crate documentation for more details.
#[derive(Debug)]
pub struct Header;

const DEFAULT: &str = "(?P<email>.*)";

impl Header {
    fn register(&self, c: &Db, email: &str) -> Result<User, Error> {
        CreateUser::builder()
            .email(email)
            .build()
            .insert(c)
            .map_err(Error::for_kind(ErrorKind::msg(
                "unable to insert new user from header",
            )))
    }
}

impl Auth for Header {
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket.attach(AdHoc::on_attach("Auth Header Config", |rocket| {
            let name = rocket
                .config()
                .get_str("auth_header")
                .unwrap_or("X-Bellhop-Email");

            let re_str = rocket
                .config()
                .get_str("auth_header_email_pattern")
                .unwrap_or(DEFAULT);

            let suffix = rocket
                .config()
                .get_str("auth_header_default_domain")
                .ok()
                .map(|x| format!("@{}", x))
                .unwrap_or(String::new());

            let re = match Regex::new(re_str) {
                Ok(x) => x,
                Err(_) => return Err(rocket),
            };

            let state = AuthRegex {
                suffix,
                re,
                header_name: name.to_owned(),
            };

            Ok(rocket.manage(state))
        }))
    }

    fn authenticate(&self, c: &Db, req: &Request) -> Result<Option<User>, Error> {
        let auths = match req.guard::<State<AuthRegex>>() {
            Outcome::Success(x) => x,
            Outcome::Failure(_) => return Err(Error::with_msg("unable to get AuthRegex")),
            Outcome::Forward(_) => return Ok(None),
        };

        let header = match req.headers().get_one(&auths.header_name) {
            None | Some("") => return Ok(None),
            Some(x) => x,
        };

        let mut email = None;

        for capture in auths.re.captures_iter(header) {
            if let Some(x) = capture.name("email") {
                email = Some(x.as_str());
                break;
            }
        }

        let mut email = match email {
            Some(x) => x.to_owned(),
            None => return Ok(None),
        };

        if !email.contains('@') {
            email.push_str(&auths.suffix);
        }

        let user = User::by_email(c, &email).map_err(Error::for_kind(ErrorKind::msg(
            "unable to get user for authentication",
        )))?;

        match user {
            None => self.register(c, &email).map(Some),
            x => Ok(x),
        }
    }
}
