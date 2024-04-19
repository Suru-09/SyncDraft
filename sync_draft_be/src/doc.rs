pub mod doc {
    use axum::Json;
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
            
            let result = mongo_wrap.insert_doc(doc.clone(), db_name, collection_name).await;
            match result {
                Ok(_) => (StatusCode::CREATED, Json(doc)),
                Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(doc))
            }
        }

        // pub async fn update_doc(Json(payload): Json<UpdateDoc>) -> (StatusCode, Json<Document>) {

        //     let filter = doc! { "_id": payload._id };
        //     let update = 

        // }
    }


    #[derive(Serialize, Deserialize)]
    pub struct CreateDoc {
        doc_name: String,
        doc_owner: String,
        body: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UpdateDoc {
        _id: String,
        update_c: char,
    }


}