#[derive(rtm::Relation)]
#[rtm(adapter = sql, derive = true, crate = entities)] // Defaults so could be omitted
struct Products;

// implements
use entities::products::ActiveModel as ProductsMut;
use entities::products::Model as Products;

