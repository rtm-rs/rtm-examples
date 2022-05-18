--
-- CreateProducts : 20150429224746
--

BEGIN;

CREATE TABLE "products" ("id" bigserial primary key, "name" character varying, "created_at" timestamp NOT NULL, "updated_at" timestamp NOT NULL);

INSERT INTO schema_migrations (version) VALUES ('20150429224746');
