--
-- ProductsDropTimestamps : 20210918003625
--

BEGIN;

ALTER TABLE "products" DROP COLUMN "created_at";

ALTER TABLE "products" DROP COLUMN "updated_at";

INSERT INTO schema_migrations (version) VALUES ('20210918003625');
