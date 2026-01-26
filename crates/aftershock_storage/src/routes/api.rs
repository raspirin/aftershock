use crate::POOL;
use crate::Result;
use crate::models::UpdateContent;
use crate::routes::worker::Worker;
use aftershock_bridge::{NewPost, Post, PostMeta};
use axum::{Json, extract::Path};

pub async fn get_published_posts() -> Result<Json<Vec<Post>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .post()
        .published_only()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_all_posts() -> Result<Json<Vec<Post>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .post()
        .all()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_published_posts_meta() -> Result<Json<Vec<PostMeta>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .post()
        .published_only()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_all_posts_meta() -> Result<Json<Vec<PostMeta>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .post()
        .all()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_post(Path(post_id): Path<String>) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let ret: Vec<Post> = Worker::builder()
        .post()
        .all()
        .by_id(post_id)
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Post not found".into())),
    }
}

pub async fn update_post(
    Path(post_id): Path<String>,
    Json(updated_set): Json<aftershock_bridge::UpdatePost>,
) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let update_content: UpdateContent = updated_set.into();

    let ret: Vec<Post> = Worker::builder()
        .post()
        .all()
        .by_id(post_id)
        .update(update_content)
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Post not found".into())),
    }
}

pub async fn delete_post(Path(post_id): Path<String>) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let ret: Vec<Post> = Worker::builder()
        .post()
        .all()
        .by_id(post_id)
        .delete()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Post not found".into())),
    }
}

pub async fn create_content(Json(post): Json<NewPost>) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let ret: Vec<Post> = Worker::builder()
        .create(post)
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound(
            "Failed to create content".into(),
        )),
    }
}

pub async fn get_content_by_uid(Path(post_uid): Path<String>) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let ret: Vec<Post> = Worker::builder()
        .published_only()
        .by_id(post_uid)
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Content not found".into())),
    }
}

pub async fn update_content_by_uid(
    Path(post_uid): Path<String>,
    Json(updated_set): Json<aftershock_bridge::UpdatePost>,
) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let update_content: UpdateContent = updated_set.into();

    let ret: Vec<Post> = Worker::builder()
        .published_only()
        .by_id(post_uid)
        .update(update_content)
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Content not found".into())),
    }
}

pub async fn delete_content_by_uid(Path(post_uid): Path<String>) -> Result<Json<Post>> {
    let conn = &mut POOL.clone().get()?;
    let ret: Vec<Post> = Worker::builder()
        .published_only()
        .by_id(post_uid)
        .delete()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    match ret.first() {
        Some(post) => Ok(Json(post.clone())),
        None => Err(crate::error::Error::NotFound("Content not found".into())),
    }
}

pub async fn get_published_pages() -> Result<Json<Vec<Post>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .page()
        .published_only()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_all_pages() -> Result<Json<Vec<Post>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .page()
        .all()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}

pub async fn get_all_pages_meta() -> Result<Json<Vec<PostMeta>>> {
    let conn = &mut POOL.clone().get()?;
    let ret = Worker::builder()
        .page()
        .all()
        .query()
        .build(conn)
        .ok_or_else(|| crate::error::Error::NotFound("Failed to build worker".into()))?
        .load()?;
    Ok(Json(ret))
}
