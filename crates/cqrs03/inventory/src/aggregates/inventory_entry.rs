#[derive(Aggregate)]
#[rtm()]
struct InventoryEntry;

type InventoryNotAvailable = anyhow::Error;
type StockLevelUndefined = anyhow::Error;
type NotEvenReserved = anyhow::Error;

fn new(product_id: ProductId) {
    Self {
        product_id,
        reserved = 0,
    }
}

fn supply(&self, quantity: usize) {
    // Impl `apply` from the `Aggregate` trait in the `eventually` crate.
    let data  = Self {
            product_id: self.product_id,
            quantity: quantity,
            stock_level: (self.in_stock || 0) + quantity
            };
    apply(StockLevelUpdated::new(data))
    }

fn dispatch(&self, quantity: usize) {
    // Impl `apply` from the `Aggregate` trait in the `eventually` crate.
    let data = Self {
            product_id: self.product_id,
            quantity: -quantity,
            stock_level: self.in_stock - quantity
            };
    apply(StockLevelUpdated::new(data))
    }

fn reserve(&self, quantity: usize) {
    if is_stock_level_defined() {
    is_available(quantity);
    let data = Self {
            product_id: self.product_id,
            quantity: quantity
            };
    apply(StockReserved::new(data))
} else {
        return Err(StockLevelUndefined)
    }

    }

fn release(quantity: usize) {
    let data = Self {
            product_id: self.product_id,
            quantity
            };
    apply(StockReleased::new(data))
    }

fn is_available(&self, desired_quantity: usize) {
    match is_stock_level_defined(){
    false => { return Ok(())},
    true if desired_quantity > self.availability() => {
        return Err(InventoryNotAvailable)
    },
    }

}

    //private
    //
    // The class method on(event_klass, &method) is an alternative for defining
    // methods by the convention of `apply_` + underscored event type
    // (`event.<event_type>` that with `RubyEventStore::Event` is equal to event's
    // class name) for event handler methods. That is, when you `apply` the
    // `OrderExpired` event, the `apply_order_expired` method is called.

on!(StockLevelUpdated, |event: Event| {
    self.in_stock = event.data.fetch("stock_level")
});

on!(StockReserved, |event: Event| {
    self.reserved += event.data.fetch("quantity")
});

on!( StockReleased, |event: Event|{
    self.reserved -= event.data.fetch("quantity")
});

fn availability(&self) {
    self.in_stock - self.reserved
}

fn is_stock_level_defined() {
    self.in_stock.is_some()
}
