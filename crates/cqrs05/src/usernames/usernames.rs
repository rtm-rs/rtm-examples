struct UsernameAlreadyRegistered;
impl Error for UsernameAlreadyRegistered {}

#[derive(rtm::Aggregate)]
struct Usernames {
    usernames: HashMap,
}

impl Usernames {
    fn new(id: String) {
        Self {
            usernames: HashMap::new(Self, id),
        }
    }

    fn instance(id: String) -> Self {
        // Implementing `Repository`:
        // https://stackoverflow.com/a/66679536
        match rtm::Ephemera::load(Self) {
            Ok(i) => i,
            Err(e) => {
                usernames = Usernames::new(id);
                rtm::Ephemera::add(usernames);
                usernames
            }
        }
    }

    fn initialize(id: String) {
        rtm::Ephemera::initialize(id);
        apply(UsernamesCreated)
    }

    fn add(username: String) {
        apply(UsernameAdded, username)
    }
}

Usernames::on!(|event: UsernamesCreated| { Self.usernames = event });
Usernames::on!(|event: UsernameAdded| { Self.usernames << event.username });
