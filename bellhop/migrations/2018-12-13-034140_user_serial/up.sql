CREATE SEQUENCE users_id_seq MINVALUE 100;

ALTER TABLE users ALTER id SET DEFAULT nextval('users_id_seq');

ALTER SEQUENCE users_id_seq OWNED BY users.id;
