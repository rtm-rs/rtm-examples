--
-- CreateOrderLines : 20150429224621
--

BEGIN;

CREATE TABLE "order_lines" ("id" bigserial primary key, "order_uid" character varying, "product_id" integer, "product_name" character varying, "quantity" integer);

INSERT INTO schema_migrations (version) VALUES ('20150429224621');
