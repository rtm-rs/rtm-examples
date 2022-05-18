--
-- MigrateYamlSerializedEventsToJson : 20190125222757
--

BEGIN;

SELECT "event_store_events".* FROM "event_store_events";

INSERT INTO schema_migrations (version) VALUES ('20190125222757');
