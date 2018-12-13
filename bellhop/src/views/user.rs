use crate::errors::*;
use crate::internal::db::Db;
use crate::models::user::User;

use diesel;

use rocket_contrib::templates::Template;

#[get("/show/<user_id>")]
pub fn detail(user_id: i32, db: Db) -> Result<Option<Template>> {
    let user = match User::by_id(&(&db).into(), user_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    #[derive(Serialize)]
    struct Context {
        user: User,
    }

    Ok(Some(Template::render("user/detail", Context { user })))
}
