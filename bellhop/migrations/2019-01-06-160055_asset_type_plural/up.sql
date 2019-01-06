ALTER TABLE asset_types ADD COLUMN plural_name VARCHAR(255) UNIQUE;

UPDATE asset_types SET plural_name = CONCAT(name, 's') WHERE plural_name IS NULL;

ALTER TABLE asset_types ALTER COLUMN plural_name SET NOT NULL;
