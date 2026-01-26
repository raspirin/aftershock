use std::env;

use aftershock_storage::{
    migration::run_migrations,
    routes::{
        create_content, delete_content_by_uid, delete_post, get_all_pages, get_all_pages_meta,
        get_all_posts, get_all_posts_meta, get_content_by_uid, get_post, get_published_pages,
        get_published_posts, get_published_posts_meta, update_content_by_uid, update_post,
    },
};
use axum::{
    Router,
    routing::get,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("AFTERSHOCK_DB_PORT").expect("AFTERSHOCK_DB_PORT is expected");
    let addr = format!("0.0.0.0:{port}");
    run_migrations().expect("Fail to run migrations");

    let app = Router::new()
        .route(
            "/api/v1/posts",
            get(get_published_posts).post(create_content),
        )
        .route("/api/v1/posts/all", get(get_all_posts))
        .route("/api/v1/posts/meta", get(get_published_posts_meta))
        .route("/api/v1/posts/all-meta", get(get_all_posts_meta))
        .route(
            "/api/v1/posts/{post_id}",
            get(get_post).put(update_post).delete(delete_post),
        )
        .route(
            "/api/v1/posts/uid/{post_uid}",
            get(get_content_by_uid)
                .put(update_content_by_uid)
                .delete(delete_content_by_uid),
        )
        .route(
            "/api/v1/pages",
            get(get_published_pages).post(create_content),
        )
        .route("/api/v1/pages/all", get(get_all_pages))
        .route("/api/v1/pages/meta", get(get_published_posts_meta))
        .route("/api/v1/pages/all-meta", get(get_all_pages_meta))
        .route(
            "/api/v1/pages/uid/{post_uid}",
            get(get_content_by_uid)
                .put(update_content_by_uid)
                .delete(delete_content_by_uid),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
