-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
    id UUID NOT NULL UNIQUE,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    pw_hash TEXT NOT NULL,
    permissions BIGINT NOT NULL,
    PRIMARY KEY (id)
)