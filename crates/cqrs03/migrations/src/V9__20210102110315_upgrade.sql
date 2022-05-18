--
-- HarmonizeSchema : 20210102110315
--

BEGIN;

ALTER TABLE event_store_events ALTER COLUMN id TYPE uuid USING id::uuid;

ALTER TABLE event_store_events_in_streams ALTER COLUMN event_id TYPE uuid USING event_id::uuid;

INSERT INTO schema_migrations (version) VALUES ('20210102110315');
