--
-- CreateClients : 20220219130650
--

BEGIN;

CREATE TABLE "clients" ("id" bigserial primary key, "uid" uuid, "name" character varying, "created_at" timestamp(6) NOT NULL, "updated_at" timestamp(6) NOT NULL);

INSERT INTO schema_migrations (version) VALUES ('20220219130650');
