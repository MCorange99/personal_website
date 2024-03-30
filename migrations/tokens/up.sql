
CREATE TABLE IF NOT EXISTS Tokens (
    token TEXT NOT NULL UNIQUE,
    owner_id UUID NOT NULL,
    permissions BIGINT NOT NULL,
    PRIMARY KEY (token)
)
