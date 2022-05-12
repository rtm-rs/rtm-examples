// In Sequent all business logic goes here - in RTM "repositories" are
// "aggregates" and no business logic goes there.  Rather, all business logic
// goes in implementation of the `Arbiter` trait - where relations and
// aggregates are prepared and contesting requirements/states are arbitrated

#[derive(AggregateRoot)] // Implements apply and initialize functions below
struct Post;

impl Post {
    fn new(command: impl Command) -> Return<impl AggregateError, _> {
        initialize(command.aggregate_id)?;
        apply(|command: PostAdded| command)?;
        apply(|command: PostAuthorChanged| Self { author_aggregate_id: command.author, ..})?;
        apply(|command: PostTitleChanged| Self { title: command.title, ..})?;
        apply(|command: PostContentChanged| Self { content: command.content, ..})?;
    }

    // This is the function the appears in `post_command_handler.rs`
    fn publish(publication_date: DateTime) -> Return<impl AggregateError, _> {
        if publication_date.any? {
            return Err(PostAlreadyPubishedError);
        }
        apply(PostPublished, publication_date)?
    }

    // ...
}
