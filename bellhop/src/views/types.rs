use crate::db::{get_all_types, Db};
use crate::errors::*;
use crate::models::asset::Asset;
use crate::models::asset_type::AssetType;
use crate::models::lease::Lease;
use crate::models::tag::Tag;
use crate::models::tag_type::TagType;
use crate::models::user::User;

use chrono::prelude::*;

use diesel::prelude::*;

use rocket::http::Cookies;
use rocket_contrib::templates::Template;

use std::collections::HashMap;
use std::result::Result as StdResult;

/*****************************************
Everything below is mouted under: "/"
******************************************/

#[get("/")]
pub fn have_access(db: Db, user: User) -> Result<Template> {
    // TODO: use user_id to get only assets you have access to...

    let asset_types = get_all_types(&db)?;

    #[derive(Serialize)]
    struct Context {
        asset_types: Vec<AssetType>,
        user: User,
    }

    Ok(Template::render(
        "types/have_access",
        Context { asset_types, user },
    ))
}

/*****************************************
Everything below is mouted under: "/types"
******************************************/

#[get("/")]
pub fn request_access(db: Db, user: User) -> Result<Template> {
    let asset_types = get_all_types(&db)?;

    #[derive(Serialize)]
    struct Context {
        asset_types: Vec<AssetType>,
        user: User,
    }

    Ok(Template::render(
        "types/request_access",
        Context { asset_types, user },
    ))
}

#[get("/<asset_type_id>")]
pub fn detail(asset_type_id: i32, db: Db, user: User) -> Result<Option<Template>> {
    use crate::schema::leases::dsl as leases;
    use crate::schema::tag_types::dsl as tt;

    let asset_type = match AssetType::by_id(&db, asset_type_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    let now = Utc::now();

    let assets_to_leases: Vec<(Asset, Option<Lease>)> = Asset::belonging_to(&asset_type)
        .left_outer_join(leases::leases)
        .load(&*db)
        .chain_err(|| "unable to get assets belonging to an asset type")?;

    let assets = assets_to_leases
        .iter()
        .map(|(asset, _)| asset.clone())
        .collect::<Vec<_>>();

    let tag_types: Vec<TagType> = TagType::belonging_to(&asset_type)
        .filter(tt::detail_only.eq(false))
        .order(tt::rightness.asc())
        .load(&*db)
        .chain_err(|| "unable to get tag types belonging to an asset type")?;

    let tags = Tag::belonging_to(&assets)
        .load::<Tag>(&*db)
        .chain_err(|| "unable to get tags belonging to assets")?
        .grouped_by(&assets);

    let asset_tags = assets_to_leases
        .into_iter()
        .zip(tags)
        .map(|((asset, lease), tags)| {
            let mut tags_by_type = tags
                .into_iter()
                .map(|tag| (tag.tag_type_id(), tag))
                .collect::<HashMap<_, _>>();

            let tags = tag_types
                .iter()
                .map(move |tt| tags_by_type.remove(&tt.id()))
                .collect();

            (
                asset,
                lease.map(|x| x.user_id() == user.id()).unwrap_or(false),
                tags,
            )
        })
        .collect::<Vec<_>>();

    #[derive(Serialize)]
    struct Context {
        tag_types: Vec<TagType>,
        asset_type: AssetType,
        asset_tags: Vec<(Asset, bool, Vec<Option<Tag>>)>,
        now: DateTime<Utc>,
        user: User,
    }

    Ok(Some(Template::render(
        "types/detail",
        Context {
            tag_types,
            asset_type,
            asset_tags,
            now,
            user,
        },
    )))
}
