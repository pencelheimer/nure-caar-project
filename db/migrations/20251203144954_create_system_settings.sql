-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE system_settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    maintenance_mode BOOLEAN NOT NULL DEFAULT false,
    registration_enabled BOOLEAN NOT NULL DEFAULT true,
    default_data_retention_days INTEGER NOT NULL DEFAULT 30,
    CONSTRAINT chk_singleton CHECK (id = 1),
    CONSTRAINT chk_retention_positive CHECK (default_data_retention_days > 0)
);

INSERT INTO system_settings (id, maintenance_mode, registration_enabled) VALUES (1, false, true);


-- migrate:down
DROP TABLE system_settings;
