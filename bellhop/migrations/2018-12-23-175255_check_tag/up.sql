-- This function is used by a trigger to ensure that whenever a new tag is
-- inserted, the asset_type of the tag_type matches the asset_type of the asset.
CREATE FUNCTION tags_type_matches_asset() RETURNS TRIGGER AS $$
DECLARE result BOOLEAN;
BEGIN
    SELECT
        (SELECT type_id FROM assets WHERE id = NEW.asset_id)
        =
        (SELECT asset_type_id FROM tag_types WHERE id = NEW.tag_type_id)
    INTO result;
    IF result THEN
        RETURN NEW;
    ELSE
        RAISE EXCEPTION 'tag.asset.type_id must equal tag.tag_type.asset_type_id';
    END IF;
END;
$$
LANGUAGE PLPGSQL
IMMUTABLE
STRICT;

-- Ensure that the asset's asset_type_id matches the tag_type's asset_type_id
CREATE CONSTRAINT TRIGGER
    tags_type_matches_asset_trigger
AFTER
    INSERT OR UPDATE
ON
    tags
INITIALLY IMMEDIATE
FOR EACH ROW
    EXECUTE PROCEDURE tags_type_matches_asset();

-- This function is used by a trigger to ensure that assets and tag_types cannot
-- change their type_id. This is required because tags have a diamond-shaped
-- relationship to asset_types through assets and tag_types.
CREATE FUNCTION raise_type_exception() RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'moving an asset or tag_type between types is not supported (yet)';
END;
$$
LANGUAGE PLPGSQL
IMMUTABLE
STRICT;

-- Ensure that assets cannot change type_id
CREATE CONSTRAINT TRIGGER
    prevent_asset_change_type_trigger
AFTER
    UPDATE
ON
    assets
INITIALLY IMMEDIATE
FOR EACH ROW
    WHEN (OLD.type_id != NEW.type_id)
    EXECUTE PROCEDURE raise_type_exception();

-- Ensure that tag_types cannot change type_id
CREATE CONSTRAINT TRIGGER
    prevent_tag_type_change_type_trigger
AFTER
    UPDATE
ON
    tag_types
INITIALLY IMMEDIATE
FOR EACH ROW
    WHEN (OLD.asset_type_id != NEW.asset_type_id)
    EXECUTE PROCEDURE raise_type_exception();
