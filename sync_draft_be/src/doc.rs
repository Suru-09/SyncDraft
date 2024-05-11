pub mod doc {
    use axum::{Json, extract::Query};
    use hyper::StatusCode;
    use serde::{Deserialize, Serialize};
    use mongodb::bson::{Uuid, doc};
    use crate::mongo_wrapper::mongo_wrap::MongoWrap;

    static DB_NAME: &str = "collabDocs";
    static COLLECTION_NAME: &str = "collab";

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Document {
        pub _id: Uuid,
        pub doc_name: String,
        pub doc_owner: String,
        pub body: String,
    }

    impl Document {
        pub async fn create_doc(Json(payload): Json<CreateDoc>) -> (StatusCode, Json<Document>) {
            let uuid = Uuid::new();
            let doc = Document {
                _id: uuid,
                doc_name: payload.doc_name,
                doc_owner: payload.doc_owner,
                body: payload.body
            };

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();
            
            let result = mongo_wrap.insert(doc.clone(), db_name, collection_name).await;
            match result {
                Ok(_) => (StatusCode::CREATED, Json(doc)),
                Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(doc))
            }
        }

        pub async fn update_doc(Json(payload): Json<UpdateDoc>) -> (StatusCode, Json<Document>) {
            let doc = Document {
                _id: payload._id,
                doc_name: payload.doc_name,
                doc_owner: payload.doc_owner,
                body: payload.body
            };

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();
            
            let result = mongo_wrap.update_doc(doc.clone(), db_name, collection_name).await;
            match result {
                Ok(_) => (StatusCode::CREATED, Json(doc)),
                Err(error) => {
                    println!("Error when updating document: {:?}", error);
                    (StatusCode::UNPROCESSABLE_ENTITY, Json(doc)) 
                }

            }
        }

        pub async fn no_of_docs_for_user(Json(payload): Json<GetDocs>) -> (StatusCode, Json<DocsNumber>) {
            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            let result: Result<u64, mongodb::error::Error> = mongo_wrap.number_of_docs_for_user(payload.doc_owner, db_name, collection_name).await;
            match result {
                Ok(no_docs) => (StatusCode::CREATED, Json(DocsNumber{number: no_docs})),
                Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(DocsNumber{number: 0}))
            }
        }

        pub async fn delete_doc(Json(payload): Json<DeleteDoc>) -> StatusCode {
            let uuid = payload._id;

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            let result = mongo_wrap.delete_after_id(uuid, db_name, collection_name).await;
            match result {
                Ok(_) => StatusCode::CREATED,
                Err(_) => StatusCode::UNPROCESSABLE_ENTITY
            }
        }

        pub async fn get_docs_for_user(Query(payload): Query<GetDocs>) -> (StatusCode, Json<Vec<Document>>) {
            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            println!("{:?}", payload.doc_owner);

            let result = mongo_wrap.get_docs_for_user(payload.doc_owner, db_name, collection_name).await;
            match result {
                Ok(docs) => (StatusCode::OK, Json(docs)),
                Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(vec![]))
            }
        }
            
    }


    #[derive(Serialize, Deserialize)]
    pub struct CreateDoc {
        doc_name: String,
        doc_owner: String,
        body: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UpdateDoc {
        _id: Uuid,
        doc_name: String,
        doc_owner: String,
        body: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DeleteDoc {
        _id: Uuid,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetDocs {
        doc_owner: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct DocsNumber {
        number: u64,
    }
}