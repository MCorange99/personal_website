
CREATE TABLE IF NOT EXISTS Posts (
    id UUID NOT NULL UNIQUE,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    img_url TEXT NOT NULL,
    origin_url TEXT NOT NULL,
    original_request JSON NOT NULL,
    posted_on TIMESTAMP NOT NULL
    PRIMARY KEY (id)
)
