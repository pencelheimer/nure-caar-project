-- pgt-ignore-all lint/safety/banDropColumn

-- migrate:up
ALTER TABLE "user" ADD COLUMN is_banned BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "user" ADD COLUMN ban_reason TEXT;


-- migrate:down
ALTER TABLE "user" DROP COLUMN ban_reason;
ALTER TABLE "user" DROP COLUMN is_banned;
