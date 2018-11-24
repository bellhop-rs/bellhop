use crate::hooks::{Data, Hook};

use diesel::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct Hooks(
    pub Vec<Box<Hook<<PgConnection as Connection>::Backend, PgConnection> + Sync + Send>>,
);

impl Hooks {
    pub fn returned(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.returned(db, data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn leased(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.leased(db, data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn evicted(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.evicted(db, data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }

    pub fn warned(&self, db: &PgConnection, data: Data) -> crate::errors::Result<()> {
        use crate::errors::*;

        for hook in self.0.iter() {
            hook.warned(db, data.clone())
                .chain_err(|| "error running hook")?;
        }

        Ok(())
    }
}
