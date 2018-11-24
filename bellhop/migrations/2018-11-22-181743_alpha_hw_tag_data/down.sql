DELETE FROM
    tags
WHERE
    asset_id >= 100
AND
    asset_id <= 115
AND
    tag_type_id >= 503
AND
    tag_type_id <= 505;


DELETE FROM
    tag_types
WHERE
    id >= 503
AND
    id <= 504;
