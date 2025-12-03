-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE alert_rule (
    id SERIAL PRIMARY KEY,
    reservoir_id INTEGER NOT NULL REFERENCES reservoir(id) ON DELETE CASCADE,
    condition_type alert_condition_type NOT NULL,
    threshold DOUBLE PRECISION NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TRIGGER update_alert_rule_modtime BEFORE UPDATE ON alert_rule FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE alert (
    id SERIAL PRIMARY KEY,
    rule_id INTEGER NOT NULL REFERENCES alert_rule(id) ON DELETE CASCADE,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_to VARCHAR(255) NOT NULL,
    status alert_status NOT NULL
);


-- migrate:down
DROP TABLE alert;
DROP TRIGGER update_alert_rule_modtime ON alert_rule;
DROP TABLE alert_rule;
