-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL UNIQUE,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    pw_hash TEXT NOT NULL,
    permissions BIGINT NOT NULL,
    PRIMARY KEY (id)
)