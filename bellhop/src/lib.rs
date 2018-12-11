#![allow(proc_macro_derive_resolution_fallback)] // Should be fixed in the next major Diesel version
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

pub mod auth;
pub mod db;
mod errors;
pub mod hooks;
mod internal;
pub mod models;
mod schema;
mod sheriff;
mod views;

use crate::auth::Auth;
use crate::hooks::Hook;
use crate::internal::hooks::Hooks;
use crate::internal::auth::Auths;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(Debug, Default)]
pub struct Bellhop {
    hooks: Hooks,
    auths: Auths,
}

impl Bellhop {
    pub fn hook<H>(mut self, hook: H) -> Self
    where
        H: 'static + Send + Sync + Hook,
    {
        self.hooks.0.push(Box::new(hook));
        self
    }

    pub fn auth<A>(mut self, auth: A) -> Self
    where
        A: 'static + Send + Sync + Auth,
    {
        self.auths.0.push(Box::new(auth));
        self
    }

    pub fn start(self) {
        let mut r = rocket::ignite()
            .mount("/", routes![views::types::have_access])
            .mount("/", routes![views::favicon::favicon])
            .mount("/login", routes![views::login::home, views::login::submit])
            .mount("/logout", routes![views::login::logout])
            .mount(
                "/types",
                routes![views::types::request_access, views::types::detail],
            )
            .mount(
                "/static",
                StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
            )
            .mount("/users", routes![views::user::detail])
            .mount(
                "/assets",
                routes![
                    views::assets::create_lease,
                    views::assets::delete_lease,
                    views::assets::detail
                ],
            )
            .mount("/", routes![views::endpoints::sheriff])
            .attach(Template::fairing())
            .attach(internal::db::Db::fairing());

        for auth in self.auths.0.iter() {
            r = auth.prelaunch(r);
        }

        r.manage(self.hooks)
            .manage(self.auths)
            .launch();
    }
}
