use rocket::http::uri::Origin;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, Route};

pub struct Base(Origin<'static>);

impl Base {
    pub fn join(&self, origin: Origin) -> Origin<'static> {
        let path = format!("{}{}", self.0.path(), origin);
        Origin::parse_owned(path).unwrap()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Base {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let route = match <&'r Route>::from_request(request) {
            Outcome::Success(s) => s,
            Outcome::Forward(f) => return Outcome::Forward(f),
            Outcome::Failure(f) => return Outcome::Failure(f),
        };

        Outcome::Success(Base(route.base.clone()))
    }
}
