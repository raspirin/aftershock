use axum::Router;
use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod error;
pub mod migration;
mod models;
mod pool;
pub mod routes;
mod schema;
mod utils;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(get_connection_pool);

pub fn create_router() -> Router {
    use axum::routing::get;

    Router::new()
        .route(
            "/api/v1/posts",
            get(routes::api::get_published_posts).post(routes::api::create_content),
        )
        .route("/api/v1/posts/all", get(routes::api::get_all_posts))
        .route("/api/v1/posts/meta", get(routes::api::get_published_posts_meta))
        .route("/api/v1/posts/all-meta", get(routes::api::get_all_posts_meta))
        .route(
            "/api/v1/posts/uid/{post_uid}",
            get(routes::api::get_post_by_uid)
                .put(routes::api::update_post_by_uid)
                .delete(routes::api::delete_post_by_uid),
        )
        .route("/api/v1/posts/tag/{tag}", get(routes::api::get_published_posts_by_tag))
        .route("/api/v1/posts/tag/{tag}/all", get(routes::api::get_all_posts_by_tag))
        .route(
            "/api/v1/posts/tag/{tag}/meta",
            get(routes::api::get_published_posts_meta_by_tag),
        )
        .route(
            "/api/v1/posts/tag/{tag}/all-meta",
            get(routes::api::get_all_posts_meta_by_tag),
        )
        .route(
            "/api/v1/pages",
            get(routes::api::get_published_pages).post(routes::api::create_content),
        )
        .route("/api/v1/pages/all", get(routes::api::get_all_pages))
        .route("/api/v1/pages/meta", get(routes::api::get_published_pages_meta))
        .route("/api/v1/pages/all-meta", get(routes::api::get_all_pages_meta))
        .route(
            "/api/v1/pages/uid/{post_uid}",
            get(routes::api::get_page_by_uid)
                .put(routes::api::update_page_by_uid)
                .delete(routes::api::delete_page_by_uid),
        )
        .route("/api/v1/pages/tag/{tag}", get(routes::api::get_published_pages_by_tag))
        .route("/api/v1/pages/tag/{tag}/all", get(routes::api::get_all_pages_by_tag))
        .route(
            "/api/v1/pages/tag/{tag}/meta",
            get(routes::api::get_published_pages_meta_by_tag),
        )
        .route(
            "/api/v1/pages/tag/{tag}/all-meta",
            get(routes::api::get_all_pages_meta_by_tag),
        )
}
