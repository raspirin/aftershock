use aftershock_storage::{
    create_post, delete_post, get_all_posts, get_post, get_post_by_uid, get_published_posts, get_published_posts_meta, update_post
};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/posts", get(get_published_posts).post(create_post))
        .route("/api/v1/posts/all", get(get_all_posts))
        .route("/api/v1/posts/meta", get(get_published_posts_meta))
        .route(
            "/api/v1/posts/:post_id",
            get(get_post).put(update_post).delete(delete_post),
        )
        .route("/api/v1/posts/uid/:post_uid", get(get_post_by_uid));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
