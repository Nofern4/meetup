-- Your SQL goes here
ALTER TABLE brawlers
ADD display_name VARCHAR(50) NOT NULL DEFAULT '',
    avatar_url VARCHAR(512),
    avatar_public_id VARCHAR(255);