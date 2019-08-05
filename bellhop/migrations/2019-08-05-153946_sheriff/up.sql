CREATE TABLE sheriff (
    primary_key BOOLEAN PRIMARY KEY NOT NULL DEFAULT true CHECK (primary_key),
    last_checked TIMESTAMP with time zone NOT NULL
);

INSERT INTO sheriff (last_checked) VALUES ('epoch');
