--
-- ProductsAddRegisteredAt : 20210918000940
--

BEGIN;

ALTER TABLE "products" ADD "registered_at" timestamp;

INSERT INTO schema_migrations (version) VALUES ('20210918000940');
