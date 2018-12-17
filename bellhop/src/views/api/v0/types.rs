use crate::errors::*;
use crate::internal::db::Db;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::tag_type::TagType;
use crate::models::user::User;

use diesel::prelude::*;

use rocket_contrib::json::Json;

#[get("/", format = "application/json")]
pub fn list(db: Db, _user: User) -> Result<Json<Vec<AssetType>>> {
    use crate::schema::asset_types::dsl::*;

    let list = asset_types
        .load::<AssetType>(&*db)
        .chain_err(|| "failed to list asset types")?;

    Ok(Json(list))
}

#[get("/<type_id>", format = "application/json")]
pub fn detail(type_id: i32, db: Db, _user: User) -> Result<Option<Json<AssetType>>> {
    match AssetType::by_id(&*db, type_id)? {
        Some(a) => Ok(Some(Json(a))),
        None => Ok(None),
    }
}

#[get("/<type_id>/tag-types", format = "application/json")]
pub fn tag_types(type_id: i32, db: Db, _user: User) -> Result<Option<Json<Vec<TagType>>>> {
    let asset_type = match AssetType::by_id(&*db, type_id)? {
        Some(a) => a,
        None => return Ok(None),
    };

    let types = TagType::belonging_to(&asset_type)
        .load(&*db)
        .chain_err(|| "unable to get tag types belonging to an asset type")?;

    Ok(Some(Json(types)))
}

#[get("/<type_id>/assets", format = "application/json")]
pub fn assets(type_id: i32, db: Db, _user: User) -> Result<Option<Json<Vec<Asset>>>> {
    let asset_type = match AssetType::by_id(&*db, type_id)? {
        Some(a) => a,
        None => return Ok(None),
    };

    let assets = Asset::belonging_to(&asset_type)
        .load(&*db)
        .chain_err(|| "unable to get assets belonging to an asset type")?;

    Ok(Some(Json(assets)))
}
