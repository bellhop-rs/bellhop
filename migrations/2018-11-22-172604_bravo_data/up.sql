INSERT INTO asset_types (id, name) VALUES (1, 'Bravo Region');
INSERT INTO tag_types (id, asset_type_id, name) VALUES (500, 1, 'IP Addresses');
INSERT INTO tag_types (id, asset_type_id, name) VALUES (501, 1, 'Region Type');
INSERT INTO tag_types (id, asset_type_id, name) VALUES (502, 1, 'Hardware');

INSERT INTO assets (id, type_id, name) VALUES (200, 1, 'Navy');
INSERT INTO assets (id, type_id, name) VALUES (201, 1, 'Mercury');
INSERT INTO assets (id, type_id, name) VALUES (202, 1, 'Red');

INSERT INTO
    tags (asset_id, tag_type_id, value)
VALUES
    (200, 500, '192.168.0.116'),
    (200, 501, 'All-In-One'),
    (200, 502, 'i7-6core'),

    (201, 500, '192.168.0.83, 192.168.0.84, 192.168.0.85'),
    (201, 501, 'Realtime'),
    (201, 502, 'Xeon'),

    (202, 500, '192.168.0.42, 192.168.0.43, 192.168.0.44, 192.168.0.45, 192.168.0.46'),
    (202, 501, 'Offline'),
    (202, 502, 'i7-16core')
;
