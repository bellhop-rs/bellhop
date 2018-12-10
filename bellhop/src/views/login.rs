use crate::db::Db;
use crate::errors::*;
use crate::models::user::{CreateUser, User};

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use std::result::Result as StdResult;

pub const LOGIN_COOKIE: &str = "user_login";

/*****************************************
Everything below is mouted under: "/login"
******************************************/

#[get("/")]
pub fn home(user: Option<User>) -> Result<Template> {
    if let Some(user) = user {
        #[derive(Serialize)]
        struct Context {
            user: User,
        }

        Ok(Template::render("types/have_access", Context { user }))
    } else {
        Ok(Template::render("login/home", ()))
    }
}

#[post("/", data = "<user>")]
pub(crate) fn submit(
    user: Form<CreateUser>,
    mut cookies: Cookies,
    db: Db,
) -> Result<StdResult<Redirect, Template>> {
    let user = match User::by_email(&*db, user.email())? {
        Some(x) => x,
        None => return Ok(StdResult::Err(Template::render("login/home", ""))),
    };

    let cookie = Cookie::new(LOGIN_COOKIE, user.id().to_string());
    cookies.add_private(cookie);

    let dest = format!("/");
    Ok(StdResult::Ok(Redirect::to(dest)))
}

/*****************************************
Everything below is mouted under: "/logout"
******************************************/

/// Remove the `user_id` cookie.
#[get("/")]
pub fn logout(mut cookies: Cookies) -> Result<Redirect> {
    cookies.remove_private(Cookie::named(LOGIN_COOKIE));
    Ok(Redirect::to("/login"))
}
