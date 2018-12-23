use crate::errors::*;
use crate::hooks::Data as HookData;
use crate::internal::db::Db;
use crate::internal::hooks::Hooks;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::{CreateLeaseForm, Lease};
use crate::models::tag::Tag;
use crate::models::tag_type::TagType;
use crate::models::user::User;

use diesel;
use diesel::prelude::*;

use rocket::http::Status;
use rocket::request::{Form, State};
use rocket::response::Redirect;

use rocket_contrib::templates::Template;

use std::result::Result as StdResult;

/*****************************************
Everything below is mouted under: "/assets"
******************************************/

#[put("/<asset_id>/lease", data = "<form>")]
pub(crate) fn create_lease(
    asset_id: i32,
    form: Form<CreateLeaseForm>,
    db: Db,
    user: User,
    hooks: State<Hooks>,
) -> Result<Option<StdResult<Redirect, Status>>> {
    use crate::schema::assets::dsl::*;

    let lease = form
        .into_inner()
        .into_create_lease(user.id())
        .insert(&(&*db).into())?;

    let to_update = assets.filter(id.eq(asset_id).and(lease_id.is_null()));

    // TODO: Leaks leases when the update fails
    let updated: Option<Asset> = diesel::update(to_update)
        .set(lease_id.eq(Some(lease.id())))
        .get_result(&*db)
        .optional()
        .chain_err(|| "unable to update asset with new lease")?;

    let asset = match updated {
        Some(x) => x,
        None => return Ok(Some(Err(Status::Conflict))),
    };

    let asset_type = AssetType::by_id(&*db, asset.type_id())?.chain_err(|| "missing asset_type")?;

    let data = HookData::new(&lease, &asset, &asset_type);
    hooks.leased(&*db, data)?;

    let dest = format!("/assets/{}", asset.id());
    Ok(Some(Ok(Redirect::to(dest))))
}

#[delete("/<asset_id>/lease")]
pub(crate) fn delete_lease(
    asset_id: i32,
    db: Db,
    user: User,
    hooks: State<Hooks>,
) -> Result<Option<StdResult<Redirect, Status>>> {
    use crate::views::api::v0::assets as api;

    match api::delete_lease(asset_id, db, user, hooks)? {
        Some(Status::NoContent) => {
            let dest = format!("/assets/{}", asset_id);
            Ok(Some(Ok(Redirect::to(dest))))
        }
        Some(x) => Ok(Some(Err(x))),
        None => Ok(None),
    }
}

#[get("/<asset_id>")]
pub fn detail(asset_id: i32, db: Db, user: User) -> Result<Option<Template>> {
    use crate::schema::tag_types::dsl as tt;
    use crate::schema::tags::dsl as t;

    let asset = match Asset::by_id(&db, asset_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    let asset_type = match AssetType::by_id(&db, asset.type_id())? {
        Some(x) => x,
        None => return Ok(None),
    };

    let tags = tt::tag_types
        .left_outer_join(t::tags)
        .filter(
            tt::asset_type_id
                .eq(asset_type.id())
                .and(t::asset_id.eq(asset_id)),
        )
        .get_results(&*db)
        .chain_err(|| "unable to get tags for asset")?;

    let lease = asset.fetch_lease_owner(&db)?;
    let user_owns_lease = lease
        .as_ref()
        .map(|(x, _)| x.user_id() == user.id())
        .unwrap_or(false);

    #[derive(Serialize)]
    struct Context {
        asset: Asset,
        asset_type: AssetType,
        tags: Vec<(TagType, Option<Tag>)>,
        lease: Option<(Lease, User)>,
        user: User,
        user_owns_lease: bool,
    }

    Ok(Some(Template::render(
        "assets/detail",
        Context {
            lease,
            user_owns_lease,
            tags,
            asset,
            asset_type,
            user,
        },
    )))
}
