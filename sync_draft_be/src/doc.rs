pub mod doc {
    use serde::{Deserialize, Serialize};
    use mongodb::bson::Uuid;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Document {
        pub _id: Uuid,
        pub doc_name: String,
        pub doc_owner: String,
        pub body: String,
    }
}