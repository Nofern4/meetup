-- This file should undo anything in `up.sql`
ALTER TABLE brawlers
DROP CONSTRAINT IF EXISTS unique_username;