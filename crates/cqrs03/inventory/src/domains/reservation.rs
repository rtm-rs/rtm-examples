#[derive(Domain)]
#[rtm(command=Reservation)]
// add to struct: command, repository
struct Reservation{
    repository: infra::Repository,
}

fn new(event_store: EventStore){
    Self { repository: infra::Repository::new(event_store) }
}


fn submit_reservation(command: commands::Reservation) {
    with_reservation(command.order_id, |reservation| {
        reserved_items = {};
        command.reservation_items.each( |product_id, quantity| {
            with_inventory_entry(product_id, |entry| {
                match entry.reserve(quantity){
                    InventoryEntry::StockLevelUndefined => todo!(),
                    _ => reserved_items[product_id] = quantity
                }
            })
        });
        reservation.create(reserved_items)
    })
}

fn complete_reservation(command: commands::Reservation){
    with_reservation(command.order_id, |reservation| {
        reservation.reservation_items.map( |item| {
            with_inventory_entry(item.product_id, |entry| {
                entry.release(item.quantity);
                entry.dispatch(item.quantity);
            })
        });
        reservation.update();
    })
}

fn cancel_reservation(command: commands::Reservation){
    with_reservation(command.order_id, |reservation| {
        if reservation.is_submitted {
            reservation.reservation_items.map( |item|{
                with_inventory_entry(item.product_id, |entry|{
                    entry.release(item.quantity)
                })
            });
            reservation.delete()
        }
    })
}

fn with_reservation(&self, order_id: OrderId){
    self.repository.with_aggregate(Reservation, order_id, |reservation| {
        yield(reservation)
    })
}

fn with_inventory_entry(&self, product_id: OrderId){
    self.repository.with_aggregate(InventoryEntry, product_id, |entry| {
        yield(entry)
    })
}
