use axum::{Json, extract::Path};
use diesel::prelude::*;
use models::{Content, NewContent};
use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod error;
pub mod models;
pub mod pool;
pub mod schema;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(get_connection_pool);

pub async fn get_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let results = contents
        .filter(published.eq(true))
        .select(Content::as_select())
        .load(conn)?;
    let ret = results.into_iter().map(|x| x.into()).collect();

    Ok(Json(ret))
}

pub async fn get_post(Path(post_id): Path<i32>) -> Result<Json<Content>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;

    let mut ret = contents
        .filter(id.eq(post_id))
        .limit(1)
        .select(Content::as_select())
        .load(conn)?;
    if ret.is_empty() {
        Err(error::Error::NotFound("Post not found".into()))
    } else {
        Ok(Json(ret.pop().unwrap()))
    }
}

pub async fn create_post(Json(post): Json<aftershock_bridge::NewPost>) -> Result<Json<Content>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let new_content: NewContent = (&post).into();

    let ret = diesel::insert_into(contents)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Json(ret))
}
