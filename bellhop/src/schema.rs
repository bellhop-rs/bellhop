table! {
    assets (id) {
        id -> Int4,
        type_id -> Int4,
        lease_id -> Nullable<Int4>,
        name -> Varchar,
    }
}

table! {
    asset_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    leases (id) {
        id -> Int4,
        user_id -> Int4,
        last_notified -> Nullable<Timestamptz>,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
    }
}

table! {
    tags (asset_id, tag_type_id) {
        asset_id -> Int4,
        tag_type_id -> Int4,
        value -> Varchar,
    }
}

table! {
    tag_types (id) {
        id -> Int4,
        asset_type_id -> Int4,
        name -> Varchar,
        detail_only -> Bool,
        rightness -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
    }
}

joinable!(assets -> asset_types (type_id));
joinable!(assets -> leases (lease_id));
joinable!(leases -> users (user_id));
joinable!(tag_types -> asset_types (asset_type_id));
joinable!(tags -> assets (asset_id));
joinable!(tags -> tag_types (tag_type_id));

allow_tables_to_appear_in_same_query!(
    assets,
    asset_types,
    leases,
    tags,
    tag_types,
    users,
);
