-- This file should undo anything in `up.sql`
ALTER TABLE posts 
DROP COLUMN IF EXISTS create_time;