--
-- CreateCustomersInOrdersReadModel : 20211116154857
--

BEGIN;

CREATE TABLE "orders_customers" ("id" bigserial primary key, "uid" uuid NOT NULL, "name" character varying);

INSERT INTO schema_migrations (version) VALUES ('20211116154857');
