--
-- CreateEventStoreEvents : 20181102132612
--

BEGIN;

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE "event_store_events_in_streams" ("id" serial NOT NULL PRIMARY KEY, "stream" character varying NOT NULL, "position" integer, "event_id" uuid NOT NULL, "created_at" timestamp NOT NULL);

CREATE UNIQUE INDEX "index_event_store_events_in_streams_on_stream_and_position" ON "event_store_events_in_streams" ("stream", "position");

CREATE INDEX "index_event_store_events_in_streams_on_created_at" ON "event_store_events_in_streams" ("created_at");

CREATE UNIQUE INDEX "index_event_store_events_in_streams_on_stream_and_event_id" ON "event_store_events_in_streams" ("stream", "event_id");

CREATE TABLE "event_store_events" ("id" uuid DEFAULT gen_random_uuid() NOT NULL PRIMARY KEY, "event_type" character varying NOT NULL, "metadata" text, "data" text NOT NULL, "created_at" timestamp NOT NULL);

CREATE INDEX "index_event_store_events_on_created_at" ON "event_store_events" ("created_at");

INSERT INTO schema_migrations (version) VALUES ('20181102132612');
