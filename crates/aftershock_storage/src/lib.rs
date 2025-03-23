use axum::{Json, extract::Path};
use diesel::prelude::*;
use models::{Content, NewContent, UpdateContent};
use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod error;
mod models;
mod pool;
mod schema;
mod utils;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(get_connection_pool);

pub use private::*;

mod private {
    use super::*;

    pub async fn get_post(Path(post_id): Path<i32>) -> Result<Json<Content>> {
        use schema::contents::dsl::*;

        let conn = &mut POOL.clone().get()?;

        let ret = contents
            .find(post_id)
            .select(Content::as_select())
            .first(conn)
            .optional()?;
        match ret {
            Some(ret) => Ok(Json(ret)),
            None => Err(error::Error::NotFound("Post not found.".into())),
        }
    }
}

pub async fn get_published_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let results = contents
        .filter(published.eq(true))
        .select(Content::as_select())
        .load(conn)?;
    let mut ret: Vec<aftershock_bridge::Post> = results.into_iter().map(|x| x.into()).collect();
    ret.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

    Ok(Json(ret))
}

pub async fn get_all_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let results = contents.select(Content::as_select()).load(conn)?;
    let mut ret: Vec<aftershock_bridge::Post> = results.into_iter().map(|x| x.into()).collect();
    ret.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

    Ok(Json(ret))
}


pub async fn get_post_by_uid(
    Path(post_uid): Path<String>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;

    let ret = contents
        .filter(uid.eq(post_uid))
        .select(Content::as_select())
        .first(conn)
        .optional()?;
    match ret {
        Some(ret) => Ok(Json(ret.into())),
        None => Err(error::Error::NotFound("Post not found.".into())),
    }
}

pub async fn create_post(
    Json(post): Json<aftershock_bridge::NewPost>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let new_content: NewContent = (&post).into();

    let ret = diesel::insert_into(contents)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Json(ret.into()))
}

pub async fn update_post(
    Path(post_id): Path<i32>,
    Json(mut updated_set): Json<UpdateContent>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;
    let now = utils::now();
    updated_set.updated_at = Some(now);

    let ret = diesel::update(contents.find(post_id))
        .set(&updated_set)
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Json(ret.into()))
}

pub async fn delete_post(Path(post_id): Path<i32>) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents::dsl::*;

    let conn = &mut POOL.clone().get()?;

    let ret = diesel::delete(contents.find(post_id))
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Json(ret.into()))
}
