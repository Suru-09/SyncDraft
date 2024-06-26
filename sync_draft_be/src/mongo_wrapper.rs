pub mod mongo_wrap {

use futures::{TryFutureExt, TryStreamExt};
use mongodb::{bson::{doc, to_bson, to_document, Uuid}, options::SelectionCriteria, results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult}, Client, Collection};
use serde::Serialize;
use std::env;
use pwhash::bcrypt;

use crate::{doc::doc::Document, user::user::{LoginUser, User}};

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

    pub async fn insert<T: Serialize>(&self, to_insert: T, db_name: String, collection_name: String) -> Result<InsertOneResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<T>(&collection_name);

        collection.insert_one(to_insert, None).await
    }

    pub async fn update_doc(&self, to_update: Document, db_name: String, collection_name: String) -> Result<UpdateResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection: Collection<Document> = db.collection::<Document>(&collection_name);

        let uuid = to_bson(&to_update._id.clone()).unwrap();
        let document = doc! {
            "$set": to_document(&to_update).unwrap()
        };
        
        println!("Document is : {:?}", document.clone());

        collection.update_one(doc! {
            "_id": to_bson(&uuid).unwrap()
        }, document, None).await
    }

    pub async fn insert_many<T: Serialize>(&self, to_insert_vec: Vec<T>, db_name: String, collection_name: String) -> Result<InsertManyResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<T>(&collection_name);

        collection.insert_many(to_insert_vec, None).await
    }

    pub async fn delete_after_id(&self, uuid: Uuid, db_name: String, collection_name: String) -> Result<DeleteResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.delete_one(doc! {
            "_id": uuid
        },
        None).await
    }
    
    pub async fn number_of_docs_for_user(&self, doc_owner: String, db_name: String, collection_name: String) -> Result<u64, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        collection.count_documents(doc! {
            "doc_owner": doc_owner
        },
        None).await
    }

    pub async fn delete_many_after_id(&self, uuids: Vec<Uuid>, db_name: String, collection_name: String) -> Result<DeleteResult, mongodb::error::Error> {
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

    pub async fn user_exists(&self, user: LoginUser, db_name: String, collection_name: String) -> Result<User, mongodb::error::Error> {
        println!("HERE {}", self.mongodb_uri);
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<User>(&collection_name);
        println!("A{:?}", client);

        let filter = doc! {
            "username": user.username,
        };

        let result = collection.find_one(filter, None).await;
        match result {
            Ok(option_user) => {
                println!("{:?}", option_user);
                let db_user = option_user.unwrap();
                let verify = bcrypt::verify(user.password.clone(), &db_user.password);
                if verify == true
                {
                    return Ok(db_user)
                }
                else {
                    return Err(mongodb::error::Error::custom("WRONG_PASSWORD"))
                }
            },
            Err(str) => Err(str)
        }
    }

    pub async fn delete_user(&self, user: LoginUser, db_name: String, collection_name: String) -> Result<DeleteResult, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<User>(&collection_name);

        let filter = doc! {
            "username": user.username,
            "password": bcrypt::hash(user.password).unwrap()
        };

        collection.delete_one(filter, None).await
    }

    pub async fn get_docs_for_user(&self, doc_owner: String, db_name: String, collection_name: String) -> Result<Vec<Document>, mongodb::error::Error> {
        let client = Client::with_uri_str(self.mongodb_uri.clone()).await?;
        let db = client.database(&db_name);
        let collection = db.collection::<Document>(&collection_name);

        let mut cursor = match collection.find(doc! {"doc_owner": doc_owner}, None).await {
            Ok(curs) => curs,
            Err(er) => {
                println!("Opening cursor failed with: {:?}", er);
                return Err(er)
            }
        };

        let mut docs = vec![];
        docs.push(cursor.deserialize_current().unwrap());
        while let Some(doc) = cursor.try_next().await? {
            docs.push(doc);
        }
        println!("{:?}", docs);

        Ok(docs)
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
    async fn test_insert() {
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
        let result = mongo_wrap.insert(test_doc.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        let delete_result = mongo_wrap.delete_after_id(uuid, db_name, collection_name).await;
        println!("{:?}", delete_result);

        // Assert
        assert!(result.is_ok());
        assert!(delete_result.is_ok());
    }

    #[test]
    async fn test_inserts() {
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
        let result = mongo_wrap.insert_many(test_docs.clone(), db_name.clone(), collection_name.clone()).await;
        println!("{:?}", result);

        let uuid_vec = vec![uuid1.clone(), uuid2.clone()];
        let delete_result = mongo_wrap.delete_many_after_id(uuid_vec, db_name, collection_name).await;
        println!("{:?}", delete_result);

        // Assert
        assert!(result.is_ok());
    }

}
