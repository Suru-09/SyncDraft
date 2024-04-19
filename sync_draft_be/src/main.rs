use axum::{
    routing::{get, post}, Router
};

mod crdt;
mod mongo_wrapper;
mod doc;

use doc::doc::Document;

#[tokio::main]
async fn main() {
    // init mongodb connection.
    mongo_wrapper::mongo_wrap::MongoWrap::new().await;
    // initialize tracing.
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(Document::create_doc));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

