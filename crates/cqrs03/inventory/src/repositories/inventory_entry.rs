// Was CheckAvailability Command in `commands/check_availability.rs`
// module Inventory
//   class CheckAvailability < Infra::Command
//     attribute :product_id, Infra::Types::UUID
//     attribute :desired_quantity, Infra::Types::Integer
//   end
// end
#[derive(Repository)]
#[rtm(relations=[products], derive=true)]
struct InventoryEntry;

// Implements
use crate::relations::Products as InventoryEntry;
use crate::relations::ProductsMut as InventoryEntryMut;
