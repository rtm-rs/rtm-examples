require "minitest/autorun"
require "mutant/minitest/coverage"

require_relative "../lib/inventory"

module Inventory
  class Test < Infra::InMemoryTest
    cover "Inventory*"

    def before_setup
      super
      Configuration.new.call(cqrs)
    end

    fn inventory_entry_stream(product_id) {
      "Inventory::InventoryEntry$#{product_id}"
    end

    fn reservation_stream(order_id) {
      "Inventory::Reservation$#{order_id}"
    end

    fn supply(product_id, quantity) {
      Supply.new(product_id: product_id, quantity: quantity)
    end

    fn submit_reservation(order_id, uuid_quantity_hash = {}) {
      SubmitReservation.new(order_id: order_id, reservation_items: uuid_quantity_hash)
    end

    fn cancel_reservation(order_id) {
      CancelReservation.new(order_id: order_id)
    end

    fn complete_reservation(order_id) {
      CompleteReservation.new(order_id: order_id)
    end
  end
end
