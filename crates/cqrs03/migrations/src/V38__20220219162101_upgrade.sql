--
-- CreateClientOrders : 20220219162101
--

BEGIN;

CREATE TABLE "client_orders" ("id" bigserial primary key, "client_uid" uuid, "number" character varying, "order_uid" uuid, "state" character varying, "created_at" timestamp(6) NOT NULL, "updated_at" timestamp(6) NOT NULL);

INSERT INTO schema_migrations (version) VALUES ('20220219162101');
