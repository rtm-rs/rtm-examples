--
-- CreateOrdersInInvoicesReadModel : 20220109175702
--

BEGIN;

CREATE TABLE "invoices_orders" ("id" bigserial primary key, "uid" uuid NOT NULL, "submitted" boolean DEFAULT FALSE);

INSERT INTO schema_migrations (version) VALUES ('20220109175702');
