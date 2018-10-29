CREATE TABLE jenkins_hooks (
    id SERIAL PRIMARY KEY NOT NULL,
    asset_type_id INTEGER NOT NULL,

    hook_at SMALLINT NOT NULL,
    username VARCHAR(64) NOT NULL,
    token VARCHAR(64) NOT NULL,
    url VARCHAR(1024) NOT NULL,

    FOREIGN KEY(asset_type_id) REFERENCES asset_types(id),
    UNIQUE(asset_type_id, hook_at)
);

-- INSERT INTO
--    jenkins_hooks (asset_type_id, hook_at, username, token, url)
-- VALUES
--    (0, 1, 'buildbot', '<token>', 'http://127.0.0.1:8081/job/bellhop_test/build');

