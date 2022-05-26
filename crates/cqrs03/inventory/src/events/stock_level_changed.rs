// Was `stock_level_changed.rb` moved to `./inventory_entry.rs`
// Events are defined by aggregates and commands.
// Events only apply to an aggregate, and there are only three commands
// that change the state of an aggregate: create, delete and update.
//
// module Inventory
//   class StockLevelChanged < Infra::Event
//   end
// end
