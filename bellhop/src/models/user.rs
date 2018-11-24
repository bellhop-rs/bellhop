use crate::errors::*;
use crate::schema::users;
use crate::views::login::LOGIN_COOKIE;

use diesel::prelude::*;

use rocket::http::Cookies;
use rocket_contrib::templates::Template;

use std::result::Result as StdResult;

#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i32,
    email: String,
}

impl User {
    pub fn from_current_cookies(
        c: &PgConnection,
        cookies: &mut Cookies,
    ) -> Result<StdResult<User, Template>> {
        let user_id_cookie = match cookies.get_private(LOGIN_COOKIE) {
            Some(x) => x,
            None => return Ok(StdResult::Err(Template::render("login/home", ""))),
        };

        println!(
            "COOKIE: {:?} (value raw is: '{:?}')",
            user_id_cookie,
            user_id_cookie.value()
        );

        let user_id = match user_id_cookie.value().parse::<i32>() {
            Ok(x) => x,
            Err(e) => bail!("Error parsing cookie value as user_id: {}", e),
        };

        let user = match User::by_id(c, user_id)? {
            Some(x) => x,
            None => bail!("No user for id: {}", user_id),
        };

        Ok(StdResult::Ok(user))
    }

    pub fn by_email(c: &PgConnection, by_email: &str) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(email.eq(by_email))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by email")?;

        Ok(user.pop())
    }

    pub fn by_id<B, Conn>(c: &Conn, by_id: i32) -> Result<Option<User>>
    where
        Conn: Connection<Backend = B>,
        B: diesel::backend::Backend<RawValue = [u8]>,
    {
        use self::users::dsl::*;

        let mut user = users
            .filter(id.eq(by_id))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by id")?;

        Ok(user.pop())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "users"]
pub(crate) struct CreateUser {
    email: String,
}

impl CreateUser {
    pub fn email(&self) -> &str {
        &self.email
    }
}
