use axum::Json;
use diesel::prelude::*;
use models::{Content, ContentKind, NewContent};
use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod models;
pub mod pool;
pub mod schema;

static POOL: LazyLock<DbPool> = LazyLock::new(|| get_connection_pool());

pub async fn get_posts() -> Json<Vec<aftershock_bridge::Post>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL
        .clone()
        .get()
        .expect("Fail to get a connection from pool.");
    let results = contents
        .filter(published.eq(true))
        .select(Content::as_select())
        .load(conn)
        .unwrap();
    let ret = results.into_iter().map(|x| x.into()).collect();

    Json(ret)
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Content {
    use crate::schema::contents;

    let new_content = NewContent::new(ContentKind::Post, title, body);

    // FIXME: better error handling ie. return error to caller and let frontend know
    diesel::insert_into(contents::table)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)
        .expect("Error saving new post.")
}
