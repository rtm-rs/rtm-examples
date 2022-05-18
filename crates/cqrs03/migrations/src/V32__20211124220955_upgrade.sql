--
-- AddVatRateCodeToProduct : 20211124220955
--

BEGIN;

ALTER TABLE "products" ADD "vat_rate_code" character varying;

INSERT INTO schema_migrations (version) VALUES ('20211124220955');
