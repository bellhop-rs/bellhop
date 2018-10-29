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
extern crate lettre;
extern crate lettre_email;

mod db;
mod errors;
mod hooks;
mod models;
mod schema;
mod sheriff;
mod views;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
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
        .attach(db::Db::fairing())
        .launch();
}
