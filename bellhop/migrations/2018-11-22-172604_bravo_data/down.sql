DELETE FROM
    tags
WHERE
    asset_id >= 200
AND
    asset_id <= 202
AND
    tag_type_id >= 500
AND
    tag_type_id <= 502;


DELETE FROM
    tag_types
WHERE
    id >= 500
AND
    id <= 502;

DELETE FROM assets WHERE type_id = 1;

DELETE FROM asset_types WHERE id = 1;
