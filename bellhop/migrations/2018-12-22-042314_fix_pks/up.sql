CREATE SEQUENCE assets_id_seq MINVALUE 500;

ALTER TABLE assets ALTER id SET DEFAULT nextval('assets_id_seq');

ALTER SEQUENCE assets_id_seq OWNED BY assets.id;
