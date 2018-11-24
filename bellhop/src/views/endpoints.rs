use crate::db::Db;
use crate::errors::*;
use crate::internal::hooks::Hooks;
use crate::sheriff;

use rocket::request::State;

#[post("/sheriff/do_rounds")]
pub(crate) fn sheriff(db: Db, hooks: State<Hooks>) -> Result<Option<String>> {
    sheriff::evict(&db, &hooks)?;
    sheriff::send_eviction_notices(&db, &hooks)?;
    Ok(Some(
        "The Sheriff successfully make their rounds.\n".to_string(),
    ))
}
