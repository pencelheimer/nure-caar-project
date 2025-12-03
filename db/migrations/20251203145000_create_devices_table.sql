-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE device (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    reservoir_id INTEGER REFERENCES reservoir(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    api_key VARCHAR(255) NOT NULL UNIQUE,
    status device_status NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TRIGGER update_device_modtime BEFORE UPDATE ON device FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();


-- migrate:down
DROP TRIGGER update_device_modtime ON device;
DROP TABLE device;
