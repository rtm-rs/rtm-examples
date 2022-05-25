#[derive(Aggregate)]
#[rtm(relation = [Order])]
struct Reservation {
    state: State,
    order_id: Infra::Types::UUID,
    reservation_items: Infra::Types::UUIDQuantityHash,
}

enum State {
    Submitted,
    Completed,
    Cancelled,
    None,
}

struct ReservationItem {
    product_id: String,
    quantity: usize,
}

type AlreadySubmitted = Error;
type AlreadyCompleted = Error;
type AlreadyCanceled = Error;
type NotSubmitted = Error;

impl Reservation {
    fn new(order: Order) {
        Self {
            state: None,
            order_id: order.id,
            reservation_items: ReservationItem = Vec::new(),
        }
    }

}
