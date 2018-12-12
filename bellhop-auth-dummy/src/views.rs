use crate::LOGIN_COOKIE;

use bellhop::db::Db;
use bellhop::errors::*;
use bellhop::models::user::{CreateUser, User};

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::templates::Template;

#[get("/login")]
pub fn login_get(user: Option<User>) -> Template {
    if let Some(user) = user {
        #[derive(Serialize)]
        struct Context {
            user: User,
        }

        Template::render("types/have_access", Context { user })
    } else {
        Template::render("login/home", ())
    }
}

#[derive(Responder)]
pub(crate) enum LoginResult {
    Success(Redirect),
    Failure(Template),
}

#[post("/login", data = "<user>")]
pub(crate) fn login_post(
    user: Form<CreateUser>,
    mut cookies: Cookies,
    db: Db,
) -> Result<LoginResult> {
    let user = match User::by_email(&db, user.email())? {
        Some(x) => x,
        None => return Ok(LoginResult::Failure(Template::render("login/home", ""))),
    };

    let cookie = Cookie::new(LOGIN_COOKIE, user.id().to_string());
    cookies.add_private(cookie);

    let dest = format!("/");
    Ok(LoginResult::Success(Redirect::to(dest)))
}

/// Remove the `user_id` cookie.
#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Result<Redirect> {
    cookies.remove_private(Cookie::named(LOGIN_COOKIE));
    Ok(Redirect::to("/auth/dummy/login"))
}
