use chrono::prelude::*;

use crate::errors::*;
use crate::schema::sheriff;

use diesel::prelude::*;

use std::time::Duration;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "sheriff"]
pub struct Sheriff {
    primary_key: bool,
    last_checked: Option<DateTime<Utc>>,
}

impl Sheriff {
    pub fn should_run(c: &PgConnection, period: Duration) -> Result<bool> {
        use self::sheriff::dsl::*;

        // https://github.com/diesel-rs/diesel/issues/1514
        let fragment = format!("now() - interval '{} milliseconds'", period.as_millis());
        let target = sheriff.filter(last_checked.lt(diesel::dsl::sql(&fragment)));

        let count = diesel::update(target)
            .set(last_checked.eq(diesel::dsl::now))
            .execute(c)
            .chain_err(|| "unable to update last_checked time for sheriff")?;

        Ok(count == 1)
    }
}
