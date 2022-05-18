--
-- BinaryToJsonb : 20210102182818
--

BEGIN;

ALTER TABLE event_store_events ALTER COLUMN data     TYPE jsonb USING convert_from(data, 'UTF-8')::jsonb;

ALTER TABLE event_store_events ALTER COLUMN metadata TYPE jsonb USING convert_from(metadata, 'UTF-8')::jsonb;

INSERT INTO schema_migrations (version) VALUES ('20210102182818');
