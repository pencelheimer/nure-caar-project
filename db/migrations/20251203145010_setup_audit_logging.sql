-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE audit_log (
    id SERIAL PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL,
    old_values JSONB,
    new_values JSONB,
    changed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION log_audit_event()
RETURNS TRIGGER AS $$
DECLARE
    old_row JSONB := NULL;
    new_row JSONB := NULL;
    rec_id TEXT;
BEGIN
    IF (TG_OP = 'DELETE') THEN
        rec_id := OLD.id::TEXT;
        old_row := to_jsonb(OLD);
    ELSIF (TG_OP = 'UPDATE') THEN
        rec_id := NEW.id::TEXT;
        old_row := to_jsonb(OLD);
        new_row := to_jsonb(NEW);
    ELSIF (TG_OP = 'INSERT') THEN
        rec_id := NEW.id::TEXT;
        new_row := to_jsonb(NEW);
    END IF;

    INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
    VALUES (TG_TABLE_NAME::TEXT, rec_id, TG_OP, old_row, new_row);

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER audit_system_settings_changes AFTER UPDATE ON system_settings FOR EACH ROW EXECUTE PROCEDURE log_audit_event();
CREATE TRIGGER audit_user_changes AFTER INSERT OR UPDATE OR DELETE ON "user" FOR EACH ROW EXECUTE PROCEDURE log_audit_event();
CREATE TRIGGER audit_device_changes AFTER INSERT OR UPDATE OR DELETE ON device FOR EACH ROW EXECUTE PROCEDURE log_audit_event();
CREATE TRIGGER audit_alert_rule_changes AFTER INSERT OR UPDATE OR DELETE ON alert_rule FOR EACH ROW EXECUTE PROCEDURE log_audit_event();


-- migrate:down
DROP TRIGGER audit_alert_rule_changes ON alert_rule;
DROP TRIGGER audit_device_changes ON device;
DROP TRIGGER audit_user_changes ON "user";
DROP TRIGGER audit_system_settings_changes ON system_settings;
DROP FUNCTION log_audit_event;

DROP TABLE audit_log;
