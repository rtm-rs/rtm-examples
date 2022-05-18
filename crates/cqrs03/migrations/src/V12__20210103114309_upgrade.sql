--
-- AddValidAt : 20210103114309
--

BEGIN;

ALTER TABLE "event_store_events" ADD "valid_at" timestamp;

CREATE INDEX "index_event_store_events_on_valid_at" ON "event_store_events" ("valid_at");

INSERT INTO schema_migrations (version) VALUES ('20210103114309');
