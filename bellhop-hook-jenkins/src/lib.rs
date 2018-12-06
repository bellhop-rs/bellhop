#![allow(proc_macro_derive_resolution_fallback)] // Should be fixed in the next major Diesel version

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod models;
mod schema;

use crate::models::{HookPoint, JenkinsHook};

use bellhop::hooks::{Data, Error, ErrorKind, Hook};

use diesel::prelude::*;

use reqwest::Client;

#[derive(Debug)]
pub struct Jenkins;

impl Jenkins {
    fn run<B, Conn>(db: &Conn, data: Data, by_hook_at: HookPoint) -> Result<(), Error>
    where
        Conn: Connection<Backend = B>,
        B: diesel::backend::Backend<RawValue = [u8]>,
    {
        use crate::schema::jenkins_hooks::dsl::*;

        let hook: Option<JenkinsHook> = JenkinsHook::belonging_to(data.asset_type())
            .filter(hook_at.eq(by_hook_at as i16))
            .get_result(db)
            .optional()
            .map_err(Error::for_kind(ErrorKind::msg("database error")))?;

        let hook = match hook {
            Some(x) => x,
            None => return Ok(()),
        };

        #[derive(Debug, Serialize)]
        struct Kv {
            name: String,
            value: String,
        }

        #[derive(Debug, Serialize)]
        struct Body {
            parameter: Vec<Kv>,
        }

        let body = Body {
            parameter: vec![Kv {
                name: "hook_at".to_owned(),
                value: by_hook_at.to_string(),
            }],
        };

        let client = Client::new();
        let resp = client
            .post(hook.url())
            .basic_auth(hook.username(), Some(hook.token()))
            .json(&body)
            .send();

        println!("POST: {:?}", &resp);
        println!("POST RESP: {:?}", resp.unwrap().text());

        Ok(())
    }
}

impl<B, Conn> Hook<B, Conn> for Jenkins
where
    Conn: Connection<Backend = B>,
    B: diesel::backend::Backend<RawValue = [u8]>,
{
    fn leased(&self, db: &Conn, data: Data) -> Result<(), Error> {
        Self::run(db, data, HookPoint::Leased)
    }

    fn returned(&self, db: &Conn, data: Data) -> Result<(), Error> {
        Self::run(db, data, HookPoint::Returned)
    }

    fn evicted(&self, db: &Conn, data: Data) -> Result<(), Error> {
        Self::run(db, data, HookPoint::Evicted)
    }
}
