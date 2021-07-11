-- Your SQL goes here
ALTER TABLE posts
ADD COLUMN create_time timestamp NOT NULL default NOW();