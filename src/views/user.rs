use crate::db::Db;
use crate::errors::*;
use crate::models::user::{CreateUser, User};
use crate::schema::users;

use diesel;
use diesel::prelude::*;

use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::templates::Template;

#[get("/show/<user_id>")]
pub fn detail(user_id: i32, db: Db) -> Result<Option<Template>> {
    let user = match User::by_id(&db, user_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    #[derive(Serialize)]
    struct Context {
        user: User,
    }

    Ok(Some(Template::render("user/detail", Context { user })))
}

#[post("/", data = "<create_user>")]
pub fn create(create_user: Form<CreateUser>, db: Db) -> Result<Redirect> {
    diesel::insert_into(users::table)
        .values(&*create_user)
        .execute(&*db)
        .chain_err(|| "unable to insert new user")?;

    let user = User::by_email(&db, create_user.email())?
        .chain_err(|| "unable to find user immediately after creation")?;

    let dest = format!("/users/show/{}", user.id());
    Ok(Redirect::to(dest))
}

#[get("/create")]
pub fn create_form() -> Template {
    Template::render("user/create", ())
}
