--
-- AddUidToCustomer : 20210626171021
--

BEGIN;

ALTER TABLE "customers" ADD "uid" uuid;

INSERT INTO schema_migrations (version) VALUES ('20210626171021');
