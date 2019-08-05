use chrono::prelude::*;

use crate::errors::*;
use crate::hooks::Data as HookData;
use crate::internal::db::DbPool;
use crate::internal::hooks::Hooks;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::Lease;
use crate::models::sheriff::Sheriff as SheriffModel;

use diesel;
use diesel::prelude::*;

use error_chain::ChainedError;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const PERIOD: Duration = Duration::from_secs(5 * 60);

struct Deputy {
    running: Arc<AtomicBool>,
    db_pool: DbPool,
    deadline: Instant,
    hooks: Hooks,
}

impl Deputy {
    fn new(db_pool: DbPool, running: Arc<AtomicBool>, hooks: Hooks) -> Self {
        Deputy {
            running,
            db_pool,
            hooks,
            deadline: Instant::now() + PERIOD, // TODO: Add jitter.
        }
    }

    fn wait(&self) -> bool {
        while self.running.load(Ordering::SeqCst) {
            let now = Instant::now();
            if now >= self.deadline {
                return true;
            }

            let timeout = self.deadline - now;

            thread::park_timeout(timeout);
        }

        false
    }

    fn run_one(&mut self) -> Result<()> {
        let conn = self
            .db_pool
            .get()
            .chain_err(|| "couldn't get database connection")?;

        if SheriffModel::should_run(&conn, PERIOD)? {
            evict(&conn, &self.hooks)?;
            send_eviction_notices(&conn, &self.hooks)?;
        }

        Ok(())
    }

    fn run(mut self) {
        while self.wait() {
            if let Err(e) = self.run_one() {
                eprintln!("deputy unable to run: {}", e.display_chain());
                std::process::exit(1);
            }

            self.deadline += PERIOD;
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sheriff {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for Sheriff {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        let handle = self.handle.take().expect("sheriff has no deputy");

        handle.thread().unpark();

        handle.join().expect("deputy thread panicked");
    }
}

impl Sheriff {
    fn new(db_pool: DbPool, hooks: Hooks) -> Self {
        let running = Arc::new(AtomicBool::new(true));

        let deputy = Deputy::new(db_pool, running.clone(), hooks);

        let handle = thread::Builder::new()
            .name("sheriff".into())
            .spawn(move || deputy.run())
            .expect("unable to start sheriff thread");

        Sheriff {
            running,
            handle: Some(handle),
        }
    }

    /// Returns a fairing that handles periodically evicting expired leases.
    /// Must be called after attaching the database fairing.
    pub fn fairing() -> impl ::rocket::fairing::Fairing {
        ::rocket::fairing::AdHoc::on_attach("Sheriff", |rocket| {
            let pool = match rocket.state::<DbPool>() {
                Some(p) => p,
                None => return Err(rocket),
            };

            let hooks = match rocket.state::<Hooks>() {
                Some(h) => h,
                None => return Err(rocket),
            };

            let sheriff = Self::new(pool.clone(), hooks.clone());

            Ok(rocket.manage(sheriff))
        })
    }
}

/// NB: Not really safe to run two copies of this at the same time.
fn send_eviction_notices(c: &PgConnection, hooks: &Hooks) -> Result<()> {
    use crate::schema::asset_types::dsl as at;
    use crate::schema::leases::dsl as l;

    let now = Utc::now();

    let all_leases: Vec<Lease> = l::leases
        .filter(l::last_notified.is_null())
        .load::<Lease>(c)
        .chain_err(|| "failed to get leases for eviction notices")?;

    for lease in all_leases {
        let time_left = lease.end_time() - lease.start_time();
        let margin = time_left / 20;

        if now > (lease.end_time() - margin) {
            // TODO: This is an N+1 queries bug
            let (asset, asset_type): (Asset, AssetType) = Asset::belonging_to(&lease)
                .inner_join(at::asset_types)
                .get_result(c)
                .chain_err(|| "unable to get asset and asset type for lease")?;

            let data = HookData::new(&lease, &asset, &asset_type);

            hooks.warned(c, data)?;

            diesel::update(&lease)
                .set(l::last_notified.eq(Some(now)))
                .execute(c)
                .chain_err(|| "unable to set last notified for lease")?;
        }
    }
    Ok(())
}

fn evict(c: &PgConnection, hooks: &Hooks) -> Result<()> {
    use crate::schema::asset_types::dsl as at;
    use crate::schema::leases::dsl as l;

    let to_delete: Vec<Lease> = l::leases
        .for_update()
        .filter(l::end_time.lt(Utc::now()))
        .load(c)
        .chain_err(|| "sheriff was unable to get leases")?;

    let assets: Vec<Vec<(Asset, AssetType)>> = Asset::belonging_to(&to_delete)
        .inner_join(at::asset_types)
        .load::<(Asset, AssetType)>(c)
        .chain_err(|| "sheriff was unable to get asset and type information")?
        .grouped_by(&to_delete);

    let to_delete_ids: Vec<_> = to_delete.iter().map(|x| x.id()).collect();

    let num_deleted_rows = diesel::delete(l::leases)
        .filter(l::id.eq_any(&to_delete_ids))
        .execute(c)
        .chain_err(|| "sheriff was unable to delete leases")?;

    println!(
        "The sheriff successfully evicted {:?} occupants.",
        num_deleted_rows,
    );

    for (lease, assets) in to_delete.into_iter().zip(assets) {
        for (asset, asset_type) in assets.into_iter() {
            let data = HookData::new(&lease, &asset, &asset_type);

            hooks
                .evicted(c, data)
                .chain_err(|| "sheriff encountered an error while sending hooks")?;
        }
    }

    Ok(())
}
