use crate::errors::*;
use crate::hooks::Data as HookData;
use crate::internal::db::Db;
use crate::internal::hooks::Hooks;
use crate::internal::uri::Base;
use crate::models::asset::{Asset, CreateAsset};
use crate::models::asset_type::AssetType;
use crate::models::lease::{CreateLeaseForm, Lease};
use crate::models::tag::{CreateOwnedTag, Tag};
use crate::models::user::User;

use diesel::prelude::*;

use rocket::http::hyper::header::Location;
use rocket::http::Status;
use rocket::request::State;

use rocket_contrib::json::Json;

use std::result::Result as StdResult;

use super::Paged;

#[get("/", format = "application/json")]
pub fn list(db: Db, _user: User) -> Result<Json<Paged<Asset>>> {
    use crate::schema::assets::dsl::*;

    let list = assets
        .load::<Asset>(&*db)
        .chain_err(|| "failed to list assets")?;

    Ok(Json(Paged::new(list)))
}

#[derive(Debug, Responder)]
#[response(status = 201)]
pub struct CreateSuccess {
    body: Json<Asset>,
    location: Location,
}

#[derive(Debug, Responder)]
pub enum Create {
    Success(CreateSuccess),

    Status(Status),
}

#[post("/", data = "<create>", format = "application/json")]
pub fn create(db: Db, user: User, create: Json<CreateAsset>, base: Base) -> Result<Create> {
    if !user.can_write() {
        return Ok(Create::Status(Status::Forbidden));
    }

    let created = create.insert(&db.into())?;
    let location = uri!(detail: asset_id = created.id());

    let result = CreateSuccess {
        location: Location(base.join(location).to_string()),
        body: Json(created),
    };

    Ok(Create::Success(result))
}

#[get("/<asset_id>", format = "application/json")]
pub fn detail(asset_id: i32, db: Db, _user: User) -> Result<Option<Json<Asset>>> {
    match Asset::by_id(&*db, asset_id)? {
        Some(a) => Ok(Some(Json(a))),
        None => Ok(None),
    }
}

#[get("/<asset_id>/tags", format = "application/json")]
pub fn tags(asset_id: i32, db: Db, _user: User) -> Result<Option<Json<Paged<Tag>>>> {
    let asset = match Asset::by_id(&*db, asset_id)? {
        Some(a) => a,
        None => return Ok(None),
    };

    let tags = Tag::belonging_to(&asset)
        .get_results(&*db)
        .chain_err(|| "unable to fetch tags for asset")?;

    Ok(Some(Json(Paged::new(tags))))
}

#[derive(Debug, Responder)]
#[response(status = 201)]
pub struct TagCreated {
    body: Json<Tag>,
    location: Location,
}

#[post("/<asset_id>/tags", data = "<create>", format = "application/json")]
pub fn create_tag(
    asset_id: i32,
    db: Db,
    user: User,
    create: Json<CreateOwnedTag>,
    base: Base,
) -> Result<StdResult<TagCreated, Status>> {
    if !user.can_write() {
        return Ok(Err(Status::Forbidden));
    }

    if let None = Asset::by_id(&*db, asset_id)? {
        return Ok(Err(Status::NotFound));
    }

    let form = create.into_inner().into_create_tag(asset_id);

    let created = form.insert(&db.into())?;
    let location = uri!(
        tag_detail: asset_id = created.asset_id(),
        tag_type_id = created.tag_type_id()
    );

    let result = TagCreated {
        location: Location(base.join(location).to_string()),
        body: Json(created),
    };

    Ok(Ok(result))
}

#[get("/<asset_id>/tags/<tag_type_id>", format = "application/json")]
pub fn tag_detail(
    asset_id: i32,
    tag_type_id: i32,
    db: Db,
    _user: User,
) -> Result<Option<Json<Tag>>> {
    use crate::schema::tags::dsl as t;

    let tag: Option<Json<Tag>> = t::tags
        .filter(t::asset_id.eq(asset_id).and(t::tag_type_id.eq(tag_type_id)))
        .get_result(&*db)
        .optional()
        .chain_err(|| "unable to get tag details")?
        .map(Json);

    Ok(tag)
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

#[derive(Debug, Responder)]
pub(crate) enum CreateLeaseResponse {
    #[response(status = 201)]
    Success(Json<Lease>),

    Status(Status),
}

#[put("/<asset_id>/lease", data = "<create>", format = "application/json")]
pub(crate) fn create_lease(
    asset_id: i32,
    db: Db,
    user: User,
    create: Json<CreateLeaseForm>,
    hooks: State<Hooks>,
) -> Result<CreateLeaseResponse> {
    use crate::schema::assets::dsl::*;

    let create_lease = create.into_inner().into_create_lease(user.id());

    let created = create_lease.insert(&(&*db).into())?;

    let to_update = assets.filter(id.eq(asset_id).and(lease_id.is_null()));

    // TODO: Leaks leases when the update fails
    let updated: Option<Asset> = diesel::update(to_update)
        .set(lease_id.eq(Some(created.id())))
        .get_result(&*db)
        .optional()
        .chain_err(|| "unable to update asset with new lease")?;

    let asset = match updated {
        Some(x) => x,
        None => return Ok(CreateLeaseResponse::Status(Status::Conflict)),
    };

    let asset_type = AssetType::by_id(&*db, asset.type_id())?.chain_err(|| "missing asset_type")?;

    let data = HookData::new(&created, &asset, &asset_type);
    hooks.leased(&*db, data)?;
    Ok(CreateLeaseResponse::Success(Json(created)))
}

#[delete("/<asset_id>/lease")]
pub(crate) fn delete_lease(
    asset_id: i32,
    db: Db,
    user: User,
    hooks: State<Hooks>,
) -> Result<Option<Status>> {
    use crate::schema::leases::dsl as leases;

    let asset = match Asset::by_id(&db, asset_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    let lease_id = match asset.lease_id() {
        Some(x) => x,
        None => return Ok(None),
    };

    let lease = match Lease::by_id(&*db, lease_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    let num_deleted_rows = match diesel::delete(leases::leases)
        .filter(leases::id.eq(lease_id).and(leases::user_id.eq(user.id())))
        .execute(&*db)
    {
        Ok(x) => x,
        Err(e) => bail!("Error deleting lease: {}", e),
    };

    println!(
        "Deleted {} rows for lease id {}",
        num_deleted_rows, lease_id
    );

    let retval = match num_deleted_rows {
        1 => Ok(Some(Status::NoContent)),
        _ => return Ok(Some(Status::Forbidden)),
    };

    let asset_type = AssetType::by_id(&*db, asset.type_id())?.chain_err(|| "missing asset_type")?;

    let data = HookData::new(&lease, &asset, &asset_type);
    hooks.returned(&*db, data)?;

    retval
}
