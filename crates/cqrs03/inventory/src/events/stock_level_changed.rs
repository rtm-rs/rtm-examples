#[derive(Event)]
#[rtm(commands=[Update])]
enum StockLevel {}

// implements
// type StockLevelCreated = aggregates::StockLevel;
// type StockLevelDeleted = aggregates::StockLevel;
type StockLevelUpdated = aggregates::StockLevel;

enum StockLevel {
    // Created(StockLevelCreated),
    // Deleted(StockLevelDeleted),
    Updated(StockLevelUpdated),
}
