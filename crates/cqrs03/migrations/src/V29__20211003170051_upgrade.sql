--
-- CreateShipments : 20211003170051
--

BEGIN;

CREATE TABLE "shipments" ("id" bigserial primary key, "order_uid" character varying NOT NULL, "address_line_1" character varying, "address_line_2" character varying, "address_line_3" character varying, "address_line_4" character varying);

INSERT INTO schema_migrations (version) VALUES ('20211003170051');
