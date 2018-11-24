CREATE TABLE tag_types (
    id SERIAL PRIMARY KEY NOT NULL,
    asset_type_id INTEGER NOT NULL,

    name VARCHAR(255) NOT NULL,

    FOREIGN KEY(asset_type_id) REFERENCES asset_types(id),
    UNIQUE(asset_type_id, name)
);

CREATE TABLE tags (
    asset_id INTEGER NOT NULL,
    tag_type_id INTEGER NOT NULL,

    value VARCHAR(255) NOT NULL,

    FOREIGN KEY(asset_id) REFERENCES assets(id),
    FOREIGN KEY(tag_type_id) REFERENCES tag_types(id),
    PRIMARY KEY(asset_id, tag_type_id)
);
