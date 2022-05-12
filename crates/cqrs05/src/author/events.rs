// Events handlers are redundant in RTM - they are implemented by the
// proc-macro - we show them for completeness and because this is a PoC exercise
#[derive(rtm::Event)]
struct AuthorCreated;

#[derive(rtm::Event)]
struct AuthorNameSet {
    name: String,
}

#[derive(rtm::Event)]
struct AuthorEmailSet {
    email: String,
}
