--
-- CreateProductsInOrdersReadModel : 20211116135113
--

BEGIN;

CREATE TABLE "orders_products" ("id" bigserial primary key, "uid" uuid NOT NULL, "name" character varying, "price" decimal(8,2));

INSERT INTO schema_migrations (version) VALUES ('20211116135113');
