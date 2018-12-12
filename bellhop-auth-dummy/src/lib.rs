#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod views;

use bellhop::auth::*;
use bellhop::models::user::User;
use bellhop::db::Db;

use rocket::Rocket;
use rocket::request::Request;
use rocket::response::Redirect;

const LOGIN_COOKIE: &str = "user_login";

#[catch(401)]
fn unauthorized() -> Redirect {
    Redirect::to("/auth/dummy/login")
}

#[derive(Debug)]
pub struct Dummy;

impl Auth for Dummy {
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        rocket
            .register(catchers![unauthorized])
            .mount("/auth/dummy/", routes![views::login_get, views::logout, views::login_post])
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

        let user = User::by_id(c, user_id)
            .map_err(Error::for_kind(ErrorKind::msg("unable to get user from db")))?;

        match user {
            Some(x) => Ok(Some(x)),
            None => Ok(None),
        }
    }
}
