#[derive(rtm::Create)]
#[rtm(relation = Authors)]
// Authors fields are added to this struct
struct AddAuthor {
    name: String,
    email: String,
}

// If no function `add_author` is defined, we implement:
// impl rtm::Create for PublishPost {
//     fn add_author(self: Self){
//        self.create() // The default command from adapter for relation Authors
//     }
// }
