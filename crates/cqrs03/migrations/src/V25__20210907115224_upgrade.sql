--
-- StringToNativeUuids : 20210907115224
--

BEGIN;

ALTER TABLE "orders" ALTER COLUMN "uid" TYPE uuid USING uid::uuid, ALTER COLUMN "uid" SET NOT NULL;

ALTER TABLE "order_lines" ALTER COLUMN "order_uid" TYPE uuid USING order_uid::uuid, ALTER COLUMN "order_uid" SET NOT NULL;

INSERT INTO schema_migrations (version) VALUES ('20210907115224');
