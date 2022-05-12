// Use `validator` crate for validations
#[derive(rtm::Create)]
#[rtm(relation = Posts)] // Could also point to an aggregate or in ROM parlance a Repository
                         // By stipulating String rust ensures a value is provided - no validate required
struct AddPost {
    author: String,
    title: String,
    content: String,
}

// If no function `publish_post` is defined, we implement::
// impl rtm::Create for AddPost {
//     fn add_post(self: Self){
//        self.create() // This is the default command for the adapter used by the relation Posts
//     }
// }

#[derive(rtm::Create)]
#[rtm(relation = Posts)] // Posts fields are added to this struct
struct PublishPost;

// struct PublishPost{
//   publication_date: DateTime
// }
// // If no function `publish_post` is defined, we implement:
// impl rtm::Create for PublishPost {
//     fn publish_post(self: Self){
//        self.create() // This is the default command for the adapter used by the relation Posts
//     }
// }

#[derive(rtm::Create)]
#[rtm(relation = Posts)] // Posts fields are added to this struct
struct AddPost {
    author_aggregate_id: String,
    title: String,
    content: String,
}
