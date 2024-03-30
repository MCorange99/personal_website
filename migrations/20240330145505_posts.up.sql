-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    id UUID NOT NULL UNIQUE,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    img_url TEXT NOT NULL,
    origin_url TEXT NOT NULL,
    original_request TEXT NOT NULL,
    posted_on TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
)
