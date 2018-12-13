use crate::db::Db as PubDb;
use crate::hooks::{Data, Hook};

use diesel::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct Hooks(pub Vec<Box<Hook + Sync + Send>>);

impl Hooks {
    pub fn returned(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.returned(&PubDb::from(db), data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn leased(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.leased(&PubDb::from(db), data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn evicted(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.evicted(&PubDb::from(db), data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn warned(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.warned(&PubDb::from(db), data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }
}
