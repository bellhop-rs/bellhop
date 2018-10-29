DELETE FROM
    tags
WHERE
    asset_id >= 100
AND
    asset_id <= 115
AND
    tag_type_id >= 400
AND
    tag_type_id <= 401;


DELETE FROM
    tag_types
WHERE
    id >= 400
AND
    id <= 401;
