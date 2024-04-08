pub mod doc {
    use serde::{Deserialize, Serialize};


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Document {
        pub _id: String,
        pub body: Vec<Vec<char>>,
    }
}