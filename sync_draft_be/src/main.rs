use axum::{
    routing::{get, post}, Router
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod crdt;
mod mongo_wrapper;
mod doc;
mod user;

use doc::doc::Document;
use user::user::User;

#[tokio::main]
async fn main() {
    // init mongodb connection.
    mongo_wrapper::mongo_wrap::MongoWrap::new().await;
    // initialize tracing.
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/user/create", post(User::create_user))
        .route("/user/login", post(User::verify_user))
        .route("/user/delete", post(User::delete_user))
        .route("/doc/create", post(Document::create_doc))
        .route("/doc/delete", post(Document::delete_doc))
        .route("/doc/get", get(Document::get_docs_for_user))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

