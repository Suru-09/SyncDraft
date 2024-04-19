pub mod mongo_wrap {

use mongodb::{bson::Uuid, bson::doc, results::{DeleteResult, InsertManyResult, InsertOneResult}, Client};
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

    pub async fn delete_doc(&self, uuid: Uuid, db_name: String, collection_name: String) -> Result<DeleteResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.delete_one(doc! {
            "_id": uuid
        },
        None).await
    }
    
    pub async fn delete_docs(&self, uuids: Vec<Uuid>, db_name: String, collection_name: String) -> Result<DeleteResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.delete_many(doc! {
            "_id": { 
                "$in": uuids
            }
        },
        None).await
    }

}

}


#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::Uuid;
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
        let uuid = Uuid::new();
        let test_doc = Document {
            _id: uuid.clone(),
            doc_owner: "".to_string(),
            doc_name: "".to_string(),
            body: "abc".to_string(),
        };

        // Act
        let result = mongo_wrap.insert_doc(test_doc.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        let delete_result = mongo_wrap.delete_doc(uuid, db_name, collection_name).await;
        println!("{:?}", delete_result);

        // Assert
        assert!(result.is_ok());
        assert!(delete_result.is_ok());
    }

    #[test]
    async fn test_insert_docs() {
        // Arrange
        let mongo_wrap = MongoWrap::new().await;
        let db_name = DB_NAME.to_string();
        let collection_name = COLLECTION_NAME.to_string();
        let uuid1 = Uuid::new();
        let uuid2 = Uuid::new();
        let test_docs = vec![
            Document {
                _id: uuid1,
                doc_owner: "".to_string(),
                doc_name: "".to_string(),
                body: "abc".to_string(),
            },
            Document {
                _id: uuid2,
                doc_owner: "".to_string(),
                doc_name: "".to_string(),
                body: "defg".to_string(),
            },
        ];

        // Act
        let result = mongo_wrap.insert_docs(test_docs.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        let uuid_vec = vec![uuid1.clone(), uuid2.clone()];
        let delete_result = mongo_wrap.delete_docs(uuid_vec, db_name, collection_name).await;
        println!("{:?}", delete_result);

        // Assert
        assert!(result.is_ok());
    }

}
