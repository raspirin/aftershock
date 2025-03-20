use aftershock_storage::{establish_connection, models::Content};
use axum::{routing::get, Router};
use diesel::prelude::*;


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
