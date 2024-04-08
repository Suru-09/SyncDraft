pub mod mongo_wrap {

use mongodb::{options::ClientOptions, results::{InsertManyResult, InsertOneResult}, Client, Database};
use std::env;

use crate::doc::doc::Document;

pub struct MongoWrap {
    pub mongodb_uri: String,
}

impl MongoWrap {
    pub async fn new() -> Self {
        let mongodb_uri = env::var("MONGODB_URI")
            .expect("You must set the MONGODB_URI environment var!");

        Self {
            mongodb_uri: mongodb_uri,
        }
    }

    pub async fn insert_doc(&self, doc: Document, db_name: String, collection_name: String) -> Result<InsertOneResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.insert_one(doc, None).await
    }

    pub async fn insert_docs(&self, docs: Vec<Document>, db_name: String, collection_name: String) -> Result<InsertManyResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.insert_many(docs, None).await
    }

}

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use mongo_wrap::MongoWrap;
    use crate::doc::doc::Document;

    static DB_NAME: &str = "collabDocs";
    static COLLECTION_NAME: &str = "collab";

    #[test]
    async fn test_insert_doc() {
        // Arrange
        let mongo_wrap = MongoWrap::new().await;
        let db_name = DB_NAME.to_string();
        let collection_name = COLLECTION_NAME.to_string();
        let test_doc = Document {
            _id: "1".to_string(),
            body: vec![vec!['a', 'b', 'c']],
        };

        // Act
        let result = mongo_wrap.insert_doc(test_doc.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    async fn test_insert_docs() {
        // Arrange
        let mongo_wrap = MongoWrap::new().await;
        let db_name = DB_NAME.to_string();
        let collection_name = COLLECTION_NAME.to_string();
        let test_docs = vec![
            Document {
                _id: "1".to_string(),
                body: vec![vec!['a', 'b', 'c']],
            },
            Document {
                _id: "2".to_string(),
                body: vec![vec!['d', 'e', 'f']],
            },
        ];

        // Act
        let result = mongo_wrap.insert_docs(test_docs.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        // Assert
        assert!(result.is_ok());
    }

}
