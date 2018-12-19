use crate::errors::*;
use crate::internal::db::Db;
use crate::models::asset::Asset;
use crate::models::asset_type::{AssetType, CreateAssetType};
use crate::models::tag_type::{CreateOwnedTagType, TagType};
use crate::models::user::User;

use diesel::prelude::*;

use rocket::http::hyper::header::Location;
use rocket::http::Status;

use rocket_contrib::json::Json;

#[get("/", format = "application/json")]
pub fn list(db: Db, _user: User) -> Result<Json<Vec<AssetType>>> {
    use crate::schema::asset_types::dsl::*;

    let list = asset_types
        .load::<AssetType>(&*db)
        .chain_err(|| "failed to list asset types")?;

    Ok(Json(list))
}

#[derive(Debug, Responder)]
#[response(status = 201)]
pub struct CreateSuccess {
    body: Json<AssetType>,
    location: Location,
}

#[derive(Debug, Responder)]
pub enum Create {
    Success(CreateSuccess),

    Status(Status),
}

#[post("/", data = "<create>", format = "application/json")]
pub fn create(db: Db, user: User, create: Json<CreateAssetType>) -> Result<Create> {
    if !user.can_write() {
        return Ok(Create::Status(Status::Forbidden));
    }

    let created = create.insert(&db.into())?;

    let result = CreateSuccess {
        location: Location(uri!(detail: type_id = created.id()).to_string()),
        body: Json(created),
    };

    Ok(Create::Success(result))
}

#[get("/<type_id>", format = "application/json")]
pub fn detail(type_id: i32, db: Db, _user: User) -> Result<Option<Json<AssetType>>> {
    match AssetType::by_id(&*db, type_id)? {
        Some(a) => Ok(Some(Json(a))),
        None => Ok(None),
    }
}

#[derive(Debug, Responder)]
#[response(status = 201)]
pub struct CreateTagTypeSuccess {
    body: Json<TagType>,
    location: Location,
}

#[derive(Debug, Responder)]
pub enum CreateTagType {
    Success(CreateTagTypeSuccess),
    Status(Status),
}

#[post("/<type_id>/tag-types", data = "<create>", format = "application/json")]
pub fn create_tag_type(
    type_id: i32,
    db: Db,
    user: User,
    create: Json<CreateOwnedTagType>,
) -> Result<CreateTagType> {
    if !user.can_write() {
        return Ok(CreateTagType::Status(Status::Forbidden));
    }

    if let None = AssetType::by_id(&*db, type_id)? {
        return Ok(CreateTagType::Status(Status::NotFound));
    }

    let form = create.into_inner().into_create_tag_type(type_id);

    let created = form.insert(&db.into())?;

    let result = CreateTagTypeSuccess {
        location: Location(
            uri!(
                tag_type_detail: type_id = type_id,
                tag_type_id = created.id()
            )
            .to_string(),
        ),
        body: Json(created),
    };

    Ok(CreateTagType::Success(result))
}

#[get("/<type_id>/tag-types/<tag_type_id>", format = "application/json")]
pub fn tag_type_detail(
    type_id: i32,
    tag_type_id: i32,
    db: Db,
    _user: User,
) -> Result<Option<Json<TagType>>> {
    let tag_type = match TagType::by_id(&*db, tag_type_id)? {
        Some(a) => a,
        None => return Ok(None),
    };

    if tag_type.asset_type_id() != type_id {
        return Ok(None);
    }

    Ok(Some(Json(tag_type)))
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