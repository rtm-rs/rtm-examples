--
-- MigrateFromIdToUuid : 20210626210851
--

BEGIN;

ALTER TABLE "products" DROP COLUMN "id";

ALTER TABLE "products" ALTER COLUMN "uid" SET DEFAULT gen_random_uuid();

ALTER TABLE "products" ALTER COLUMN "uid" SET NOT NULL;

ALTER TABLE "products" RENAME COLUMN "uid" TO "id";

ALTER TABLE products ADD PRIMARY KEY (id);

ALTER TABLE "customers" DROP COLUMN "id";

ALTER TABLE "customers" ALTER COLUMN "uid" SET DEFAULT gen_random_uuid();

ALTER TABLE "customers" ALTER COLUMN "uid" SET NOT NULL;

ALTER TABLE "customers" RENAME COLUMN "uid" TO "id";

ALTER TABLE customers ADD PRIMARY KEY (id);

ALTER TABLE "order_lines" DROP COLUMN "product_id";

ALTER TABLE "order_lines" ADD "product_id" uuid;

INSERT INTO schema_migrations (version) VALUES ('20210626210851');
