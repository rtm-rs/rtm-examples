--
-- IndexByEventType : 20181123154324
--

BEGIN;

CREATE INDEX "index_event_store_events_on_event_type" ON "event_store_events" ("event_type");

INSERT INTO schema_migrations (version) VALUES ('20181123154324');
