--
-- AddUidToProduct : 20210617230722
--

BEGIN;

ALTER TABLE "products" ADD "uid" uuid;

INSERT INTO schema_migrations (version) VALUES ('20210617230722');
