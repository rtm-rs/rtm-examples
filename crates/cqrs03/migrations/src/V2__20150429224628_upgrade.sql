--
-- CreateCustomers : 20150429224628
--

BEGIN;

CREATE TABLE "customers" ("id" bigserial primary key, "name" character varying, "created_at" timestamp NOT NULL, "updated_at" timestamp NOT NULL);

INSERT INTO schema_migrations (version) VALUES ('20150429224628');
