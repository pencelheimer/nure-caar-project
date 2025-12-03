-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT chk_email_format CHECK (email LIKE '%@%')
);

CREATE TRIGGER update_user_modtime BEFORE UPDATE ON "user" FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();


-- migrate:down
DROP TRIGGER update_user_modtime ON "user";
DROP TABLE "user";
