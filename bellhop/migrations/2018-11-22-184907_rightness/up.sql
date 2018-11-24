UPDATE tag_types SET rightness = 100 where id = 504; --; vCPUs
UPDATE tag_types SET rightness = 200 where id = 503; --; RAM
UPDATE tag_types SET rightness = 300 where id = 400; --; IP Address
UPDATE tag_types SET rightness = 400 where id = 401; --; VIP Range

UPDATE tag_types SET detail_only = true WHERE id = 401;
