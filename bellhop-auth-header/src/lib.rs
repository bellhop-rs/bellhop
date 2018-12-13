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
    re: Regex,
}

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

            let re = match Regex::new(re_str) {
                Ok(x) => x,
                Err(_) => return Err(rocket),
            };

            let state = AuthRegex {
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

        let email = match email {
            Some(x) => x,
            None => return Ok(None),
        };

        let user = User::by_email(c, email).map_err(Error::for_kind(ErrorKind::msg(
            "unable to get user for authentication",
        )))?;

        match user {
            None => self.register(c, email).map(Some),
            x => Ok(x),
        }
    }
}
