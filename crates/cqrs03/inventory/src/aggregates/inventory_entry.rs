#[derive(Aggregate)]
#[rtm(derive=true, repository=inventory_entry)]
// No direct data-store entity for this type - relates to the Products entity.
// Merges fields in upstream InventoryEntry and CheckAvailability
struct InventoryEntry {
    product_id: infra::types::Uuid,
    desired_quantity: infra::types::Integer,
    quantity: infra::types::Integer,
    reserved: infra::types::Integer,
    in_stock: infra::types::Integer,
    stock_level: infra::types::Integer,
}

// Macro Implements
struct InventoryEntry {
    repository: eventually::repository::Repository<InventoryEntry, InventoryRoot>,
    product_id: infra::types::UUID,
    desired_quantity: infra::types::Integer,
    reserved: infra::types::Integer,
}

impl eventually::aggregate::Aggregate for InventoryEntry {
    type Id = InventoryEntryId;
    type Event = crate::inventory::events::inventory_entry::Event;
    type Error = InventoryEntryError;

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        //The user implementation goes here.
    }
}

// User responsibility:

fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    todo!()
}

type InventoryNotAvailable = anyhow::Error;
type InventoryEntryUndefined = anyhow::Error;
type NotEvenReserved = anyhow::Error;

// Generated using Default::default() if `new` not implemented.
fn new(product_id: ProductId) {
    Self {
        product_id,
        desired_quantity: 0,
        quantity: 0,
        reserved: 0,
        in_stock: 0,
        stock_level: 0,
    }
}

fn supply(&self, quantity: usize) {
    // Impl `apply` from the `Aggregate` trait in the `eventually` crate.
    let data = Self {
        product_id: self.product_id,
        quantity: quantity,
        stock_level: self.in_stock + quantity,
    };
    apply(data, Event::Updated)
}

fn dispatch(&self, quantity: usize) {
    // Impl `apply` from the `Aggregate` trait in the `eventually` crate.
    let data = Self {
        product_id: self.product_id,
        quantity: -self.quantity,
        stock_level: self.in_stock - self.quantity,
        ..
    };
    apply(data, Event::Updated)
}

fn reserve(&self, quantity: usize) {
    if is_stock_level_defined() {
        is_available(quantity);
        let data = Self {
            product_id: self.product_id,
            quantity: quantity,
            // Was on!(StockReserved, |event: Event| { self.reserved += event.data.fetch("quantity") });
            reserved: self.reserved + quantity
            ..
        };
        apply(StockReserved::new(data))
    } else {
        return Err(InventoryEntryUndefined);
    }
}

fn release(quantity: usize) {
    let data = Self {
        product_id: self.product_id,
        quantity,
    };
    apply(StockReleased::new(data))
}

fn is_available(&self, desired_quantity: usize) {
    match is_stock_level_defined() {
        false => return Ok(()),
        true if desired_quantity > self.availability() => return Err(InventoryNotAvailable),
    }
}

//private
//
// The class method on(event_klass, &method) is an alternative for defining
// methods by the convention of `apply_` + underscored event type
// (`event.<event_type>` that with `RubyEventStore::Event` is equal to event's
// class name) for event handler methods. That is, when you `apply` the
// `OrderExpired` event, the `apply_order_expired` method is called.

on!(InventoryEntryUpdated, |event: Event| {
    self.in_stock = event.data.fetch("stock_level")
});

on!(StockReserved, |event: Event| {
    self.reserved += event.data.fetch("quantity")
});

on!(StockReleased, |event: Event| {
    self.reserved -= event.data.fetch("quantity")
});

fn availability(&self) {
    self.in_stock - self.reserved
}

fn is_stock_level_defined() {
    self.in_stock.is_some()
}
