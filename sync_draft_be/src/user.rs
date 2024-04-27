pub mod user {
    use axum::Json;
    use hyper::StatusCode;
    use serde::{Deserialize, Serialize};
    use crate::mongo_wrapper::mongo_wrap::MongoWrap;
    use pwhash::bcrypt;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        pub username: String, 
        pub password: String,
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct LoginUser {
        pub username: String, 
        pub password: String,
    }

    static DB_NAME: &str = "collabDocs";
    static COLLECTION_NAME: &str = "users";

    impl User {
        pub fn new() -> Self {
            User {
                username: "".to_string(),
                password: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string()
            }
        }
        pub async fn create_user(Json(payload): Json<User>) -> StatusCode {
            let hashed_pw =  match bcrypt::hash(payload.password) {
                Ok(h_pw) => h_pw,
                Err(_) => return StatusCode::FAILED_DEPENDENCY 
            };

            let user = User {
                username: payload.username,
                password: hashed_pw,
                first_name: payload.first_name,
                last_name: payload.last_name

            };

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            let result = mongo_wrap.insert(user, db_name, collection_name).await;
            match result {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::EXPECTATION_FAILED
            }
        }

        pub async fn verify_user(Json(payload): Json<LoginUser>) -> (StatusCode, Json<User>) {
            let user = LoginUser {
                username: payload.username,
                password: payload.password,
            };

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            let result = mongo_wrap.user_exists(user, db_name, collection_name).await;
            match result {
                Ok(db_user) => (StatusCode::OK, Json(db_user)),
                Err(mongo_err) => {
                    if mongo_err.to_string() == "WRONG_PASSWORD".to_string() {
                        return (StatusCode::UNAUTHORIZED, Json(User::new()))
                    }
                    return (StatusCode::NOT_FOUND, Json(User::new()))
                }
            }
        }

        pub async fn delete_user(Json(payload): Json<User>) -> StatusCode {
            let hashed_pw =  match bcrypt::hash(payload.password) {
                Ok(h_pw) => h_pw,
                Err(_) => return StatusCode::FAILED_DEPENDENCY 
            };

            let user = LoginUser {
                username: payload.username,
                password: hashed_pw,
            };

            let mongo_wrap = MongoWrap::new().await;
            let db_name = DB_NAME.to_string();
            let collection_name = COLLECTION_NAME.to_string();

            let result = mongo_wrap.delete_user(user, db_name, collection_name).await;
            match result {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::NOT_FOUND
            }
        }
    }

}