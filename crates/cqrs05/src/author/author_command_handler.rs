// Command handlers are redundant in RTM, we show them for completeness
// and because this is a PoC exercise
#[derive(rtm::CommandHandler)]
struct AuthorCommandHandler;

AuthorCommandHandler::on!(|command: AddAuthor| {
    Usernames.instance.add(command.email);
    rtm::Ephemera::add(Author::new(command));
});
