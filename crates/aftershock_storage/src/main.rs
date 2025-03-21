use aftershock_storage::{create_post, delete_post, get_all_posts, get_post, get_published_posts, update_post};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/posts", get(get_published_posts).post(create_post))
        .route("/api/v1/posts/all", get(get_all_posts))
        .route("/api/v1/posts/:post_id", get(get_post).put(update_post).delete(delete_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
