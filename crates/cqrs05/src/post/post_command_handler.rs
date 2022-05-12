// Command handlers are redundant in RTM, we show them for completeness
// and because this is a PoC exercise
#[derive(rtm::Handler)]
struct PostCommandHandler;

// Subscribe to post commands
PostCommandHandler::on!(|command: AddPost| { rtm::Ephemera::add(Post::new(command)) });

// Subscribe to post commands
PostCommandHandler::on!(|command: PublishPost| {
    do_with!(|command: PublishPost, post: Post| { post.publish(command.publication_date) })
});
