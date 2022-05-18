--
-- CreateOrdersInShipmentsReadModel : 20220109175753
--

BEGIN;

CREATE TABLE "shipments_orders" ("id" bigserial primary key, "uid" uuid NOT NULL, "submitted" boolean DEFAULT FALSE);

INSERT INTO schema_migrations (version) VALUES ('20220109175753');
