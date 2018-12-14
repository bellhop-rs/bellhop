//! An implementation of [`bellhop::auth::Auth`] that authenticates a user based
//! on their email address (and *only* their email address.)
//!
//! Uses HTTP cookies to store the logged in user.
//!
//! ## Routes
//!
//! Provides a couple routes:
//!  - `/auth/dummy/login`: Handles logging in.
//!  - `/auth/dummy/logout`: Deletes the stored cookie.
//!
//! ## Catchers
//!
//! Provides one error catcher:
//!  - `401 Unauthorized`: Redirects to `/auth/dummy/login`.
//!
//! ## Example
//!
//! ```no_run
//! use bellhop::Bellhop;
//! use bellhop_auth_dummy::Dummy;
//!
//! fn main() {
//!     Bellhop::default()
//!         .auth(Dummy)
//!         .start()
//! }
//! ```

#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod views;

use bellhop::auth::*;
use bellhop::db::Db;
use bellhop::models::user::User;

use rocket::request::Request;
use rocket::response::Redirect;
use rocket::Rocket;

const LOGIN_COOKIE: &str = "user_login";

#[catch(401)]
fn unauthorized() -> Redirect {
    Redirect::to("/auth/dummy/login")
}

/// An implementation of [`bellhop::auth::Auth`].
///
/// See the crate documentation for more information.
#[derive(Debug)]
pub struct Dummy;

impl Auth for Dummy {
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket.register(catchers![unauthorized]).mount(
            "/auth/dummy/",
            routes![views::login_get, views::logout, views::login_post],
        )
    }

    fn authenticate(&self, c: &Db, req: &Request) -> Result<Option<User>, Error> {
        let mut cookies = req.cookies();

        let user_id_cookie = match cookies.get_private(LOGIN_COOKIE) {
            Some(x) => x,
            None => return Ok(None),
        };

        println!(
            "COOKIE: {:?} (value raw is: '{:?}')",
            user_id_cookie,
            user_id_cookie.value()
        );

        let user_id = match user_id_cookie.value().parse::<i32>() {
            Ok(x) => x,
            Err(_) => return Ok(None),
        };

        let user = User::by_id(c, user_id).map_err(Error::for_kind(ErrorKind::msg(
            "unable to get user from db",
        )))?;

        match user {
            Some(x) => Ok(Some(x)),
            None => Ok(None),
        }
    }
}
