-- Clean up duplicate usernames and delete all brawlers
DELETE FROM crew_memberships;
DELETE FROM missions;
DELETE FROM brawlers;

ALTER SEQUENCE brawlers_id_seq RESTART WITH 1;
