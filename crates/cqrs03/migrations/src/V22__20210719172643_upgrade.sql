--
-- AddDiscountAndTotalValueAndDiscountValueToOrders : 20210719172643
--

BEGIN;

ALTER TABLE "orders" ADD "percentage_discount" decimal(8,2);

ALTER TABLE "orders" ADD "total_value" decimal(8,2);

ALTER TABLE "orders" ADD "discounted_value" decimal(8,2);

INSERT INTO schema_migrations (version) VALUES ('20210719172643');
