use crate::db::Db;
use crate::errors::*;
use crate::sheriff;

#[post("/sheriff/do_rounds")]
pub fn sheriff(db: Db) -> Result<Option<String>> {
    sheriff::evict(&db)?;
    sheriff::send_eviction_notices(&db)?;
    Ok(Some(
        "The Sheriff successfully make their rounds.\n".to_string(),
    ))
}
