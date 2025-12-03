-- migrate:up
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('admin', 'user', 'viewer');
CREATE TYPE device_status AS ENUM ('online', 'offline', 'maintenance');
CREATE TYPE alert_condition_type AS ENUM ('greater_than', 'less_than', 'equals');
CREATE TYPE alert_status AS ENUM ('pending', 'sent', 'failed');

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';


-- migrate:down
DROP FUNCTION update_updated_at_column;
DROP TYPE alert_status;
DROP TYPE alert_condition_type;
DROP TYPE device_status;
DROP TYPE user_role;
DROP EXTENSION "uuid-ossp";
