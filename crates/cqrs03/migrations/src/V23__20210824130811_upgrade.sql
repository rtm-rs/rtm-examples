--
-- AddStockLevelToProduct : 20210824130811
--

BEGIN;

ALTER TABLE "products" ADD "stock_level" integer;

INSERT INTO schema_migrations (version) VALUES ('20210824130811');
