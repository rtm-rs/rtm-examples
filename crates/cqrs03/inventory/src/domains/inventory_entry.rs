#[derive(Domain)]
struct InventoryEntry;

    fn new(&mut self, event_store: EventStore) {
      self.repository = rtm::AggregateRootRepository::new(event_store)
    }

    fn supply(command: impl Command) {
      with_inventory_entry(command.product_id, |entry|{
        entry.supply(command.quantity);
      })
    }

    fn check_availability(command: impl Command) {
      with_inventory_entry(command.product_id, |entry| {
        //entry.check_availability!(command.desired_quantity);
        entry.is_available(command.desired_quantity)
      })
    }

    fn with_inventory_entry(product_id: ProductId) {
      self.repository.with_aggregate(InventoryEntry, product_id, |entry| {
        yield(entry)
      })
    }
