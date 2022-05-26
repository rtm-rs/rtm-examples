// We can likely infer all this from the aggregate context
#[derive(Event)]
#[rtm(commands=[update])] // default is [create,delete,update]
enum InventoryEntry {}

// Implements:
//
// type InventoryEntryCreated = crate::aggregates::InventoryEntry;
// type InventoryEntryDeleted = crate::aggregates::InventoryEntry;
type InventoryEntryUpdated = crate::aggregates::InventoryEntry;

// this is in the namespace crate::inventory::events::inventory_entry::Event
enum Event {
    // Created(InventoryEntryCreated),
    // Deleted(InventoryEntryDeleted),
    Updated(InventoryEntryUpdated),
}
