INSERT INTO tag_types (id, asset_type_id, name) VALUES (503, 0, 'RAM');
INSERT INTO tag_types (id, asset_type_id, name) VALUES (504, 0, 'vCPUs');

INSERT INTO
    tags (asset_id, tag_type_id, value)
VALUES
    (100, 503, '32 GiB'),
    (100, 504, '8 cores'),

    (101, 503, '32 GiB'),
    (101, 504, '8 cores'),

    (102, 503, '32 GiB'),
    (102, 504, '8 cores'),

    (103, 503, '32 GiB'),
    (103, 504, '8 cores'),

    (104, 503, '32 GiB'),
    (104, 504, '8 cores'),

    (105, 503, '32 GiB'),
    (105, 504, '8 cores'),

    (106, 503, '32 GiB'),
    (106, 504, '8 cores'),

    (107, 503, '32 GiB'),
    (107, 504, '8 cores'),

    (108, 503, '32 GiB'),
    (108, 504, '8 cores'),

    (109, 503, '32 GiB'),
    (109, 504, '8 cores'),

    (110, 503, '32 GiB'),
    (110, 504, '8 cores'),

    (111, 503, '32 GiB'),
    (111, 504, '8 cores'),

    (112, 503, '32 GiB'),
    (112, 504, '8 cores'),

    (113, 503, '32 GiB'),
    (113, 504, '8 cores'),

    (114, 503, '16 GiB, 32 GiB'),
    (114, 504, '8 cores'),

    (115, 503, '8 GiB'),
    (115, 504, '4 cores')
;
