-- pgt-ignore-all lint/safety/banDropTable

-- migrate:up
CREATE TABLE measurement (
    time TIMESTAMPTZ NOT NULL,
    device_id INTEGER NOT NULL REFERENCES device(id) ON DELETE CASCADE,
    value DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (time, device_id)
);

CREATE INDEX idx_measurement_device_time ON measurement (device_id, time DESC);


-- migrate:down
DROP TABLE measurement;
