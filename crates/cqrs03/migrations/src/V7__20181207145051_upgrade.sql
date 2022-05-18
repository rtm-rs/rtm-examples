--
-- BinaryDataAndMetadata : 20181207145051
--

BEGIN;

ALTER TABLE event_store_events ALTER COLUMN data     TYPE bytea USING convert_to(data,     'UTF8');
        ALTER TABLE event_store_events ALTER COLUMN metadata TYPE bytea USING convert_to(metadata, 'UTF8');

INSERT INTO schema_migrations (version) VALUES ('20181207145051');
