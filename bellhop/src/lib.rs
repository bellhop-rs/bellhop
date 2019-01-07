//! Bellhop is a web application for sharing and reserving assets (like lab
//! computers, test credentials, etc.) among members of a team.
//!
//! There are some plugins for supporting authentication and for integration
//! with other services. See the Bellhop website https://bellhop.rs for more
//! documentation and usage examples.

#![allow(proc_macro_derive_resolution_fallback)] // Should be fixed in the next major Diesel version
#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]

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
pub mod errors;
pub mod hooks;
mod internal;
pub mod models;
mod schema;
mod sheriff;
mod views;

use crate::auth::Auth;
use crate::hooks::Hook;
use crate::internal::auth::Auths;
use crate::internal::hooks::Hooks;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

/// Configuration for a Bellhop server.
///
/// ## Example
///
/// ```no_run
/// use bellhop::Bellhop;
///
/// fn main() {
///     Bellhop::default()
///         .start() // Start running the server.
/// }
/// ```
///
/// ## See Also
///
/// The `bellhop-demo` crate has a more fully featured example that includes
/// attaching plugins.
#[derive(Debug, Default)]
pub struct Bellhop {
    hooks: Hooks,
    auths: Auths,
}

impl Bellhop {
    /// Add a hook plugin.
    ///
    /// `Hook` plugins provide additional functionality when the status of an
    /// [`models::asset::Asset`] changes.
    pub fn hook<H>(mut self, hook: H) -> Self
    where
        H: 'static + Send + Sync + Hook,
    {
        self.hooks.0.push(Box::new(hook));
        self
    }

    /// Add an authentication plugin.
    ///
    /// The order of `auth` calls is significant. All added `Auth` instances
    /// are tried in the order that they were added. The first `Auth` to return
    /// a [`models::user::User`] is used to authenticate. If any `Auth` returns
    /// a failure, authentication is aborted.
    pub fn auth<A>(mut self, auth: A) -> Self
    where
        A: 'static + Send + Sync + Auth,
    {
        self.auths.0.push(Box::new(auth));
        self
    }

    /// Launch the Bellhop server.
    pub fn start(self) {
        let mut r = rocket::ignite()
            .mount("/api/v0/", routes![views::api::v0::docs])
            .mount(
                "/api/v0/types/",
                routes![
                    views::api::v0::types::list,
                    views::api::v0::types::detail,
                    views::api::v0::types::tag_types,
                    views::api::v0::types::tag_type_detail,
                    views::api::v0::types::assets,
                    views::api::v0::types::create,
                    views::api::v0::types::create_tag_type,
                ],
            )
            .mount(
                "/api/v0/assets/",
                routes![
                    views::api::v0::assets::create,
                    views::api::v0::assets::create_lease,
                    views::api::v0::assets::delete_lease,
                    views::api::v0::assets::list,
                    views::api::v0::assets::detail,
                    views::api::v0::assets::tags,
                    views::api::v0::assets::tag_detail,
                    views::api::v0::assets::create_tag,
                    views::api::v0::assets::lease,
                ],
            )
            .mount("/", routes![views::types::have_access])
            .mount("/", routes![views::favicon::favicon])
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

        for hook in self.hooks.0.iter() {
            r = hook.prelaunch(r);
        }

        for auth in self.auths.0.iter() {
            r = auth.prelaunch(r);
        }

        r.manage(self.hooks).manage(self.auths).launch();
    }
}
