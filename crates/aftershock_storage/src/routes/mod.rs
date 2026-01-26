pub mod api;
pub mod worker;

pub use api::{
    create_content, delete_page_by_uid, delete_post_by_uid, get_all_pages, get_all_pages_meta,
    get_all_posts, get_all_posts_meta, get_page_by_uid, get_post_by_uid, get_published_pages,
    get_published_pages_meta, get_published_posts, get_published_posts_meta, update_page_by_uid,
    update_post_by_uid,
};

// pub mod private {
//     use crate::schema;

//     use super::*;

//     pub async fn get_post(Path(post_id): Path<i32>) -> Result<Json<Content>> {
//         use schema::contents::dsl::*;

//         let conn = &mut POOL.clone().get()?;

//         let ret = contents
//             .find(post_id)
//             .select(Content::as_select())
//             .first(conn)
//             .optional()?;
//         match ret {
//             Some(ret) => Ok(Json(ret)),
//             None => Err(crate::error::Error::NotFound("Post not found.".into())),
//         }
//     }
// }

// fn get_tags_from_content(content: &Content) -> Result<Vec<Tag>> {
//     let conn = &mut POOL.clone().get()?;
//     let tags = ContentTag::belonging_to(content)
//         .inner_join(tags::table)
//         .select(Tag::as_select())
//         .load(conn)?;
//     Ok(tags)
// }

// async fn get_published_content(kind: &str) -> Result<Json<Vec<aftershock_bridge::Post>>> {
//     use schema::contents;
//     use schema::tags;

//     let conn = &mut POOL.clone().get()?;
//     let mut posts = contents::table
//         .select(Content::as_select())
//         .filter(contents::kind.eq(kind))
//         .filter(contents::published.eq(true))
//         .load(conn)?;
//     posts.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

//     let tags: Vec<(ContentTag, Tag)> = ContentTag::belonging_to(&posts)
//         .inner_join(tags::table)
//         .select((ContentTag::as_select(), Tag::as_select()))
//         .load(conn)?;

//     let tags_per_content: Vec<_> = tags
//         .grouped_by(&posts)
//         .into_iter()
//         .zip(posts)
//         .map(|(tags, post)| (post, tags.into_iter().map(|(_, tag)| tag).collect()))
//         .map(|x| x.into_post())
//         .collect();

//     Ok(Json(tags_per_content))
// }

// async fn get_published_contents_meta(kind: &str) -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//     let Json(posts) = get_published_content(kind).await?;
//     Ok(Json(posts.into_iter().map(|post| post.into()).collect()))
// }

// async fn get_all_contents(kind: &str) -> Result<Json<Vec<aftershock_bridge::Post>>> {
//     use schema::contents;
//     use schema::tags;

//     let conn = &mut POOL.clone().get()?;
//     let mut posts = contents::table
//         .filter(contents::kind.eq(kind))
//         .select(Content::as_select())
//         .load(conn)?;
//     posts.sort_by(|lhs, rhs| rhs.created_at.cmp(&lhs.created_at));

//     let tags: Vec<(ContentTag, Tag)> = ContentTag::belonging_to(&posts)
//         .inner_join(tags::table)
//         .select((ContentTag::as_select(), Tag::as_select()))
//         .load(conn)?;

//     let tags_per_content: Vec<_> = tags
//         .grouped_by(&posts)
//         .into_iter()
//         .zip(posts)
//         .map(|(tags, post)| (post, tags.into_iter().map(|(_, tag)| tag).collect()))
//         .map(|x| x.into_post())
//         .collect();

//     Ok(Json(tags_per_content))
// }

// async fn get_all_contents_meta(kind: &str) -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//     let Json(contents) = get_all_contents(kind).await?;
//     Ok(Json(
//         contents.into_iter().map(|content| content.into()).collect(),
//     ))
// }

// pub async fn create_content(
//     Json(post): Json<aftershock_bridge::NewPost>,
// ) -> Result<Json<aftershock_bridge::Post>> {
//     use schema::contents;
//     use schema::contents_tags;
//     use schema::tags;

//     let conn = &mut POOL.clone().get()?;
//     let new_content: NewContent = (&post).into();

//     let content = diesel::insert_into(contents::table)
//         .values(&new_content)
//         .returning(Content::as_returning())
//         .get_result(conn)?;

//     let new_tags: Vec<NewTag<'_>> = post.tags.iter().map(|x| x.into()).collect();

//     let tags = conn.transaction(|conn| {
//         for new_tag in new_tags {
//             let _ = diesel::insert_into(tags::table)
//                 .values(&new_tag)
//                 .on_conflict_do_nothing()
//                 .execute(conn)?;
//         }
//         tags::table
//             .filter(tags::tag.eq_any(&post.tags))
//             .select(Tag::as_returning())
//             .get_results(conn)
//     })?;

//     let ct: Vec<ContentTag> = tags.iter().map(|tag| (content.id, tag.id).into()).collect();
//     diesel::insert_into(contents_tags::table)
//         .values(&ct)
//         .execute(conn)?;

//     Ok(Json((content, tags).into_post()))
// }

// pub async fn delete_content_by_uid(
//     Path(content_id): Path<String>,
// ) -> Result<Json<aftershock_bridge::Post>> {
//     let conn = &mut POOL.clone().get()?;

//     let content = diesel::delete(contents::table.filter(contents::uid.eq(content_id)))
//         .returning(Content::as_returning())
//         .get_result(conn)?;

//     let tags = get_tags_from_content(&content)?;

//     diesel::delete(contents_tags::table.filter(contents_tags::content_id.eq(content.id)))
//         .execute(conn)?;

//     Ok(Json((content, tags).into_post()))
// }

// pub async fn update_content_by_uid(
//     Path(post_uid): Path<String>,
//     Json(mut updated_set): Json<UpdateContent>,
// ) -> Result<Json<aftershock_bridge::Post>> {
//     let conn = &mut POOL.clone().get()?;
//     let now = crate::utils::now();
//     updated_set.updated_at = Some(now);
//     if updated_set.published.is_some_and(|x| x) {
//         updated_set.created_at = Some(now);
//     }

//     let content = diesel::update(contents::table.filter(contents::uid.eq(post_uid)))
//         .set(updated_set)
//         .returning(Content::as_returning())
//         .get_result(conn)?;

//     let tags = get_tags_from_content(&content)?;

//     Ok(Json((content, tags).into_post()))
// }

// pub async fn get_content_by_uid(
//     Path(post_uid): Path<String>,
// ) -> Result<Json<aftershock_bridge::Post>> {
//     use schema::contents;

//     let conn = &mut POOL.clone().get()?;

//     let posts = contents::table
//         .filter(contents::uid.eq(post_uid))
//         .filter(contents::published.eq(true))
//         .select(Content::as_select())
//         .first(conn)
//         .optional()?;

//     let ret = posts.and_then(|post| {
//         let tags = get_tags_from_content(&post);
//         tags.map(|tags| (post, tags).into_post()).ok()
//     });

//     match ret {
//         Some(ret) => Ok(Json(ret)),
//         None => Err(crate::error::Error::NotFound("Post not found.".into())),
//     }
// }

// pub mod post {
//     use super::*;

//     pub async fn get_published_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
//         get_published_content("post").await
//     }

//     pub async fn get_published_posts_meta() -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//         get_published_contents_meta("post").await
//     }

//     pub async fn get_all_posts() -> Result<Json<Vec<aftershock_bridge::Post>>> {
//         get_all_contents("post").await
//     }

//     pub async fn get_all_posts_meta() -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//         get_all_contents_meta("post").await
//     }

//     pub async fn update_post(
//         Path(post_id): Path<i32>,
//         Json(updated_set): Json<aftershock_bridge::UpdatePost>,
//     ) -> Result<Json<aftershock_bridge::Post>> {
//         use schema::contents;

//         let conn = &mut POOL.clone().get()?;
//         let now = crate::utils::now();
//         let mut updated_set: UpdateContent = updated_set.into();
//         updated_set.updated_at = Some(now);

//         let content = diesel::update(contents::table.find(post_id))
//             .set(&updated_set)
//             .returning(Content::as_returning())
//             .get_result(conn)?;

//         let tags = get_tags_from_content(&content)?;

//         Ok(Json((content, tags).into_post()))
//     }

//     pub async fn delete_post(Path(post_id): Path<i32>) -> Result<Json<aftershock_bridge::Post>> {
//         use schema::contents;
//         use schema::contents_tags;

//         let conn = &mut POOL.clone().get()?;

//         let content = diesel::delete(contents::table.find(post_id))
//             .returning(Content::as_returning())
//             .get_result(conn)?;

//         let tags = get_tags_from_content(&content)?;

//         diesel::delete(contents_tags::table.filter(contents_tags::content_id.eq(content.id)))
//             .execute(conn)?;

//         Ok(Json((content, tags).into_post()))
//     }
// }

// pub mod page {
//     use super::*;

//     pub async fn get_all_pages() -> Result<Json<Vec<aftershock_bridge::Post>>> {
//         get_all_contents("page").await
//     }

//     pub async fn get_published_pages() -> Result<Json<Vec<aftershock_bridge::Post>>> {
//         get_published_content("page").await
//     }

//     pub async fn get_all_pages_meta() -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//         get_all_contents_meta("page").await
//     }

//     pub async fn get_published_pages_meta() -> Result<Json<Vec<aftershock_bridge::PostMeta>>> {
//         get_published_contents_meta("page").await
//     }
// }
