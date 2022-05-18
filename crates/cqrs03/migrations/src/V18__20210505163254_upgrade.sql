--
-- AddPriceToOrderLines : 20210505163254
--

BEGIN;

ALTER TABLE "order_lines" ADD "price" decimal(8,2);

INSERT INTO schema_migrations (version) VALUES ('20210505163254');
