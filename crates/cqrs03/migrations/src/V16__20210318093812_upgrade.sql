--
-- DisablePgCrypto : 20210318093812
--

BEGIN;

DROP EXTENSION IF EXISTS "pg_crypto" CASCADE;

INSERT INTO schema_migrations (version) VALUES ('20210318093812');
