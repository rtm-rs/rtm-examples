--
-- NoGlobalStreamEntries : 20210103114314
--

BEGIN;

ALTER TABLE "event_store_events" RENAME COLUMN "id" TO "event_id";

ALTER TABLE "event_store_events" ALTER COLUMN "event_id" DROP DEFAULT;

ALTER TABLE "event_store_events" ADD "id" serial;

UPDATE event_store_events
  SET id = event_store_events_in_streams.id
  FROM event_store_events_in_streams
  WHERE event_store_events.event_id = event_store_events_in_streams.event_id AND event_store_events_in_streams.stream = 'all';
UPDATE event_store_events
  SET id = ese.new_id
  FROM (SELECT id, row_number() OVER (ORDER BY id) AS new_id FROM event_store_events) ese
  WHERE event_store_events.id = ese.id;
SELECT setval(pg_get_serial_sequence('event_store_events', 'id'), max(id)) FROM event_store_events;
ALTER TABLE event_store_events DROP CONSTRAINT event_store_events_pkey;
ALTER TABLE event_store_events ADD PRIMARY KEY (id);

CREATE UNIQUE INDEX "index_event_store_events_on_event_id" ON "event_store_events" ("event_id");

INSERT INTO schema_migrations (version) VALUES ('20210103114314');
