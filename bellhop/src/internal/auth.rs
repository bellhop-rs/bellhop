use crate::auth::Auth;

use diesel::prelude::*;

#[derive(Debug, Default)]
pub struct Auths(
    pub Vec<Box<Auth<<PgConnection as Connection>::Backend, PgConnection> + Sync + Send>>,
);
