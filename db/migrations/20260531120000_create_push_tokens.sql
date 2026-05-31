-- migrate:up

CREATE TABLE push_tokens (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    token       TEXT NOT NULL,
    platform    VARCHAR(20) NOT NULL DEFAULT 'android',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, token)
);

-- migrate:down

DROP TABLE IF EXISTS push_tokens;
