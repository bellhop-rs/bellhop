-- Tags to Assets
ALTER TABLE tags DROP CONSTRAINT tags_asset_id_fkey;
ALTER TABLE tags ADD CONSTRAINT tags_asset_id_fkey FOREIGN KEY (asset_id) REFERENCES assets(id);

-- Tags to TagTypes
ALTER TABLE tags DROP CONSTRAINT tags_tag_type_id_fkey;
ALTER TABLE tags ADD CONSTRAINT tags_tag_type_id_fkey FOREIGN KEY (tag_type_id) REFERENCES tag_types(id);
