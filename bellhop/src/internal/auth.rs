use crate::auth::Auth;

#[derive(Debug, Default)]
pub struct Auths(pub Vec<Box<dyn Auth + Sync + Send>>);
