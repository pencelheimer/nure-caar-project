-- migrate:up

ALTER TABLE alert ADD COLUMN value DOUBLE PRECISION NOT NULL DEFAULT 0;

-- migrate:down

ALTER TABLE alert DROP COLUMN value;
