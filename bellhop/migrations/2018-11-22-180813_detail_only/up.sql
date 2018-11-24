ALTER TABLE tag_types ADD COLUMN detail_only BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tag_types ADD COLUMN rightness INTEGER NOT NULL DEFAULT 0;

UPDATE tag_types SET detail_only = true WHERE id = 500;
