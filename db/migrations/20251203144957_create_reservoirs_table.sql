-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE reservoir (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    capacity DOUBLE PRECISION NOT NULL,
    location VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT chk_capacity_positive CHECK (capacity > 0)
);

CREATE TRIGGER update_reservoir_modtime BEFORE UPDATE ON reservoir FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();


-- migrate:down
DROP TRIGGER update_reservoir_modtime ON reservoir;
DROP TABLE reservoir;
