use axum::{Json, extract::Path};
use diesel::prelude::*;
use models::{Content, ContentTag, IntoPost, NewContent, NewTag, Tag, UpdateContent};
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
    use schema::contents;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;
    let mut posts = contents::table
        .select(Content::as_select())
        .filter(contents::published.eq(true))
        .load(conn)?;
    posts.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

    let tags: Vec<(ContentTag, Tag)> = ContentTag::belonging_to(&posts)
        .inner_join(tags::table)
        .select((ContentTag::as_select(), Tag::as_select()))
        .load(conn)?;

    let tags_per_content: Vec<_> = tags
        .grouped_by(&posts)
        .into_iter()
        .zip(posts)
        .map(|(tags, post)| (post, tags.into_iter().map(|(_, tag)| tag).collect()))
        .map(|x| x.into_post())
        .collect();

    Ok(Json(tags_per_content))
}

pub async fn get_published_posts_meta() -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
    let Json(posts) = get_published_posts().await?;
    Ok(Json(posts.into_iter().map(|post| post.into()).collect()))
}

pub async fn get_all_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
    use schema::contents;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;
    let mut posts = contents::table.select(Content::as_select()).load(conn)?;
    posts.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

    let tags: Vec<(ContentTag, Tag)> = ContentTag::belonging_to(&posts)
        .inner_join(tags::table)
        .select((ContentTag::as_select(), Tag::as_select()))
        .load(conn)?;

    let tags_per_content: Vec<_> = tags
        .grouped_by(&posts)
        .into_iter()
        .zip(posts)
        .map(|(tags, post)| (post, tags.into_iter().map(|(_, tag)| tag).collect()))
        .map(|x| x.into_post())
        .collect();

    Ok(Json(tags_per_content))
}

pub async fn get_post_by_uid(
    Path(post_uid): Path<String>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;

    let posts = contents::table
        .filter(contents::uid.eq(post_uid))
        .select(Content::as_select())
        .first(conn)
        .optional()?;

    let ret = posts
        .map(|post| {
            ContentTag::belonging_to(&post)
                .inner_join(tags::table)
                .select(Tag::as_select())
                .load(conn)
                .map(|tags| (post, tags).into_post())
                .ok()
        })
        .flatten();

    match ret {
        Some(ret) => Ok(Json(ret)),
        None => Err(error::Error::NotFound("Post not found.".into())),
    }
}

pub async fn create_post(
    Json(post): Json<aftershock_bridge::NewPost>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents;
    use schema::contents_tags;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;
    let new_content: NewContent = (&post).into();

    let content = diesel::insert_into(contents::table)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)?;

    let new_tags: Vec<NewTag<'_>> = post.tags.iter().map(|x| x.into()).collect();

    let tags = conn.transaction(|conn| {
        for new_tag in new_tags {
            let _ = diesel::insert_into(tags::table)
                .values(&new_tag)
                .on_conflict_do_nothing()
                .execute(conn)?;
        }
        tags::table
            .filter(tags::tag.eq_any(&post.tags))
            .select(Tag::as_returning())
            .get_results(conn)
    })?;

    let ct: Vec<ContentTag> = tags.iter().map(|tag| (content.id, tag.id).into()).collect();
    diesel::insert_into(contents_tags::table)
        .values(&ct)
        .execute(conn)?;

    Ok(Json((content, tags).into_post()))
}

pub async fn update_post(
    Path(post_id): Path<i32>,
    Json(mut updated_set): Json<UpdateContent>,
) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;
    let now = utils::now();
    updated_set.updated_at = Some(now);

    let content = diesel::update(contents::table.find(post_id))
        .set(&updated_set)
        .returning(Content::as_returning())
        .get_result(conn)?;

    let tags = ContentTag::belonging_to(&content)
        .inner_join(tags::table)
        .select(Tag::as_select())
        .load(conn)?;

    Ok(Json((content, tags).into_post()))
}

pub async fn delete_post(Path(post_id): Path<i32>) -> Result<Json<aftershock_bridge::Post>> {
    use schema::contents;
    use schema::contents_tags;
    use schema::tags;

    let conn = &mut POOL.clone().get()?;

    let content = diesel::delete(contents::table.find(post_id))
        .returning(Content::as_returning())
        .get_result(conn)?;

    let tags = ContentTag::belonging_to(&content)
        .inner_join(tags::table)
        .select(Tag::as_returning())
        .get_results(conn)?;
    diesel::delete(contents_tags::table.filter(contents_tags::content_id.eq(content.id)))
        .execute(conn)?;

    Ok(Json((content, tags).into_post()))
}
