use crate::errors::*;
use crate::internal::db::Db;
use crate::models::asset::Asset;
use crate::models::lease::Lease;
use crate::models::tag::Tag;
use crate::models::user::User;

use diesel::prelude::*;

use rocket_contrib::json::Json;

#[get("/", format = "application/json")]
pub fn list(db: Db, _user: User) -> Result<Json<Vec<Asset>>> {
    use crate::schema::assets::dsl::*;

    let list = assets
        .load::<Asset>(&*db)
        .chain_err(|| "failed to list assets")?;

    Ok(Json(list))
}

#[get("/<asset_id>", format = "application/json")]
pub fn detail(asset_id: i32, db: Db, _user: User) -> Result<Option<Json<Asset>>> {
    match Asset::by_id(&*db, asset_id)? {
        Some(a) => Ok(Some(Json(a))),
        None => Ok(None),
    }
}

#[get("/<asset_id>/tags", format = "application/json")]
pub fn tags(asset_id: i32, db: Db, _user: User) -> Result<Option<Json<Vec<Tag>>>> {
    let asset = match Asset::by_id(&*db, asset_id)? {
        Some(a) => a,
        None => return Ok(None),
    };

    let tags = Tag::belonging_to(&asset)
        .get_results(&*db)
        .chain_err(|| "unable to fetch tags for asset")?;

    Ok(Some(Json(tags)))
}

#[get("/<asset_id>/lease", format = "application/json")]
pub fn lease(asset_id: i32, db: Db, _user: User) -> Result<Option<Json<Lease>>> {
    use crate::schema::assets::dsl as a;
    use crate::schema::leases::dsl as l;

    let got: Option<(Asset, Option<Lease>)> = a::assets
        .filter(a::id.eq(asset_id))
        .left_outer_join(l::leases)
        .get_result(&*db)
        .optional()
        .chain_err(|| "unable to get lease for asset")?;

    let lease = match got.and_then(|x| x.1) {
        Some(g) => g,
        None => return Ok(None),
    };

    Ok(Some(Json(lease)))
}
