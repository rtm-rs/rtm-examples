--
-- CustomersAddRegisteredAt : 20210916191138
--

BEGIN;

ALTER TABLE "customers" ADD "registered_at" timestamp;

INSERT INTO schema_migrations (version) VALUES ('20210916191138');
