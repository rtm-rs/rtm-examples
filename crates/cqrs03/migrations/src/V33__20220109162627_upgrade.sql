--
-- CreateInvoiceReadModel : 20220109162627
--

BEGIN;

CREATE TABLE "invoices" ("id" bigserial primary key, "order_uid" character varying NOT NULL, "number" character varying, "tax_id_number" character varying, "address_line_1" character varying, "address_line_2" character varying, "address_line_3" character varying, "address_line_4" character varying, "address_present" boolean DEFAULT FALSE, "issued" boolean DEFAULT FALSE, "issue_date" date, "disposal_date" date, "payment_date" date, "total_value" decimal(8,2));

CREATE TABLE "invoice_items" ("id" bigserial primary key, "invoice_id" bigint, "name" character varying, "unit_price" decimal(8,2), "vat_rate" decimal(4,1), "quantity" integer, "value" decimal(8,2));

CREATE INDEX "index_invoice_items_on_invoice_id" ON "invoice_items" ("invoice_id");

INSERT INTO schema_migrations (version) VALUES ('20220109162627');
