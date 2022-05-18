--
-- AddVipToCustomer : 20220121131334
--

BEGIN;

ALTER TABLE "customers" ADD "vip" boolean DEFAULT FALSE NOT NULL;

INSERT INTO schema_migrations (version) VALUES ('20220121131334');
