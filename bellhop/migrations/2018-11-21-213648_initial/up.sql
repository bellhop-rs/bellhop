CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    email VARCHAR(1024) UNIQUE NOT NULL
);

CREATE TABLE leases (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,

    last_notified TIMESTAMP with time zone,
    start_time TIMESTAMP with time zone NOT NULL,
    end_time TIMESTAMP with time zone NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE asset_types (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE assets (
    id INTEGER PRIMARY KEY NOT NULL,
    type_id INTEGER NOT NULL,
    lease_id INTEGER,

    name VARCHAR(255) NOT NULL,

    FOREIGN KEY(type_id) REFERENCES asset_types(id),
    FOREIGN KEY(lease_id) REFERENCES leases(id) ON DELETE SET NULL,
    UNIQUE(type_id, name)
);

--
-- Sample Data
--
INSERT INTO users (id, email) VALUES (0, 'ew@example.com');

INSERT INTO asset_types (id, name) VALUES (0, 'Alpha Region');

INSERT INTO assets (id, type_id, name) VALUES (100, 0, '10013');
INSERT INTO assets (id, type_id, name) VALUES (101, 0, '10014');
INSERT INTO assets (id, type_id, name) VALUES (102, 0, '10015');
INSERT INTO assets (id, type_id, name) VALUES (103, 0, '10016');
INSERT INTO assets (id, type_id, name) VALUES (104, 0, '10017');

INSERT INTO assets (id, type_id, name) VALUES (105, 0, '10020');
INSERT INTO assets (id, type_id, name) VALUES (106, 0, '10021');
INSERT INTO assets (id, type_id, name) VALUES (107, 0, '10022');
INSERT INTO assets (id, type_id, name) VALUES (108, 0, '10023');
INSERT INTO assets (id, type_id, name) VALUES (109, 0, '10024');
INSERT INTO assets (id, type_id, name) VALUES (110, 0, '10025');
INSERT INTO assets (id, type_id, name) VALUES (111, 0, '10026');
INSERT INTO assets (id, type_id, name) VALUES (112, 0, '10027');
INSERT INTO assets (id, type_id, name) VALUES (113, 0, '10028');
INSERT INTO assets (id, type_id, name) VALUES (114, 0, '10029');
INSERT INTO assets (id, type_id, name) VALUES (115, 0, '10150');
