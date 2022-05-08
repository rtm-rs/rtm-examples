#[derive(rtm::Command)]
struct AddPost {
  author: String,
  title: String,
  content: String
}

// Implements:
// impl rtm::Command for AddPost {}

// `validates_presence_of` may not be required due to Rust properties/design?
AddPost::validates_presence_of!(fields = []);

#[derive(rtm::Command)]
struct PublishPost {
    publication_date: DateTime
}

impl rtm::Command for PublishPost {}

PublishPost::validates_presence_of!(field=publication_date);
