DROP TRIGGER tags_type_matches_asset_trigger ON tags;
DROP FUNCTION tags_type_matches_asset();

DROP TRIGGER prevent_tag_type_change_type_trigger ON tag_types;
DROP TRIGGER prevent_asset_change_type_trigger ON assets;
DROP FUNCTION raise_type_exception();
