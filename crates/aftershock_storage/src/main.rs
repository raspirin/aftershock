use aftershock_storage::get_posts;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1/posts", get(get_posts));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
