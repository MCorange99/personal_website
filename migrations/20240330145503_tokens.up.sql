-- Add up migration script here
CREATE TABLE IF NOT EXISTS tokens (
    token TEXT NOT NULL UNIQUE,
    owner_id UUID NOT NULL,
    permissions BIGINT NOT NULL,
    PRIMARY KEY (token)
)