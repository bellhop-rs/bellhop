use crate::errors::*;
use crate::hooks::Data as HookData;
use crate::internal::hooks::Hooks;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::Lease;

use diesel;
use diesel::prelude::*;

use chrono::prelude::*;

pub(crate) fn send_eviction_notices(c: &PgConnection, hooks: &Hooks) -> Result<()> {
    use crate::schema::asset_types::dsl as at;
    use crate::schema::leases::dsl as l;

    let now = Utc::now();

    let all_leases: Vec<Lease> = l::leases
        .load::<Lease>(c)
        .chain_err(|| "failed to get leases for eviction notices")?;

    for lease in all_leases {
        let time_left = lease.end_time() - lease.start_time();
        let margin = time_left / 20;

        if now > (lease.end_time() - margin) {
            // TODO: This is an N+1 queries bug
            // TODO: Check/set last notified time
            let (asset, asset_type): (Asset, AssetType) = Asset::belonging_to(&lease)
                .inner_join(at::asset_types)
                .get_result(c)
                .chain_err(|| "unable to get asset and asset type for lease")?;

            let data = HookData::new(&lease, &asset, &asset_type);

            hooks.warned(c, data)?;
        }
    }
    Ok(())
}

pub(crate) fn evict(c: &PgConnection, hooks: &Hooks) -> Result<()> {
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
        "The sherrif successfully evicted {:?} occupants.",
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
