use axum::Json;
use diesel::prelude::*;
use models::{Content, NewContent};
use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod models;
pub mod pool;
pub mod schema;
pub mod error;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(|| get_connection_pool());

pub async fn get_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL
        .clone()
        .get()?;
    let results = contents
        .filter(published.eq(true))
        .select(Content::as_select())
        .load(conn)?;
    let ret = results.into_iter().map(|x| x.into()).collect();

    Ok(Json(ret))
}

pub async fn create_post(Json(post): Json<aftershock_bridge::NewPost>) -> Result<Json<Content>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL
        .clone()
        .get()?;
    let new_content: NewContent = (&post).into();

    let ret = diesel::insert_into(contents)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Json(ret))
}
