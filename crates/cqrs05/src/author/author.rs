// Here the aggregate intermingles commands - in RTM the AggregateRoot only
// aggregates data from different relations and adapters.
#[derive(rtm::Aggregate)]
struct Author;

impl Author {
    fn initialize(command: impl rtm::Command) {
        rtm::AggregateRoot::initialize(command.aggregate_id);
        apply!(AuthorCreated);
        apply!(AuthorNameSet, name = command.name);
        apply!(AuthorEmailSet, email = command.email);
    }
}

Author::on!(|command: AuthorCreated|{});
