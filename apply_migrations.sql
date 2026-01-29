-- Drop existing objects to ensure a clean slate (idempotency)
DROP TABLE IF EXISTS crew_memberships CASCADE;
DROP TABLE IF EXISTS missions CASCADE;
DROP TABLE IF EXISTS brawlers CASCADE;
DROP FUNCTION IF EXISTS diesel_manage_updated_at CASCADE;
DROP FUNCTION IF EXISTS diesel_set_updated_at CASCADE;

CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE missions (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(255) NOT NULL,
    chief_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE brawlers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE crew_memberships (
    mission_id INTEGER NOT NULL,
    brawler_id INTEGER NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (mission_id, brawler_id)
);

ALTER TABLE missions ADD CONSTRAINT fk_chief FOREIGN KEY (chief_id) REFERENCES brawlers(id);
ALTER TABLE crew_memberships ADD CONSTRAINT fk_mission FOREIGN KEY (mission_id) REFERENCES missions(id);
ALTER TABLE crew_memberships ADD CONSTRAINT fk_brawler FOREIGN KEY (brawler_id) REFERENCES brawlers(id);

SELECT diesel_manage_updated_at('missions');
SELECT diesel_manage_updated_at('brawlers');

ALTER TABLE brawlers ADD CONSTRAINT unique_username UNIQUE (username);

-- Seed Data
INSERT INTO brawlers (username, "password") VALUES 
('ChiefCommander', '$argon2id$v=19$m=4096,t=3,p=1$ZnJlZWRvbQ$Up2Q7Q'); -- Password: 'password' (example hash)

INSERT INTO missions ("name", "description", "status", chief_id) 
SELECT 'Explore the Unknown', 'A daring mission to find new territories.', 'Open', id 
FROM brawlers WHERE username = 'ChiefCommander';

INSERT INTO missions ("name", "description", "status", chief_id) 
SELECT 'Defend the Base', 'Protect our headquarters from incoming threats.', 'InProgress', id 
FROM brawlers WHERE username = 'ChiefCommander';
