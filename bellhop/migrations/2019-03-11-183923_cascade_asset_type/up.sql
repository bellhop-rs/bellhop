-- TagType to AssetType
ALTER TABLE tag_types DROP CONSTRAINT tag_types_asset_type_id_fkey;
ALTER TABLE tag_types ADD CONSTRAINT tag_types_asset_type_id_fkey FOREIGN KEY (asset_type_id) REFERENCES asset_types(id) ON DELETE CASCADE;

-- Asset to AssetType
ALTER TABLE assets DROP CONSTRAINT assets_type_id_fkey;
ALTER TABLE assets ADD CONSTRAINT assets_type_id_fkey FOREIGN KEY (type_id) REFERENCES asset_types(id) ON DELETE CASCADE;
