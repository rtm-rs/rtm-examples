--
-- AddPriceToProducts : 20210505162028
--

BEGIN;

ALTER TABLE "products" ADD "price" decimal(8,2);

INSERT INTO schema_migrations (version) VALUES ('20210505162028');
