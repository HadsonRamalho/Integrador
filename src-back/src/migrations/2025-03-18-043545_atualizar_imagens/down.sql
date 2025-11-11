-- This file should undo anything in `up.sql`
ALTER TABLE imagens ADD COLUMN bin bin BYTEA NOT NULL;